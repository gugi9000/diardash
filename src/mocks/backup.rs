use chrono::Local;
use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "_mock": true,
        "green":  1,
        "yellow": 1,
        "orange": 1,
        "red":    1,
        "grey":   1,
        "_meta": {
            "last_update": Local::now().to_rfc3339()
        }
    })
}
