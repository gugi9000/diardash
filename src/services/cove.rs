use dotenvy::dotenv;
use reqwest::header::{ACCEPT, ACCEPT_ENCODING, CONTENT_TYPE};
use serde_json::{json, Value};

fn is_debug_enabled() -> bool {
    std::env::var("DEBUG")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
        .unwrap_or(false)
}

fn parse_json_response(body_bytes: &[u8], headers: &reqwest::header::HeaderMap) -> Result<Value, String> {
    match serde_json::from_slice(body_bytes) {
        Ok(value) => Ok(value),
        Err(primary_error) => {
            // Some Cove deployments return JSON encoded as Latin-1/Windows-1252.
            let latin1_text = body_bytes
                .iter()
                .map(|&byte| byte as char)
                .collect::<String>();
            serde_json::from_str::<Value>(&latin1_text).map_err(|fallback_error| {
                let content_type = headers
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("<missing>");
                let content_encoding = headers
                    .get("content-encoding")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("<missing>");
                let preview = String::from_utf8_lossy(body_bytes)
                    .chars()
                    .take(220)
                    .collect::<String>();

                format!(
                    "Failed to parse Cove response JSON: utf8-error={primary_error}; latin1-error={fallback_error}. content-type={content_type}, content-encoding={content_encoding}, body-preview={preview}"
                )
            })
        }
    }
}

fn customer_partner_id() -> Result<i64, String> {
    let raw = std::env::var("COVE_CUSTOMER_ID")
        .or_else(|_| std::env::var("COVER_CUSTOMER_ID"))
        .map_err(|_| String::from("Missing COVE_CUSTOMER_ID in .env"))?;

    raw.trim().parse::<i64>().map_err(|error| {
        format!("Invalid COVE_CUSTOMER_ID value '{raw}': {error}")
    })
}

/// Shared JSON-RPC POST helper. Sends `body`, reads the response, and returns the
/// parsed payload. HTTP and API-level errors are returned as `Err`.
async fn post_json_rpc(
    client: &reqwest::Client,
    endpoint: &str,
    body: Value,
) -> Result<Value, String> {
    let response = client
        .post(endpoint)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT_ENCODING, "identity")
        .json(&body)
        .send()
        .await
        .map_err(|error| format!("Cove HTTP request failed: {error}"))?;

    let http_status = response.status();
    let headers = response.headers().clone();
    let body_bytes = response
        .bytes()
        .await
        .map_err(|error| format!("Failed to read Cove response body: {error}"))?;
    let payload = parse_json_response(&body_bytes, &headers)?;

    if !http_status.is_success() {
        return Err(format!("Cove request failed with HTTP {http_status}: {payload}"));
    }
    if let Some(error) = payload.get("error") {
        return Err(format!("Cove API returned error: {error}"));
    }

    Ok(payload)
}

/// Authenticate and return a visa token.
async fn get_visa() -> Result<String, String> {
    dotenv().ok();

    let endpoint = std::env::var("COVE_ENDPOINT")
        .map_err(|_| String::from("Missing COVE_ENDPOINT in .env"))?;
    let username = std::env::var("COVE_USER")
        .map_err(|_| String::from("Missing COVE_USER in .env"))?;
    let password = std::env::var("COVE_PASSWORD")
        .map_err(|_| String::from("Missing COVE_PASSWORD in .env"))?;
    let partner = std::env::var("COVE_PARTNER")
        .map_err(|_| String::from("Missing COVE_PARTNER in .env"))?;

    let client = reqwest::Client::new();
    let payload = post_json_rpc(&client, &endpoint, json!({
        "jsonrpc": "2.0",
        "method": "Login",
        "params": {
            "partner": partner,
            "username": username,
            "password": password
        },
        "id": "1"
    }))
    .await
    .map_err(|error| format!("Cove Login failed: {error}"))?;

    payload
        .get("visa")
        .and_then(Value::as_str)
        .map(String::from)
        .ok_or_else(|| format!("Cove Login response missing visa field: {payload}"))
}

/// List all devices for the configured customer partner.
pub async fn get_devices_for_customer() -> Result<Vec<Value>, String> {
    dotenv().ok();

    let endpoint = std::env::var("COVE_ENDPOINT")
        .map_err(|_| String::from("Missing COVE_ENDPOINT in .env"))?;
    let partner_id = customer_partner_id()?;
    let visa = get_visa().await?;

    let client = reqwest::Client::new();
    let payload = post_json_rpc(&client, &endpoint, json!({
        "id": "jsonrpc",
        "jsonrpc": "2.0",
        "visa": visa,
        "method": "EnumerateAccounts",
        "params": { "partnerId": partner_id }
    }))
    .await?;

    let devices = payload
        .get("result")
        .and_then(|r| r.get("result"))
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| format!("Cove EnumerateAccounts response missing result.result array: {payload}"))?;

    if is_debug_enabled() {
        eprintln!(
            "Cove EnumerateAccounts: loaded {} devices for partnerId={partner_id}",
            devices.len()
        );
        for device in &devices {
            let id   = device.get("Id").and_then(Value::as_i64).unwrap_or_default();
            let name = device.get("Name").and_then(Value::as_str).unwrap_or("<unknown>");
            let kind = device.get("Type").and_then(Value::as_str).unwrap_or("<unknown>");
            eprintln!("  device id={id} type={kind} name={name}");
        }
    }

    Ok(devices)
}

/// Look up a named column value inside a `Settings` array.
/// Settings is structured as: `[{"I1": "value"}, {"D09F00": "5"}, …]`
fn get_setting<'a>(settings: &'a Value, key: &str) -> Option<&'a str> {
    settings
        .as_array()?
        .iter()
        .find_map(|entry| entry.get(key).and_then(Value::as_str))
}

/// Map a Cove Last Session Status code (column F00) to a dashboard colour.
///
/// | Code | Meaning              | Colour |
/// |------|----------------------|--------|
/// | 1    | In process           | orange |
/// | 2    | Failed               | red    |
/// | 3    | Aborted              | red    |
/// | 5    | Completed            | green  |
/// | 6    | Interrupted          | orange |
/// | 7    | NotStarted           | grey   |
/// | 8    | CompletedWithErrors  | yellow |
/// | 9    | InProgressWithFaults | yellow |
/// | 10   | OverQuota            | red    |
/// | 11   | NoSelection          | grey   |
/// | 12   | Restarted            | orange |
fn status_to_color(code: i64) -> &'static str {
    match code {
        5          => "green",
        8 | 9      => "yellow",
        1 | 6 | 12 => "orange",
        2 | 3 | 10 => "red",
        _          => "grey",
    }
}

/// Fetch device statistics for `partner_id` using `EnumerateAccountStatistics`.
/// Requests device name (I1), computer name (I18), and the Total data-source
/// last-session status (D09F00). Paginates 500 records at a time.
async fn fetch_device_statistics(
    client: &reqwest::Client,
    endpoint: &str,
    visa: &str,
    partner_id: i64,
) -> Result<Vec<Value>, String> {
    const PAGE_SIZE: i64 = 500;
    let mut all_records: Vec<Value> = Vec::new();
    let mut start: i64 = 0;

    loop {
        let payload = post_json_rpc(client, endpoint, json!({
            "id": "jsonrpc",
            "jsonrpc": "2.0",
            "visa": visa,
            "method": "EnumerateAccountStatistics",
            "params": {
                "query": {
                    "PartnerId": partner_id,
                    "SelectionMode": "Merged",
                    "StartRecordNumber": start,
                    "RecordsCount": PAGE_SIZE,
                    "Columns": ["I1", "I18", "D09F00"]
                }
            }
        }))
        .await?;

        let records = payload
            .get("result")
            .and_then(|r| r.get("result"))
            .and_then(Value::as_array)
            .cloned()
            .ok_or_else(|| format!("Cove EnumerateAccountStatistics response missing result.result array: {payload}"))?;

        let page_len = records.len() as i64;

        if is_debug_enabled() {
            eprintln!(
                "Cove EnumerateAccountStatistics: page start={start} returned {page_len} records"
            );
        }

        all_records.extend(records);

        if page_len < PAGE_SIZE {
            break;
        }
        start += PAGE_SIZE;
    }

    Ok(all_records)
}

pub async fn fetch_payload() -> Result<Value, String> {
    dotenv().ok();

    let endpoint = std::env::var("COVE_ENDPOINT")
        .map_err(|_| String::from("Missing COVE_ENDPOINT in .env"))?;
    let partner_id = customer_partner_id()?;
    let visa = get_visa().await?;
    let client = reqwest::Client::new();

    let statistics = fetch_device_statistics(&client, &endpoint, &visa, partner_id).await?;

    let (mut green, mut yellow, mut orange, mut red, mut grey) = (0i32, 0i32, 0i32, 0i32, 0i32);

    for record in &statistics {
        let settings   = record.get("Settings").unwrap_or(&Value::Null);
        let status_str = get_setting(settings, "D09F00");
        let color      = status_str
            .and_then(|s| s.parse::<i64>().ok())
            .map(status_to_color)
            .unwrap_or("grey");

        if is_debug_enabled() {
            let account_id = record.get("AccountId").and_then(Value::as_i64).unwrap_or_default();
            let name       = get_setting(settings, "I1").unwrap_or("<unknown>");
            let computer   = get_setting(settings, "I18").unwrap_or("<unknown>");
            eprintln!(
                "  device id={account_id} name={name} computer={computer} D09F00={} → {color}",
                status_str.unwrap_or("<none>")
            );
        }

        match color {
            "green"  => green  += 1,
            "yellow" => yellow += 1,
            "orange" => orange += 1,
            "red"    => red    += 1,
            _        => grey   += 1,
        }
    }

    if is_debug_enabled() {
        eprintln!(
            "Cove backup summary: green={green} yellow={yellow} orange={orange} red={red} grey={grey} ({} total)",
            statistics.len()
        );
    }

    Ok(json!({
        "green":  green,
        "yellow": yellow,
        "orange": orange,
        "red":    red,
        "grey":   grey
    }))
}

#[test]
fn test_get_visa() {
    let visa = tokio_test::block_on(get_visa());
    assert!(
        visa.is_ok(),
        "Expected get_visa to succeed, but it failed with: {:?}",
        visa.err()
    );
}
