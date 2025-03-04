//! CLI tool to run a worker;

fn main() -> anyhow::Result<()> {
    hoover3_tracing::init_tracing();
    hoover3_server::init_server_plugins()?;
    hoover3_taskdef::tasks::run_worker(hoover3_filesystem_scanner::tasks::FilesystemScannerQueue)?;
    Ok(())
}
