use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "green":  72,
        "yellow": 8,
        "orange": 4,
        "red":    2,
        "grey":   1
    })
}
