use std::str::FromStr;
use std::sync::Arc;
pub use temporal_client::{Client, RetryClient};
use temporal_sdk::sdk_client_options;
use temporal_sdk_core::Url;
use tokio::sync::OnceCell;
use tracing::info;

/// Temporal client wrapper
pub type TemporalioClient = Arc<RetryClient<Client>>;

/// Get a Temporal client from global client cache.
pub async fn get_client() -> Result<TemporalioClient, anyhow::Error> {
    static CELL: OnceCell<TemporalioClient> = OnceCell::const_new();
    let client = CELL.get_or_init(|| async move {
        let url = Url::from_str("http://localhost:7233").unwrap();
        let server_options = sdk_client_options(url).build().unwrap();
        let client = Arc::new(server_options.connect("default", None).await.unwrap());
        info!("temporalio client started.");
        client
    });

    Ok(client.await.clone())
}
