//! CLI tool to run a worker;

#[tokio::main]
async fn main() {
    hoover3_tracing::init_tracing();
    hoover3_taskdef::tasks::run_worker::<hoover3_filesystem_scanner::tasks::AllTasks>().unwrap();
}
