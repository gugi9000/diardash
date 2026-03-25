#[macro_use]
extern crate rocket;

mod mocks;
mod models;
mod routes;
mod schema;
mod services;
mod weather_payload;

use crate::routes::{
    get_ad_metrics, get_atera_data, get_backup_status, get_datetime, get_ncentral_alerts,
    get_veeam_jobs, get_vipre_data, get_wazuh_alerts, get_weather, index,
};
use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    if let Err(error) = services::atera::initialize_database() {
        eprintln!("Atera database initialization failed: {error}");
    }

    rocket::build()
        .attach(Template::fairing())
        .mount("/static", rocket::fs::FileServer::from("static"))
        .mount(
            "/",
            routes![
                index,
                get_wazuh_alerts,
                get_atera_data,
                get_vipre_data,
                get_backup_status,
                get_ad_metrics,
                get_ncentral_alerts,
                get_veeam_jobs,
                get_weather,
                get_datetime,
            ],
        )
}
