use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "_mock": true,
        "count": {
            "critical": 44,
            "high": 5,
            "active_devices": 412,
            "disconnected_devices": 37
        },
        "history": [
            {"day": "Mon", "critical": 44, "high": 5, "active_devices": 406, "disconnected_devices": 34},
            {"day": "Tue", "critical": 38, "high": 7, "active_devices": 398, "disconnected_devices": 41},
            {"day": "Wed", "critical": 41, "high": 3, "active_devices": 421, "disconnected_devices": 29},
            {"day": "Thu", "critical": 50, "high": 9, "active_devices": 415, "disconnected_devices": 35},
            {"day": "Fri", "critical": 33, "high": 4, "active_devices": 409, "disconnected_devices": 32},
            {"day": "Sat", "critical": 29, "high": 2, "active_devices": 401, "disconnected_devices": 28},
            {"day": "Sun", "critical": 44, "high": 5, "active_devices": 412, "disconnected_devices": 37}
        ]
    })
}
