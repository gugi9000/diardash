use std::collections::BTreeMap;
use std::fs;

use chrono::{Duration, NaiveDate, NaiveDateTime};
use serde_json::{json, Value};

const AD_STATS_FILE: &str = "data/ad-stats.txt";
const HISTORY_DAYS: i64 = 7;

pub fn fetch_payload() -> Result<Value, String> {
    let file_content = fs::read_to_string(AD_STATS_FILE)
        .map_err(|error| format!("Failed to read {AD_STATS_FILE}: {error}"))?;

    let mut records = parse_records(&file_content)?;
    if records.is_empty() {
        return Err(format!("No AD records found in {AD_STATS_FILE}"));
    }

    records.sort_by_key(|record| record.recorded_at);
    let latest = records
        .last()
        .ok_or_else(|| format!("No AD records found in {AD_STATS_FILE}"))?;

    let mut by_day: BTreeMap<NaiveDate, (i32, i32, NaiveDateTime)> = BTreeMap::new();
    for record in &records {
        let day = record.recorded_at.date();
        match by_day.get(&day) {
            Some((_, _, existing)) if *existing >= record.recorded_at => {}
            _ => {
                by_day.insert(
                    day,
                    (record.locked_out, record.password_expired, record.recorded_at),
                );
            }
        }
    }

    let latest_day = latest.recorded_at.date();
    let history_start = latest_day - Duration::days(HISTORY_DAYS - 1);
    let mut history = Vec::new();
    for offset in 0..HISTORY_DAYS {
        let day = history_start + Duration::days(offset);
        let (locked_out, password_expired) = by_day
            .get(&day)
            .map(|(locked_out, password_expired, _)| (*locked_out, *password_expired))
            .unwrap_or((0, 0));

        history.push(json!({
            "day": day.format("%a").to_string(),
            "password_expired": password_expired,
            "locked_out": locked_out,
        }));
    }

    Ok(json!({
        "_mock": false,
        "password_expired": latest.password_expired,
        "locked_out": latest.locked_out,
        "history": history,
        "_meta": {
            "source": AD_STATS_FILE,
            "latest_timestamp": latest.recorded_at.format("%Y-%m-%d %H:%M:%S").to_string()
        }
    }))
}

struct AdRecord {
    recorded_at: NaiveDateTime,
    locked_out: i32,
    password_expired: i32,
}

fn parse_records(content: &str) -> Result<Vec<AdRecord>, String> {
    let mut records = Vec::new();

    for (index, raw_line) in content.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split(',').map(str::trim);
        let recorded_at_raw = parts
            .next()
            .ok_or_else(|| format!("Line {}: missing timestamp", index + 1))?;
        let locked_out_raw = parts
            .next()
            .ok_or_else(|| format!("Line {}: missing LockedOut value", index + 1))?;
        let password_expired_raw = parts
            .next()
            .ok_or_else(|| format!("Line {}: missing Pwd Expired value", index + 1))?;

        if parts.next().is_some() {
            return Err(format!(
                "Line {}: expected 3 comma-separated fields (timestamp, LockedOut, Pwd Expired)",
                index + 1
            ));
        }

        let recorded_at = NaiveDateTime::parse_from_str(recorded_at_raw, "%m/%d/%Y %H:%M:%S")
            .map_err(|error| format!("Line {}: invalid timestamp '{}': {error}", index + 1, recorded_at_raw))?;
        let locked_out = locked_out_raw
            .parse::<i32>()
            .map_err(|error| format!("Line {}: invalid LockedOut '{}': {error}", index + 1, locked_out_raw))?;
        let password_expired = password_expired_raw
            .parse::<i32>()
            .map_err(|error| format!("Line {}: invalid Pwd Expired '{}': {error}", index + 1, password_expired_raw))?;

        records.push(AdRecord {
            recorded_at,
            locked_out,
            password_expired,
        });
    }

    Ok(records)
}