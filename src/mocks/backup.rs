use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "green":  1,
        "yellow": 1,
        "orange": 1,
        "red":    4,
        "grey":   1
    })
}
