use std::env;
use std::sync::Once;

use dotenvy::dotenv;
use serde_json::Value;

use crate::weather_payload::WeatherPayload;

static WEATHER_CONFIG_LOG_ONCE: Once = Once::new();
static WEATHER_KEY_LOG_ONCE: Once = Once::new();
const DEFAULT_LAT: f64 = 55.6761;
const DEFAULT_LON: f64 = 12.5683;

pub async fn fetch_payload() -> Result<Value, String> {
    dotenv().ok();
    log_weather_config_once();

    let (api_key, api_key_source) = read_api_key().ok_or_else(|| {
        String::from("Missing API key in .env (tried OPENWEATHER_API_KEY, OPENWEATHER_KEY, API_KEY)")
    })?;
    let debug: String = env::var("DEBUG").unwrap_or_default();

    if debug == "true" {
        eprintln!("Weather debug: resolved coordinates lat={DEFAULT_LAT}, lon={DEFAULT_LON}");
        log_api_key_once(api_key_source, &api_key);
    }

    let client = reqwest::Client::new();

    let (lat, lon) = resolve_coordinates()?;

    let data = request_json(
        &client,
        "https://api.openweathermap.org/data/3.0/onecall",
        &[
            ("lat", lat.to_string()),
            ("lon", lon.to_string()),
            ("appid", api_key.clone()),
            ("units", String::from("metric")),
            ("exclude", String::from("minutely,hourly,daily,alerts")),
        ],
    )
    .await?;

    let current = data
        .get("current")
        .ok_or_else(|| String::from("Missing current in OpenWeather 3.0 response"))?;

    let temperature_c = current
        .get("temp")
        .and_then(Value::as_f64)
        .ok_or_else(|| String::from("Missing current.temp in OpenWeather response"))?
        .round() as i64;

    let humidity_pct = current
        .get("humidity")
        .and_then(Value::as_i64)
        .ok_or_else(|| String::from("Missing current.humidity in OpenWeather response"))?;

    let wind_kmh = current
        .get("wind_speed")
        .and_then(Value::as_f64)
        .map(|speed_ms| (speed_ms * 3.6).round() as i64)
        .ok_or_else(|| String::from("Missing current.wind_speed in OpenWeather response"))?;

    let (condition, icon_code) = current
        .get("weather")
        .and_then(Value::as_array)
        .and_then(|weather| weather.first())
        .map(|entry| {
            let description = entry
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or("Unknown");
            let icon = entry.get("icon").and_then(Value::as_str).unwrap_or("");
            (to_title_case(description), icon)
        })
        .ok_or_else(|| String::from("Missing current.weather[0] in OpenWeather response"))?;

    Ok(WeatherPayload {
        temperature_c,
        condition,
        icon: map_openweather_icon(icon_code).to_string(),
        humidity_pct,
        wind_kmh,
        source: String::from("openweather-3.0"),
        fallback_used: false,
    }
    .to_json())
}

fn log_weather_config_once() {
    WEATHER_CONFIG_LOG_ONCE.call_once(|| {
        if env::var("OPENWEATHER_COORDS").is_ok() {
            eprintln!(
                "Weather config: OpenWeather 3.0 enabled using OPENWEATHER_COORDS"
            );
            return;
        }

        let has_lat = env::var("OPENWEATHER_LAT").is_ok();
        let has_lon = env::var("OPENWEATHER_LON").is_ok();

        if has_lat && has_lon {
            eprintln!(
                "Weather config: OpenWeather 3.0 enabled using OPENWEATHER_LAT/OPENWEATHER_LON"
            );
        } else {
            eprintln!(
                "Weather config: OpenWeather 3.0 enabled using default coordinates lat={DEFAULT_LAT}, lon={DEFAULT_LON}"
            );
        }
    });
}

fn log_api_key_once(source: &str, key: &str) {
    WEATHER_KEY_LOG_ONCE.call_once(|| {
        eprintln!("Weather debug: using API key from {source} = {key}");
    });
}

fn resolve_coordinates() -> Result<(f64, f64), String> {
    if let Some(coords) = read_coordinates_from_env()? {
        return Ok(coords);
    }

    Ok((DEFAULT_LAT, DEFAULT_LON))
}

fn read_coordinates_from_env() -> Result<Option<(f64, f64)>, String> {
    if let Some(coords) = read_coordinates_compact()? {
        return Ok(Some(coords));
    }

    let lat_raw = env::var("OPENWEATHER_LAT").ok();
    let lon_raw = env::var("OPENWEATHER_LON").ok();

    match (lat_raw, lon_raw) {
        (Some(lat), Some(lon)) => {
            let lat_parsed = lat
                .parse::<f64>()
                .map_err(|_| String::from("OPENWEATHER_LAT is not a valid number"))?;
            let lon_parsed = lon
                .parse::<f64>()
                .map_err(|_| String::from("OPENWEATHER_LON is not a valid number"))?;
            Ok(Some((lat_parsed, lon_parsed)))
        }
        (None, None) => Ok(None),
        _ => Err(String::from(
            "Set both OPENWEATHER_LAT and OPENWEATHER_LON, or neither to use OPENWEATHER_COORDS/default coordinates",
        )),
    }
}

fn read_coordinates_compact() -> Result<Option<(f64, f64)>, String> {
    let raw = match env::var("OPENWEATHER_COORDS") {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };

    let mut parts = raw.split(',').map(str::trim);
    let lat = parts.next().ok_or_else(|| {
        String::from("OPENWEATHER_COORDS must be in the format 'lat,lon'")
    })?;
    let lon = parts.next().ok_or_else(|| {
        String::from("OPENWEATHER_COORDS must be in the format 'lat,lon'")
    })?;

    if parts.next().is_some() {
        return Err(String::from(
            "OPENWEATHER_COORDS must contain exactly two values: 'lat,lon'",
        ));
    }

    let lat_parsed = lat
        .parse::<f64>()
        .map_err(|_| String::from("OPENWEATHER_COORDS latitude is not a valid number"))?;
    let lon_parsed = lon
        .parse::<f64>()
        .map_err(|_| String::from("OPENWEATHER_COORDS longitude is not a valid number"))?;

    Ok(Some((lat_parsed, lon_parsed)))
}

async fn request_json(
    client: &reqwest::Client,
    url: &str,
    query: &[(&str, String)],
) -> Result<Value, String> {
    let response = client
        .get(url)
        .query(query)
        .send()
        .await
        .map_err(|err| format!("Request error: {err}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| String::from("<no response body>"));
        return Err(format!("OpenWeather returned {status}: {body}"));
    }

    response
        .json::<Value>()
        .await
        .map_err(|err| format!("Invalid JSON response: {err}"))
}

fn read_api_key() -> Option<(String, &'static str)> {
    if let Ok(key) = env::var("OPENWEATHER_API_KEY") {
        return Some((key, "OPENWEATHER_API_KEY"));
    }

    if let Ok(key) = env::var("OPENWEATHER_KEY") {
        return Some((key, "OPENWEATHER_KEY"));
    }

    if let Ok(key) = env::var("API_KEY") {
        return Some((key, "API_KEY"));
    }

    None
}

fn to_title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let mut output = first.to_uppercase().to_string();
                    output.push_str(chars.as_str());
                    output
                }
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn map_openweather_icon(icon_code: &str) -> &'static str {
    match icon_code {
        "01d" | "01n" => "clear-day",
        "02d" | "02n" => "partly-cloudy-day",
        "03d" | "03n" | "04d" | "04n" | "50d" | "50n" => "cloudy",
        "09d" | "09n" | "10d" | "10n" => "rain",
        "11d" | "11n" => "thunderstorm",
        "13d" | "13n" => "snow",
        _ => "cloudy",
    }
}
