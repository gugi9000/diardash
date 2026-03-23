use chrono::DateTime;
use dotenvy::dotenv;
use serde_json::{json, Value};
use std::sync::Once;

const TARGET_NCOD_ID: &str = "103";
static NCENTRAL_CONFIG_LOG_ONCE: Once = Once::new();

pub async fn fetch_payload() -> Result<Value, String> {
    dotenv().ok();

    let endpoint = std::env::var("NCENTRAL_ENDPOINT")
        .map_err(|_| String::from("Missing NCENTRAL_ENDPOINT in .env"))?;

    log_ncentral_config_once(&endpoint);

    let client = reqwest::Client::new();
    let response = client
        .get(&endpoint)
        .send()
        .await
        .map_err(|error| format!("Request error: {error}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| String::from("<no response body>"));
        return Err(format!("N-central endpoint returned {status}: {body}"));
    }

    let raw: Value = response
        .json()
        .await
        .map_err(|error| format!("Invalid JSON response: {error}"))?;

    let issues = raw
        .as_array()
        .ok_or_else(|| String::from("N-central payload was not a JSON array"))?;

    let alerts = issues
        .iter()
        .filter(matches_target_customer)
        .map(transform_issue)
        .collect::<Vec<Value>>();

    Ok(json!({ "alerts": alerts }))
}

fn log_ncentral_config_once(endpoint: &str) {
    NCENTRAL_CONFIG_LOG_ONCE.call_once(|| {
        eprintln!("N-central config: using NCENTRAL_ENDPOINT={endpoint}");
    });
}

fn matches_target_customer(issue: &&Value) -> bool {
    issue
        .get("customer")
        .and_then(|customer| customer.get("ncod_id"))
        .and_then(Value::as_str)
        .map(|ncod_id| ncod_id == TARGET_NCOD_ID)
        .unwrap_or(false)
}

fn transform_issue(issue: &Value) -> Value {
    let device = issue
        .get("device_name")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("Unknown device");

    let service = issue
        .get("service")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("Unknown service");

    let transition_raw = issue
        .get("transition_time")
        .and_then(Value::as_str)
        .unwrap_or("");

    let transition_time = normalize_transition_time(transition_raw)
        .unwrap_or_else(|| transition_raw.to_string());

    let state = issue.get("state").and_then(Value::as_i64);
    let alert_type = map_state_to_alert_type(state);

    json!({
        "device": device,
        "service": service,
        "transition_time": transition_time,
        "type": alert_type,
    })
}

fn normalize_transition_time(input: &str) -> Option<String> {
    DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S%.3f %z")
        .or_else(|_| DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S %z"))
        .map(|dt| dt.to_rfc3339())
        .ok()
}

fn map_state_to_alert_type(state: Option<i64>) -> &'static str {
    match state {
        Some(5) => "Critical",
        Some(4) => "Failed",
        Some(3) | Some(2) => "Warning",
        Some(1) => "Info",
        _ => "Unknown",
    }
}
