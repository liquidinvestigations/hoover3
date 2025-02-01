#[tokio::main]
async fn main() {
    hoover3_database::tracing::init_tracing();
    hoover3_taskdef::run_worker::<hoover3_filesystem_scanner::AllTasks>()
        .await
        .unwrap();
}
