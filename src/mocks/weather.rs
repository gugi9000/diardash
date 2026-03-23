use serde_json::Value;

use crate::weather_payload::WeatherPayload;

pub fn payload() -> Value {
    WeatherPayload {
        temperature_c: 7,
        condition: String::from("Partly Cloudy"),
        icon: String::from("partly-cloudy-day"),
        humidity_pct: 68,
        wind_kmh: 14,
        source: String::from("mock"),
        fallback_used: true,
    }
    .to_json()
}
