use rocket::serde::json::{Json, Value};
use rocket_dyn_templates::{context, Template};
use serde_json::json;

// Default route — renders the dashboard HTML
#[rocket::get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

// Widget 1 — Wazuh: critical/high alert counts + 7-day history
#[rocket::get("/api/wazuh")]
pub fn get_wazuh_alerts() -> Json<Value> {
    Json(json!({
        "count": {"critical": 44, "high": 5},
        "history": [
            {"day": "Mon", "critical": 44, "high": 5},
            {"day": "Tue", "critical": 38, "high": 7},
            {"day": "Wed", "critical": 41, "high": 3},
            {"day": "Thu", "critical": 50, "high": 9},
            {"day": "Fri", "critical": 33, "high": 4},
            {"day": "Sat", "critical": 29, "high": 2},
            {"day": "Sun", "critical": 44, "high": 5}
        ]
    }))
}

// Widget 2 — Atera: active alerts, open tickets, patching status + 7-day history
#[rocket::get("/api/atera")]
pub fn get_atera_data() -> Json<Value> {
    Json(json!({
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
    }))
}

// Widget 3 — Vipre AV: device compliance counts + 7-day history
#[rocket::get("/api/vipre")]
pub fn get_vipre_data() -> Json<Value> {
    Json(json!({
        "outdated_devices": 8,
        "lost_devices": 2,
        "in_ad_no_av": 5,
        "av_not_in_ad": 3,
        "history": [
            {"day": "Mon", "outdated": 8,  "lost": 2, "in_ad_no_av": 5, "av_not_in_ad": 3},
            {"day": "Tue", "outdated": 9,  "lost": 2, "in_ad_no_av": 4, "av_not_in_ad": 3},
            {"day": "Wed", "outdated": 7,  "lost": 3, "in_ad_no_av": 5, "av_not_in_ad": 4},
            {"day": "Thu", "outdated": 10, "lost": 2, "in_ad_no_av": 6, "av_not_in_ad": 2},
            {"day": "Fri", "outdated": 8,  "lost": 1, "in_ad_no_av": 5, "av_not_in_ad": 3},
            {"day": "Sat", "outdated": 8,  "lost": 2, "in_ad_no_av": 5, "av_not_in_ad": 3},
            {"day": "Sun", "outdated": 8,  "lost": 2, "in_ad_no_av": 5, "av_not_in_ad": 3}
        ]
    }))
}

// Widget 4 — Backup: pie chart of device backup statuses
#[rocket::get("/api/backup")]
pub fn get_backup_status() -> Json<Value> {
    Json(json!({
        "green":  85,
        "yellow": 10,
        "orange": 5,
        "red":    3,
        "grey":   2
    }))
}

// Widget 5 — AD Metrics: expired passwords, locked accounts + 7-day history
#[rocket::get("/api/ad")]
pub fn get_ad_metrics() -> Json<Value> {
    Json(json!({
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
    }))
}

// Widget 6 — N-central: list of active alerts
#[rocket::get("/api/ncentral")]
pub fn get_ncentral_alerts() -> Json<Value> {
    Json(json!({
        "alerts": [
            {"device": "SRV-DC01",   "service": "CPU Usage",       "transition_time": "2026-03-23T08:14:00Z", "type": "Warning"},
            {"device": "WRK-JSMITH", "service": "Disk Space",      "transition_time": "2026-03-23T07:45:00Z", "type": "Critical"},
            {"device": "SRV-FILE02", "service": "Backup Agent",    "transition_time": "2026-03-23T06:30:00Z", "type": "Failed"},
            {"device": "WRK-MJONES", "service": "Windows Updates", "transition_time": "2026-03-22T23:00:00Z", "type": "Warning"},
            {"device": "SRV-MAIL01", "service": "SMTP Service",    "transition_time": "2026-03-22T21:15:00Z", "type": "Critical"}
        ]
    }))
}

// Widget 7 — Veeam Jobs: pie chart of backup job statuses
#[rocket::get("/api/veeam")]
pub fn get_veeam_jobs() -> Json<Value> {
    Json(json!({
        "green":  72,
        "yellow": 8,
        "orange": 4,
        "red":    2,
        "grey":   1
    }))
}

// Widget 8 — Weather: current temperature and conditions
#[rocket::get("/api/weather")]
pub fn get_weather() -> Json<Value> {
    Json(json!({
        "temperature_c": 7,
        "condition": "Partly Cloudy",
        "icon": "partly-cloudy-day",
        "humidity_pct": 68,
        "wind_kmh": 14
    }))
}

// Widget 9 — Duck: current date and time
#[rocket::get("/api/datetime")]
pub fn get_datetime() -> Json<Value> {
    Json(json!({
        "date": "Monday, March 23 2026",
        "time": "14:32:07"
    }))
}

