//! Plan for computing file hashes for all files in all collections.
//! Splits the work into chunks of similar file size.

use charybdis::operations::Find;
use futures::{pin_mut, StreamExt, TryStreamExt};
use hoover3_data_access::api::get_all_datasources;
use hoover3_database::models::collection::GraphEdgeQuery;
use hoover3_database::{
    client_query::collections::get_all_collections,
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
    models::collection::GraphEdge,
};

use hoover3_filesystem_scanner::models::{FsDatasourceToDirectory, FsDirectoryToFile, FsFileDbRow};
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    for collection in get_all_collections(()).await? {
        println!("COLLECTION {}", collection.collection_id);
        for datasource in get_all_datasources(collection.collection_id.clone()).await? {
            println!(
                "DATASOURCE {} / {}",
                collection.collection_id, datasource.datasource_id
            );
            compute_file_hash_plan(
                collection.collection_id.clone(),
                datasource.datasource_id.clone(),
            )
            .await?;
        }
    }
    Ok(())
}

async fn compute_file_hash_plan(
    collection_id: CollectionId,
    datasource_id: DatabaseIdentifier,
) -> Result<(), anyhow::Error> {
    println!("COMPUTE PLAN");
    let session = ScyllaDatabaseHandle::collection_session(&collection_id).await?;
    let dir_stream =
        FsDatasourceToDirectory::list_target_pks(&collection_id, &(datasource_id.to_string(),))
            .await?;
    pin_mut!(dir_stream);

    let mut x = 0;
    while let (Some(Ok(dir))) = dir_stream.next().await {
        let file_stream = FsDirectoryToFile::list_target_pks(&collection_id, &dir).await?;
        pin_mut!(file_stream);
        while let (Some(Ok(file))) = file_stream.next().await {
            println!("{:?}", file);
            x += 1;
        }
    }

    println!("total: {}", x);

    Ok(())
}
