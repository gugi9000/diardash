diesel::table! {
    atera_metrics (id) {
        id -> Integer,
        recorded_at -> Timestamp,
        active_alerts -> Integer,
        open_tickets -> Integer,
        pending_patches -> Integer,
        device_count -> Integer,
    }
}
