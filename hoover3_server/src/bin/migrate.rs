//! CLI tool to migrate all databases;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hoover3_tracing::init_tracing();
    hoover3_server::init_server_plugins()?;
    hoover3_server::migrate_all().await?;
    Ok(())
}
