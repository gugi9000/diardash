use chrono::Local;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

diesel::table! {
    atera_metrics (id) {
        id -> Integer,
        recorded_at -> Timestamp,
        active_alerts -> Integer,
        open_tickets -> Integer,
        pending_patches -> Integer,
        device_count -> Integer,
    }
}

#[derive(Insertable)]
#[diesel(table_name = atera_metrics)]
struct NewAteraMetric {
    recorded_at: chrono::NaiveDateTime,
    active_alerts: i32,
    open_tickets: i32,
    pending_patches: i32,
    device_count: i32,
}

const ALERTS_API: &str = "https://app.atera.com/api/v3/alerts?alertStatus=Open";
const TICKETS_API: &str = "https://app.atera.com/api/v3/tickets?ticketStatus=Open";
const DEVICES_API: &str = "https://app.atera.com/api/v3/agents";
const DEVICES_PAGE_SIZE: usize = 500;
const AVAILABLE_PATCHES_API: &str = "https://app.atera.com/api/v3/agents/{guid}/available-patches";

#[derive(Clone, Copy)]
struct RunOptions {
    dry_run: bool,
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let options = match parse_options() {
        Ok(options) => options,
        Err(error) => {
            eprintln!("{error}");
            print_usage();
            std::process::exit(2);
        }
    };

    if let Err(error) = run(options).await {
        eprintln!("Atera updater failed: {error}");
        std::process::exit(1);
    }
}

async fn run(options: RunOptions) -> Result<(), String> {
    dotenv().ok();

    let raw_database_url = std::env::var("DATABASE_URL")
        .map_err(|_| String::from("Missing DATABASE_URL in .env"))?;
    let database_url = resolve_database_url(&raw_database_url)?;
    let api_key = std::env::var("ATERA_API_KEY")
        .map_err(|_| String::from("Missing ATERA_API_KEY in .env"))?;

    if options.verbose {
        println!("Raw DATABASE_URL={raw_database_url}");
        println!("Using DATABASE_URL={database_url}");
        println!("Dry-run mode: {}", options.dry_run);
    }

    let mut connection = SqliteConnection::establish(&database_url)
        .map_err(|error| format!("Failed to open SQLite database: {error}"))?;

    ensure_schema(&mut connection)?;

    let client = reqwest::Client::builder()
        .default_headers(build_headers(&api_key)?)
        .build()
        .map_err(|error| format!("Failed to build HTTP client: {error}"))?;

    let active_alerts = fetch_total_item_count(&client, ALERTS_API, "alerts", options.verbose).await?;
    let open_tickets = fetch_total_item_count(&client, TICKETS_API, "tickets", options.verbose).await?;

    let device_guids = fetch_device_guids(&client, DEVICES_API, options.verbose).await?;
    let device_count = i32::try_from(device_guids.len())
        .map_err(|_| String::from("Device count exceeds i32 range"))?;

    let total_patches = fetch_total_patches(&client, &device_guids, options.verbose).await?;
    let pending_patches = if device_count > 0 {
        (total_patches as f64 / device_count as f64).round() as i32
    } else {
        0
    };

    if options.verbose {
        println!(
            "Computed metrics: alerts={active_alerts}, tickets={open_tickets}, devices={device_count}, total_patches={total_patches}, pending_patches={pending_patches}"
        );
    }

    let new_row = NewAteraMetric {
        recorded_at: Local::now().naive_local(),
        active_alerts,
        open_tickets,
        pending_patches,
        device_count,
    };

    if options.dry_run {
        println!(
            "Atera updater dry-run: would write row alerts={active_alerts}, tickets={open_tickets}, devices={device_count}, pending_patches={pending_patches}"
        );
    } else {
        diesel::insert_into(atera_metrics::table)
            .values(&new_row)
            .execute(&mut connection)
            .map_err(|error| format!("Failed to insert Atera metric row: {error}"))?;

        println!(
            "Atera updater wrote row: alerts={active_alerts}, tickets={open_tickets}, devices={device_count}, pending_patches={pending_patches}"
        );
    }

    Ok(())
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

    ensure_parent_directory(&absolute_path)?;

    Ok(absolute_path.to_string_lossy().into_owned())
}

fn ensure_parent_directory(path: &Path) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("Failed to create database directory: {error}"))?;
        }
    }

    Ok(())
}

fn parse_options() -> Result<RunOptions, String> {
    let mut dry_run = false;
    let mut verbose = false;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "--dry-run" => dry_run = true,
            "--verbose" => verbose = true,
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            _ => return Err(format!("Unknown argument: {arg}")),
        }
    }

    Ok(RunOptions { dry_run, verbose })
}

fn print_usage() {
    println!("Usage: cargo run --bin atera_updater -- [--dry-run] [--verbose]");
    println!("  --dry-run  Fetch and compute metrics without writing to database");
    println!("  --verbose  Print detailed debug output during API fetch and processing");
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

async fn fetch_total_item_count(
    client: &reqwest::Client,
    url: &str,
    label: &str,
    verbose: bool,
) -> Result<i32, String> {
    let payload = get_json(client, url, verbose).await?;

    let total = payload
        .get("totalItemCount")
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("Missing totalItemCount in {label} API response"))?;

    if verbose {
        println!("{label} totalItemCount={total}");
    }

    i32::try_from(total).map_err(|_| format!("{label} totalItemCount exceeds i32 range"))
}

async fn fetch_device_guids(client: &reqwest::Client, url: &str, verbose: bool) -> Result<Vec<String>, String> {
    let mut page: usize = 1;
    let mut guid_set: BTreeSet<String> = BTreeSet::new();

    loop {
        let paged_url = build_paginated_url(url, page, DEVICES_PAGE_SIZE);
        let payload = get_json(client, &paged_url, verbose).await?;
        let items = extract_items(&payload)
            .ok_or_else(|| String::from("Devices API did not return an items array (expected items/Items or top-level array)"))?;

        let page_guids = items
            .iter()
            .filter_map(extract_device_guid_flexible)
            .collect::<Vec<String>>();

        if page_guids.is_empty() {
            if items.is_empty() {
                if verbose {
                    println!("Devices page {page} returned 0 items, stopping pagination.");
                }
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

        if verbose {
            println!("Loaded {} items from devices page {page}", items.len());
        }

        if items.len() < DEVICES_PAGE_SIZE {
            break;
        }

        page += 1;
    }

    if guid_set.is_empty() {
        return Err(String::from("No device GUID values found across paginated device responses"));
    }

    let guids = guid_set.into_iter().collect::<Vec<String>>();

    if verbose {
        println!("Loaded {} unique devices", guids.len());
    }

    Ok(guids)
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
    verbose: bool,
) -> Result<i32, String> {
    let mut total: i64 = 0;

    for guid in device_guids {
        let url = AVAILABLE_PATCHES_API.replace("{guid}", guid);
        let payload = get_json(client, &url, verbose).await?;

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

        if verbose {
            println!("Device {guid} patch_count={patch_count}");
        }

        total += patch_count;
    }

    if verbose {
        println!("Total patches across devices={total}");
    }

    i32::try_from(total).map_err(|_| String::from("Total patch count exceeds i32 range"))
}

async fn get_json(client: &reqwest::Client, url: &str, verbose: bool) -> Result<Value, String> {
    if verbose {
        println!("GET {url}");
    }

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|error| format!("Request error for {url}: {error}"))?;

    if verbose {
        println!("{url} -> HTTP {}", response.status());
    }

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
