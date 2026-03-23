use serde_json::{json, Value};

pub struct WeatherPayload {
    pub temperature_c: i64,
    pub condition: String,
    pub icon: String,
    pub humidity_pct: i64,
    pub wind_kmh: i64,
    pub source: String,
    pub fallback_used: bool,
}

impl WeatherPayload {
    pub fn to_json(self) -> Value {
        json!({
            "temperature_c": self.temperature_c,
            "condition": self.condition,
            "icon": self.icon,
            "humidity_pct": self.humidity_pct,
            "wind_kmh": self.wind_kmh,
            "source": self.source,
            "fallback_used": self.fallback_used
        })
    }
}
