use serde_json::{json, Value};

pub fn payload() -> Value {
    json!({
        "_mock": true,
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
    })
}
