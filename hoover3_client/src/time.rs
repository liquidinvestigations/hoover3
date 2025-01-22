pub fn current_time() -> f64 {
    web_time::SystemTime::now()
        .duration_since(web_time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}
