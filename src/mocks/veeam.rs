use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "_mock": true,
        "green":  5,
        "yellow": 1,
        "orange": 0,
        "red":    0,
        "grey":   0
    })
}
