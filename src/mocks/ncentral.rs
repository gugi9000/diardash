use chrono::Local;
use serde_json::{json, Value};

pub fn payload() -> Value {
    let now = Local::now();
    json!({
        "_mock": true,
        "alerts": [
            {"device": "NCOD",   "service": "N-central API",
                   "transition_time": now.to_rfc3339(), "type": "Critical"},
        ]
    })
}
