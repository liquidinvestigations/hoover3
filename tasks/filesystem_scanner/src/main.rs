//! Filesystem scanner worker binary entrypoint.

/// Run filesystem scanner worker
fn main() {
    hoover3_tracing::init_tracing();
    hoover3_taskdef::run_worker::<hoover3_filesystem_scanner::AllTasks>().unwrap();
}
