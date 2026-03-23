use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "green":  85,
        "yellow": 10,
        "orange": 5,
        "red":    3,
        "grey":   2
    })
}
