use serde_json::{json, Value};

pub async fn fetch_payload() -> Result<Value, String> {
    Ok(json!({
        "green":  85,
        "yellow": 10,
        "orange": 5,
        "red":    3,
        "grey":   2
    }))
}
