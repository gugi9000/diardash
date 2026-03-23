use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "active_alerts": 12,
        "open_tickets": 7,
        "patching": {
            "pending_patches": 43,
            "device_count": 120
        },
        "history": [
            {"day": "Mon", "alerts": 12, "tickets": 7},
            {"day": "Tue", "alerts": 15, "tickets": 9},
            {"day": "Wed", "alerts": 10, "tickets": 6},
            {"day": "Thu", "alerts": 18, "tickets": 11},
            {"day": "Fri", "alerts": 8,  "tickets": 5},
            {"day": "Sat", "alerts": 4,  "tickets": 3},
            {"day": "Sun", "alerts": 12, "tickets": 7}
        ]
    })
}
