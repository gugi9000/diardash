use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "alerts": [
            {"device": "SRV-DC01",   "service": "CPU Usage",       "transition_time": "2026-03-23T08:14:00Z", "type": "Warning"},
            {"device": "WRK-JSMITH", "service": "Disk Space",      "transition_time": "2026-03-23T07:45:00Z", "type": "Critical"},
            {"device": "SRV-FILE02", "service": "Backup Agent",    "transition_time": "2026-03-23T06:30:00Z", "type": "Failed"},
            {"device": "WRK-MJONES", "service": "Windows Updates", "transition_time": "2026-03-22T23:00:00Z", "type": "Warning"},
            {"device": "SRV-MAIL01", "service": "SMTP Service",    "transition_time": "2026-03-22T21:15:00Z", "type": "Critical"}
        ]
    })
}
