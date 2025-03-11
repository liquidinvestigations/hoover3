//! Plan for computing file hashes for all files in all collections.
//! Splits the work into chunks of similar file size.

use hoover3_data_access::api::get_all_datasources;
use hoover3_database::client_query::collections::get_all_collections;
use hoover3_filesystem_scanner::tasks::hash_files_plan::compute_file_hash_plan;
use hoover3_tracing::init_tracing;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init_tracing();
    for collection in get_all_collections(()).await? {
        println!("COLLECTION {}", collection.collection_id);
        for datasource in get_all_datasources(collection.collection_id.clone()).await? {
            println!(
                "DATASOURCE {} / {}",
                collection.collection_id, datasource.datasource_id
            );
            compute_file_hash_plan((
                collection.collection_id.clone(),
                datasource.datasource_id.clone(),
            ))
            .await?;
        }
    }
    Ok(())
}

// fn reorder_stream<Value, SortKey>(
//     stream: ResultStream<Value>,
//     sort_key: impl Fn(&Value) -> SortKey + Send + Sync + 'static,
//     buffer_size: usize,
// ) -> ResultStream<Value>
// where
//     Value: Send + 'static,
//     SortKey: Ord + Eq + Send + 'static,
// {
//     struct OrdItem<SortKey, Value>(SortKey, Value);

//     impl<SortKey: Ord + Eq, Value> PartialOrd for OrdItem<SortKey, Value> {
//         fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//             self.0.partial_cmp(&other.0).map(|o| o.reverse())
//         }
//     }
//     impl<SortKey: Ord + Eq, Value> Ord for OrdItem<SortKey, Value> {
//         fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//             self.0.cmp(&other.0).reverse()
//         }
//     }
//     impl<SortKey: Ord + Eq, Value> PartialEq for OrdItem<SortKey, Value> {
//         fn eq(&self, other: &Self) -> bool {
//             self.0 == other.0
//         }
//     }
//     impl<SortKey: Ord + Eq, Value> Eq for OrdItem<SortKey, Value> {}

//     let s = try_stream! {
//         let mut buffer = std::collections::BinaryHeap::new();

//         let stream = stream.map_ok(|item| {
//             let key = sort_key(&item);
//             OrdItem(key, item)
//         });
//         pin_mut!(stream);
//         while let Some(item) = stream.next().await {
//             let item = item.context("reorder stream error:")?;
//             buffer.push(item);
//             if buffer.len() > buffer_size {
//                 let item = buffer.pop().unwrap();
//                 yield item.1;
//             }
//         }
//         while let Some(item) = buffer.pop() {
//             yield item.1;
//         }
//     };
//     Box::pin(s)
// }
