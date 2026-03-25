use chrono::Local;
use rocket::serde::json::{Json, Value};
use rocket_dyn_templates::{context, Template};
use serde_json::json;

use crate::mocks;
use crate::services;

#[rocket::get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[rocket::get("/api/wazuh")]
pub fn get_wazuh_alerts() -> Json<Value> {
    Json(mocks::wazuh::payload())
}

#[rocket::get("/api/atera")]
pub fn get_atera_data() -> Json<Value> {
    match services::atera::fetch_payload() {
        Ok(payload) => Json(payload),
        Err(error) => {
            eprintln!("Atera fetch failed, using mock payload: {error}");
            Json(mocks::atera::payload())
        }
    }
}

#[rocket::get("/api/vipre")]
pub fn get_vipre_data() -> Json<Value> {
    Json(mocks::vipre::payload())
}

#[rocket::get("/api/backup")]
pub async fn get_backup_status() -> Json<Value> {
    match services::cove::fetch_payload().await {
        Ok(payload) => Json(payload),
        Err(error) => {
            eprintln!("Cove fecth failed, using mock payload: {error}");
            Json(mocks::backup::payload())
        }
    }
}

#[rocket::get("/api/ad")]
pub fn get_ad_metrics() -> Json<Value> {
    Json(mocks::ad::payload())
}

#[rocket::get("/api/ncentral")]
pub async fn get_ncentral_alerts() -> Json<Value> {
    match services::ncentral::fetch_payload().await {
        Ok(payload) => Json(payload),
        Err(error) => {
            eprintln!("N-central fetch failed, using mock payload: {error}");
            Json(mocks::ncentral::payload())
        }
    }
}

#[rocket::get("/api/veeam")]
pub fn get_veeam_jobs() -> Json<Value> {
    Json(mocks::veeam::payload())
}

#[rocket::get("/api/weather")]
pub async fn get_weather() -> Json<Value> {
    match services::openweather::fetch_payload().await {
        Ok(payload) => Json(payload),
        Err(error) => {
            eprintln!("OpenWeather fetch failed, using mock payload: {error}");
            Json(mocks::weather::payload())
        }
    }
}

#[rocket::get("/api/datetime")]
pub fn get_datetime() -> Json<Value> {
    let now = Local::now();

    Json(json!({
        "timestamp": now.to_rfc3339(),
        "date": now.format("%A, %B %-d %Y").to_string(),
        "time": now.format("%H:%M:%S").to_string()
    }))
}
