//! CLI tool to migrate all databases;

#[tokio::main]
async fn main() {
    hoover3_tracing::init_tracing();
    hoover3_server::migrate_all().await.unwrap();
}
