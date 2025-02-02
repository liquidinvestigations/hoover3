use anyhow::Result;
use hoover3_database::db_management::CollectionId;
use hoover3_tracing::init_tracing;
#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    let c = hoover3_database::client_query::collections::get_all_collections(()).await?;
    println!("{:?}", c);
    let c = CollectionId::new(&c[0].collection_id).unwrap();
    let s = hoover3_database::migrate::nebula_get_tags_schema(&c).await?;
    println!("{:#?}", s);
    Ok(())
}
