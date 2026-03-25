use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use chrono::{Duration, Local, NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use rand::Rng;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{json, Value};
use tokio::runtime::Builder;

use crate::models::{AteraMetric, NewAteraMetric};
use crate::schema::atera_metrics;

const SEED_DAYS: i64 = 12;
const ENTRIES_PER_DAY: i64 = 4;
const STALE_AFTER_HOURS: i64 = 4;
const ALERTS_API: &str = "https://app.atera.com/api/v3/alerts?alertStatus=Open";
const TICKETS_API: &str = "https://app.atera.com/api/v3/tickets?ticketStatus=Open";
const DEVICES_API: &str = "https://app.atera.com/api/v3/agents";
const DEVICES_PAGE_SIZE: usize = 500;
const AVAILABLE_PATCHES_API: &str = "https://app.atera.com/api/v3/agents/{guid}/available-patches";

static ATERA_REFRESH_IN_PROGRESS: AtomicBool = AtomicBool::new(false);

pub fn initialize_database() -> Result<(), String> {
    dotenv().ok();

    let database_url = database_url()?;
    let mut connection = establish_connection(&database_url)?;
    ensure_schema(&mut connection)?;
    let seeded = seed_if_empty(&mut connection)?;

    eprintln!("Atera DB startup: using DATABASE_URL={database_url}");
    if seeded {
        eprintln!("Atera DB startup: inserted initial seed data");
    } else {
        eprintln!("Atera DB startup: seed skipped, existing data found");
    }

    Ok(())
}

pub fn fetch_payload() -> Result<Value, String> {
    dotenv().ok();

    let database_url = database_url()?;
    let mut connection = establish_connection(&database_url)?;
    ensure_schema(&mut connection)?;
    seed_if_empty(&mut connection)?;

    let latest = latest_metric(&mut connection)?;
    let refresh_in_progress = maybe_refresh_metrics_in_background(&latest, database_url.clone());
    let history = history_maxima(&mut connection)?;

    Ok(json!({
        "active_alerts": latest.active_alerts,
        "open_tickets": latest.open_tickets,
        "patching": {
            "pending_patches": latest.pending_patches,
            "device_count": latest.device_count
        },
        "history": history,
        "_meta": {
            "refresh_in_progress": refresh_in_progress,
            "stale_after_hours": STALE_AFTER_HOURS
        }
    }))
}

fn maybe_refresh_metrics_in_background(latest: &AteraMetric, database_url: String) -> bool {
    let age = Local::now().naive_local() - latest.recorded_at;
    if age < Duration::hours(STALE_AFTER_HOURS) {
        return ATERA_REFRESH_IN_PROGRESS.load(Ordering::Acquire);
    }

    if ATERA_REFRESH_IN_PROGRESS.swap(true, Ordering::AcqRel) {
        return true;
    }

    std::thread::spawn(move || {
        let result = refresh_metrics_now(&database_url);
        if let Err(error) = result {
            eprintln!("Atera background refresh failed: {error}");
        }
        ATERA_REFRESH_IN_PROGRESS.store(false, Ordering::Release);
    });

    true
}

fn refresh_metrics_now(database_url: &str) -> Result<(), String> {
    let api_key = std::env::var("ATERA_API_KEY")
        .map_err(|_| String::from("Missing ATERA_API_KEY in .env"))?;

    let runtime = Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| format!("Failed to build async runtime for Atera refresh: {error}"))?;

    let metrics = runtime.block_on(fetch_live_metrics(&api_key))?;

    let mut connection = establish_connection(database_url)?;
    ensure_schema(&mut connection)?;

    let new_row = NewAteraMetric {
        recorded_at: Local::now().naive_local(),
        active_alerts: metrics.active_alerts,
        open_tickets: metrics.open_tickets,
        pending_patches: metrics.pending_patches,
        device_count: metrics.device_count,
    };

    diesel::insert_into(atera_metrics::table)
        .values(&new_row)
        .execute(&mut connection)
        .map_err(|error| format!("Failed to insert Atera metric row: {error}"))?;

    eprintln!(
        "Atera background refresh inserted row: alerts={}, tickets={}, devices={}, pending_patches={}",
        metrics.active_alerts,
        metrics.open_tickets,
        metrics.device_count,
        metrics.pending_patches
    );

    Ok(())
}

struct LiveAteraMetrics {
    active_alerts: i32,
    open_tickets: i32,
    pending_patches: i32,
    device_count: i32,
}

async fn fetch_live_metrics(api_key: &str) -> Result<LiveAteraMetrics, String> {
    let client = reqwest::Client::builder()
        .default_headers(build_headers(api_key)?)
        .build()
        .map_err(|error| format!("Failed to build HTTP client: {error}"))?;

    let active_alerts = fetch_total_item_count(&client, ALERTS_API, "alerts").await?;
    let open_tickets = fetch_total_item_count(&client, TICKETS_API, "tickets").await?;

    let device_guids = fetch_device_guids(&client, DEVICES_API).await?;
    let device_count = i32::try_from(device_guids.len())
        .map_err(|_| String::from("Device count exceeds i32 range"))?;

    let total_patches = fetch_total_patches(&client, &device_guids).await?;
    let pending_patches = if device_count > 0 {
        (total_patches as f64 / device_count as f64).round() as i32
    } else {
        0
    };

    Ok(LiveAteraMetrics {
        active_alerts,
        open_tickets,
        pending_patches,
        device_count,
    })
}

fn build_headers(api_key: &str) -> Result<HeaderMap, String> {
    let mut headers = HeaderMap::new();
    headers.insert(
        "X-API-KEY",
        HeaderValue::from_str(api_key)
            .map_err(|error| format!("Invalid ATERA_API_KEY header value: {error}"))?,
    );

    Ok(headers)
}

async fn fetch_total_item_count(
    client: &reqwest::Client,
    url: &str,
    label: &str,
) -> Result<i32, String> {
    let payload = get_json(client, url).await?;

    let total = payload
        .get("totalItemCount")
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("Missing totalItemCount in {label} API response"))?;

    i32::try_from(total).map_err(|_| format!("{label} totalItemCount exceeds i32 range"))
}

async fn fetch_device_guids(client: &reqwest::Client, url: &str) -> Result<Vec<String>, String> {
    let mut page: usize = 1;
    let mut guid_set: BTreeSet<String> = BTreeSet::new();

    loop {
        let paged_url = build_paginated_url(url, page, DEVICES_PAGE_SIZE);
        let payload = get_json(client, &paged_url).await?;
        let items = extract_items(&payload)
            .ok_or_else(|| String::from("Devices API did not return an items array (expected items/Items or top-level array)"))?;

        let page_guids = items
            .iter()
            .filter_map(extract_device_guid_flexible)
            .collect::<Vec<String>>();

        if page_guids.is_empty() {
            if items.is_empty() {
                break;
            }

            let first_type = items
                .first()
                .map(value_type_name)
                .unwrap_or("none");
            let key_hint = items
                .first()
                .and_then(Value::as_object)
                .map(|obj| obj.keys().cloned().collect::<Vec<String>>().join(", "))
                .unwrap_or_else(|| String::from("<no object keys available>"));
            let first_preview = items
                .first()
                .map(preview_value)
                .unwrap_or_else(|| String::from("<none>"));
            return Err(format!(
                "No device GUID values found in devices API response on page {page}. Looked for DeviceGuid/deviceGuid/deviceGUID/guid. First item type: {first_type}. First item keys: {key_hint}. First item preview: {first_preview}"
            ));
        }

        for guid in page_guids {
            guid_set.insert(guid);
        }

        if items.len() < DEVICES_PAGE_SIZE {
            break;
        }

        page += 1;
    }

    if guid_set.is_empty() {
        return Err(String::from("No device GUID values found across paginated device responses"));
    }

    Ok(guid_set.into_iter().collect::<Vec<String>>())
}

fn build_paginated_url(base_url: &str, page: usize, items_in_page: usize) -> String {
    let separator = if base_url.contains('?') { '&' } else { '?' };
    format!("{base_url}{separator}page={page}&itemsInPage={items_in_page}")
}

fn extract_items(payload: &Value) -> Option<&Vec<Value>> {
    payload
        .get("items")
        .and_then(Value::as_array)
        .or_else(|| payload.get("Items").and_then(Value::as_array))
        .or_else(|| payload.as_array())
}

fn extract_device_guid(item: &Value) -> Option<&str> {
    item.get("DeviceGuid")
        .and_then(Value::as_str)
        .or_else(|| item.get("deviceGuid").and_then(Value::as_str))
        .or_else(|| item.get("deviceGUID").and_then(Value::as_str))
        .or_else(|| item.get("guid").and_then(Value::as_str))
        .map(str::trim)
        .filter(|guid| !guid.is_empty())
}

fn extract_device_guid_flexible(item: &Value) -> Option<String> {
    if let Some(guid) = extract_device_guid(item) {
        return Some(guid.to_string());
    }

    let as_text = item.as_str()?;
    let parsed = serde_json::from_str::<Value>(as_text).ok()?;
    extract_device_guid(&parsed).map(str::to_string)
}

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn preview_value(value: &Value) -> String {
    let rendered = value.to_string();
    if rendered.len() <= 160 {
        rendered
    } else {
        format!("{}...", &rendered[..160])
    }
}

async fn fetch_total_patches(
    client: &reqwest::Client,
    device_guids: &[String],
) -> Result<i32, String> {
    let mut total: i64 = 0;

    for guid in device_guids {
        let url = AVAILABLE_PATCHES_API.replace("{guid}", guid);
        let payload = get_json(client, &url).await?;

        let updates_value = payload
            .get("availableUpdates")
            .or_else(|| payload.get("AvailableUpdates"));

        let patch_count = match updates_value {
            Some(Value::Array(updates)) => updates.len() as i64,
            Some(Value::Null) | None => 0,
            Some(other) => {
                let key_hint = payload
                    .as_object()
                    .map(|obj| obj.keys().cloned().collect::<Vec<String>>().join(", "))
                    .unwrap_or_else(|| String::from("<no object keys available>"));
                return Err(format!(
                    "Installed patches availableUpdates has unexpected type '{}' for device {guid}. Top-level keys: {key_hint}. Value preview: {}",
                    value_type_name(other),
                    preview_value(other)
                ));
            }
        };

        total += patch_count;
    }

    i32::try_from(total).map_err(|_| String::from("Total patch count exceeds i32 range"))
}

async fn get_json(client: &reqwest::Client, url: &str) -> Result<Value, String> {
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| format!("Request error for {url}: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| String::from("<no response body>"));
        return Err(format!("API returned {status} for {url}: {body}"));
    }

    response
        .json::<Value>()
        .await
        .map_err(|error| format!("Invalid JSON from {url}: {error}"))
}

fn database_url() -> Result<String, String> {
    let raw = std::env::var("DATABASE_URL").map_err(|_| String::from("Missing DATABASE_URL in .env"))?;
    resolve_database_url(&raw)
}

fn resolve_database_url(raw_database_url: &str) -> Result<String, String> {
    if raw_database_url == ":memory:" {
        return Ok(String::from(raw_database_url));
    }

    let raw_path = PathBuf::from(raw_database_url);
    let absolute_path = if raw_path.is_absolute() {
        raw_path
    } else {
        let cwd = env::current_dir()
            .map_err(|error| format!("Failed to resolve working directory: {error}"))?;
        cwd.join(raw_path)
    };

    Ok(absolute_path.to_string_lossy().into_owned())
}

fn establish_connection(database_url: &str) -> Result<SqliteConnection, String> {
    if database_url != ":memory:" {
        ensure_parent_directory(database_url)?;
    }

    SqliteConnection::establish(database_url)
        .map_err(|error| format!("Failed to open SQLite database: {error}"))
}

fn ensure_parent_directory(database_url: &str) -> Result<(), String> {
    let path = Path::new(database_url);
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Failed to create database directory: {error}"))?;
        }
    }

    Ok(())
}

fn ensure_schema(connection: &mut SqliteConnection) -> Result<(), String> {
    sql_query(
        "CREATE TABLE IF NOT EXISTS atera_metrics (\
            id INTEGER PRIMARY KEY AUTOINCREMENT, \
            recorded_at TIMESTAMP NOT NULL, \
            active_alerts INTEGER NOT NULL, \
            open_tickets INTEGER NOT NULL, \
            pending_patches INTEGER NOT NULL, \
            device_count INTEGER NOT NULL\
        )",
    )
    .execute(connection)
    .map_err(|error| format!("Failed to ensure atera_metrics schema: {error}"))?;

    Ok(())
}

fn seed_if_empty(connection: &mut SqliteConnection) -> Result<bool, String> {
    use crate::schema::atera_metrics::dsl::atera_metrics as atera_metrics_table;

    let row_count = atera_metrics_table
        .count()
        .get_result::<i64>(connection)
        .map_err(|error| format!("Failed to count atera_metrics rows: {error}"))?;

    if row_count > 0 {
        return Ok(false);
    }

    let mut rng = rand::thread_rng();
    let today = Local::now().date_naive();
    let mut rows = Vec::new();

    for days_back in 0..SEED_DAYS {
        let day = today - Duration::days(days_back);
        let trend = (SEED_DAYS - days_back) as i32;

        for sample in 0..ENTRIES_PER_DAY {
            let hour = 2 + (sample as u32 * 5);
            let minute = rng.gen_range(0..60);
            let second = rng.gen_range(0..60);
            let recorded_at = day.and_time(
                NaiveTime::from_hms_opt(hour, minute, second)
                    .ok_or_else(|| String::from("Failed to construct seed timestamp"))?,
            );

            rows.push(NewAteraMetric {
                recorded_at,
                active_alerts: rng.gen_range(6..18) + (trend % 5),
                open_tickets: rng.gen_range(2..10) + (trend % 4),
                pending_patches: rng.gen_range(24..70) + (trend % 12),
                device_count: rng.gen_range(105..145),
            });
        }
    }

    diesel::insert_into(atera_metrics::table)
        .values(&rows)
        .execute(connection)
        .map_err(|error| format!("Failed to seed atera_metrics: {error}"))?;

    Ok(true)
}

fn latest_metric(connection: &mut SqliteConnection) -> Result<AteraMetric, String> {
    use crate::schema::atera_metrics::dsl::{atera_metrics as atera_metrics_table, recorded_at};

    atera_metrics_table
        .order(recorded_at.desc())
        .select(AteraMetric::as_select())
        .first::<AteraMetric>(connection)
        .map_err(|error| format!("Failed to load latest Atera metric: {error}"))
}

fn history_maxima(connection: &mut SqliteConnection) -> Result<Vec<Value>, String> {
    use crate::schema::atera_metrics::dsl::{atera_metrics as atera_metrics_table, recorded_at};

    let today = Local::now().date_naive();
    let start_date = today - Duration::days(6);
    let start_of_window = start_date.and_hms_opt(0, 0, 0)
        .ok_or_else(|| String::from("Failed to construct history window start"))?;

    let rows = atera_metrics_table
        .filter(recorded_at.ge(start_of_window))
        .order(recorded_at.asc())
        .select(AteraMetric::as_select())
        .load::<AteraMetric>(connection)
        .map_err(|error| format!("Failed to load Atera history: {error}"))?;

    let mut by_day: BTreeMap<NaiveDate, DailyMaxima> = BTreeMap::new();

    for row in rows {
        let entry = by_day.entry(row.recorded_at.date()).or_default();
        entry.active_alerts = entry.active_alerts.max(row.active_alerts);
        entry.open_tickets = entry.open_tickets.max(row.open_tickets);
        entry.pending_patches = entry.pending_patches.max(row.pending_patches);
        entry.device_count = entry.device_count.max(row.device_count);
    }

    let mut history = Vec::new();
    for day_offset in 0..7 {
        let day = start_date + Duration::days(day_offset);
        let maxima = by_day.get(&day).cloned().unwrap_or_default();
        history.push(json!({
            "day": day.format("%a").to_string(),
            "alerts": maxima.active_alerts,
            "tickets": maxima.open_tickets,
            "pending_patches": maxima.pending_patches,
            "device_count": maxima.device_count,
        }));
    }

    Ok(history)
}

#[derive(Clone, Default)]
struct DailyMaxima {
    active_alerts: i32,
    open_tickets: i32,
    pending_patches: i32,
    device_count: i32,
}
