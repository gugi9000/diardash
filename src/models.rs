use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::atera_metrics)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AteraMetric {
    #[allow(dead_code)]
    pub id: i32,
    pub recorded_at: NaiveDateTime,
    pub active_alerts: i32,
    pub open_tickets: i32,
    pub pending_patches: i32,
    pub device_count: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::atera_metrics)]
pub struct NewAteraMetric {
    pub recorded_at: NaiveDateTime,
    pub active_alerts: i32,
    pub open_tickets: i32,
    pub pending_patches: i32,
    pub device_count: i32,
}
