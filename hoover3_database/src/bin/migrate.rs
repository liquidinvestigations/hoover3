use anyhow::Result;
use hoover3_database::tracing::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    hoover3_database::migrate::migrate_all().await
}
