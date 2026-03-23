use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use chrono::{Duration, Local, NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use rand::Rng;
use serde_json::{json, Value};

use crate::models::{AteraMetric, NewAteraMetric};
use crate::schema::atera_metrics;

const SEED_DAYS: i64 = 12;
const ENTRIES_PER_DAY: i64 = 4;

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
    let history = history_maxima(&mut connection)?;

    Ok(json!({
        "active_alerts": latest.active_alerts,
        "open_tickets": latest.open_tickets,
        "patching": {
            "pending_patches": latest.pending_patches,
            "device_count": latest.device_count
        },
        "history": history
    }))
}

fn database_url() -> Result<String, String> {
    std::env::var("DATABASE_URL").map_err(|_| String::from("Missing DATABASE_URL in .env"))
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
