//! CLI tool to migrate all databases;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let providers = hoover3_tracing::init_tracing().await;
    info!("migrate all databases");
    hoover3_server::init_server_plugins()?;
    hoover3_server::migrate_all().await?;
    if let Some((log_provider, trace_provider)) = providers {
        let _ = log_provider.shutdown();
        let _ = trace_provider.shutdown();
    }
    Ok(())
}
