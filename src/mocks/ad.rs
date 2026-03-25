use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "_mock": true,
        "password_expired": 14,
        "locked_out": 3,
        "history": [
            {"day": "Mon", "password_expired": 14, "locked_out": 3},
            {"day": "Tue", "password_expired": 16, "locked_out": 4},
            {"day": "Wed", "password_expired": 13, "locked_out": 2},
            {"day": "Thu", "password_expired": 17, "locked_out": 5},
            {"day": "Fri", "password_expired": 12, "locked_out": 1},
            {"day": "Sat", "password_expired": 14, "locked_out": 3},
            {"day": "Sun", "password_expired": 14, "locked_out": 3}
        ]
    })
}
