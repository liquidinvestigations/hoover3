//! Hash files task - go over filesystem file rows and read the file content once.
//! Hash the file content using different algorithms.
//! Also run libmagic on the files to get mime type.
//! Save the results to the database in [FsBlobHashesDbRow].

use charybdis::{batch::ModelBatch, model::BaseModel};
use futures::{pin_mut, StreamExt};
use hoover3_database::{
    client_query::list_disk::read_file_to_stream,
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
    models::collection::{DatabaseExtraCallbacks, GraphEdgeInsert},
};
use hoover3_macro::{activity, workflow};
use hoover3_taskdef::{
    TemporalioActivityDescriptor, TemporalioWorkflowDescriptor, WfContext, WfExitValue,
    WorkflowResult,
};
use hoover3_types::{
    datasource::DatasourceSettings,
    filesystem::FsScanHashesResult,
    identifier::{CollectionId, DatabaseIdentifier},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use super::{
    hash_files_plan::{compute_file_hash_plan_activity, FileHashPlanChunk},
    FilesystemScannerQueue,
};
use crate::models::{FsBlobHashesDbRow, FsFileHashPlanDbRow, FsFileToHashes};

/// Argument for hashing a file
#[derive(Clone, Serialize, Deserialize)]
pub struct HashFileArgs {
    /// Collection to hash the file from
    pub collection_id: CollectionId,
    /// Datasource to hash the file from
    pub datasource_id: DatabaseIdentifier,
    /// Plan chunk id
    pub plan_chunk_id: i32,
}

trait HashFunction {
    fn h_update(&mut self, data: &[u8]);
    fn h_finalize(&self) -> String;
    fn h_type(&self) -> HashType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HashType {
    Sha3_256,
    Sha1,
    Sha256,
    Md5,
}

fn to_hex(b: &[u8]) -> String {
    let hex: String = b
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("");
    hex
}
mod sha3_impl {
    use super::*;
    use sha3::Digest;
    impl super::HashFunction for sha3::Sha3_256 {
        fn h_update(&mut self, data: &[u8]) {
            self.update(data);
        }
        fn h_finalize(&self) -> String {
            to_hex(&(self.clone()).finalize())
        }
        fn h_type(&self) -> HashType {
            HashType::Sha3_256
        }
    }
    pub fn sha3_256_new() -> sha3::Sha3_256 {
        sha3::Sha3_256::new()
    }
}

mod sha1_impl {
    use super::*;
    use sha1::{Digest, Sha1};
    impl super::HashFunction for Sha1 {
        fn h_update(&mut self, data: &[u8]) {
            self.update(data);
        }
        fn h_finalize(&self) -> String {
            to_hex(&(self.clone()).finalize())
        }
        fn h_type(&self) -> HashType {
            HashType::Sha1
        }
    }
    pub fn sha1_new() -> Sha1 {
        Sha1::new()
    }
}

mod sha256_impl {
    use super::*;
    use sha2::{Digest, Sha256};
    impl super::HashFunction for Sha256 {
        fn h_update(&mut self, data: &[u8]) {
            self.update(data);
        }
        fn h_finalize(&self) -> String {
            to_hex(&(self.clone()).finalize())
        }
        fn h_type(&self) -> HashType {
            HashType::Sha256
        }
    }
    pub fn sha256_new() -> Sha256 {
        Sha256::new()
    }
}

mod md5_impl {
    use super::*;
    use md5::Context;
    impl super::HashFunction for Context {
        fn h_update(&mut self, data: &[u8]) {
            self.consume(data);
        }
        fn h_finalize(&self) -> String {
            to_hex(&(self.clone()).compute().0)
        }
        fn h_type(&self) -> HashType {
            HashType::Md5
        }
    }
    pub fn md5_new() -> Context {
        Context::new()
    }
}

/// Hash some fiels and save the results to the database in [FsBlobHashesDbRow].
/// The files are hashed by this function one after the other, so it should receive
/// batches of similar size (in file byte total)
#[activity(FilesystemScannerQueue)]
async fn fs_do_hash_files(args: HashFileArgs) -> anyhow::Result<FsScanHashesResult> {
    let mut new_hashes = vec![];

    let session = ScyllaDatabaseHandle::collection_session(&args.collection_id).await?;
    let plan_chunk = FsFileHashPlanDbRow::find_by_datasource_id_and_plan_chunk_id(
        args.datasource_id.to_string(),
        args.plan_chunk_id,
    )
    .execute(&session)
    .await?;
    // get plan data
    let plan_chunk_data = serde_json::from_str::<FileHashPlanChunk>(&plan_chunk.plan_data)?;
    let scan_file_args = plan_chunk_data
        .dirs
        .iter()
        .map(|(dir, files)| {
            files
                .iter()
                .map(|(file_name, file_size)| (dir.clone(), file_name.clone(), *file_size))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    let file_count = scan_file_args.len() as u64;

    // get datasource dir
    let ds_row = hoover3_data_access::api::get_datasource((
        args.collection_id.clone(),
        args.datasource_id.clone(),
    ))
    .await?;

    let DatasourceSettings::LocalDisk { path: root_path } = &ds_row.datasource_settings else {
        anyhow::bail!("Datasource is not a local disk");
    };
    let root_path = root_path.to_path_buf();
    drop(session);

    let mut edge_batch = FsFileToHashes::edge_batch(&args.collection_id);

    for (dir, file_name, plan_file_size) in scan_file_args {
        let file_path = root_path.join(&dir).join(&file_name);
        let (file_size, chunks) = read_file_to_stream(file_path).await?;
        assert_eq!(file_size as i64, plan_file_size);
        pin_mut!(chunks);
        let mut hash_functions: Vec<Arc<Mutex<dyn HashFunction + Send>>> = vec![
            Arc::new(Mutex::new(sha3_impl::sha3_256_new())),
            Arc::new(Mutex::new(sha1_impl::sha1_new())),
            Arc::new(Mutex::new(sha256_impl::sha256_new())),
            Arc::new(Mutex::new(md5_impl::md5_new())),
        ];

        while let Some(Ok(chunk)) = chunks.next().await {
            for hash_function in hash_functions.iter_mut() {
                hash_function.lock().await.h_update(&chunk);
            }
        }
        let mut finished_hashes = HashMap::new();
        for hash_function in hash_functions.into_iter() {
            let l = hash_function.lock().await;
            let v = l.h_finalize();
            let k = l.h_type();
            finished_hashes.insert(k, v);
        }
        let hashes_row = FsBlobHashesDbRow {
            blob_sha3_256: finished_hashes[&HashType::Sha3_256].clone(),
            blob_sha256: finished_hashes[&HashType::Sha256].clone(),
            blob_md5: finished_hashes[&HashType::Md5].clone(),
            blob_sha1: finished_hashes[&HashType::Sha1].clone(),
            size_bytes: file_size as i64,
        };
        new_hashes.push(hashes_row.clone());
        edge_batch.add_edge_from_pk(
            &(args.datasource_id.to_string(), dir, file_name),
            &hashes_row.primary_key_values(),
        );
    }
    edge_batch.execute().await?;
    let session = ScyllaDatabaseHandle::collection_session(&args.collection_id).await?;
    let mut batch = FsBlobHashesDbRow::batch();
    batch.append_inserts(&new_hashes);
    batch.execute(&session).await?;
    DatabaseExtraCallbacks::new(&args.collection_id)
        .await?
        .insert(&new_hashes)
        .await?;

    Ok(FsScanHashesResult {
        file_count,
        hash_count: new_hashes.len() as u64,
    })
}

/// Execute one hash file chunk.
#[workflow(FilesystemScannerQueue)]
async fn hash_files_one(ctx: WfContext, args: HashFileArgs) -> WorkflowResult<FsScanHashesResult> {
    Ok(WfExitValue::Normal(
        fs_do_hash_files_activity::run(&ctx, args).await?,
    ))
}

/// Hash a group of plan chunks in parallel in parallel.
#[workflow(FilesystemScannerQueue)]
async fn hash_files_group(
    ctx: WfContext,
    (collection_id, datasource_id, plan_chunk_ids): (CollectionId, DatabaseIdentifier, Vec<i32>),
) -> WorkflowResult<FsScanHashesResult> {
    let mut total = FsScanHashesResult::default();
    let args = plan_chunk_ids
        .into_iter()
        .map(|plan_chunk_id| HashFileArgs {
            collection_id: collection_id.clone(),
            datasource_id: datasource_id.clone(),
            plan_chunk_id,
        })
        .collect::<Vec<_>>();

    for (_arg, res) in hash_files_one_workflow::run_parallel(&ctx, args).await? {
        let res = res?;
        total += res;
    }
    Ok(WfExitValue::Normal(total))
}

/// Create a file hashing plan and execute it.
#[workflow(FilesystemScannerQueue)]
async fn hash_files_root(
    ctx: WfContext,
    (collection_id, datasource_id): (CollectionId, DatabaseIdentifier),
) -> WorkflowResult<FsScanHashesResult> {
    let plan =
        compute_file_hash_plan_activity::run(&ctx, (collection_id.clone(), datasource_id.clone()))
            .await?;
    let chunks = plan
        .chunks(300)
        .into_iter()
        .map(|v| (collection_id.clone(), datasource_id.clone(), v.to_vec()))
        .collect::<Vec<_>>();
    let mut total = FsScanHashesResult::default();
    for (_arg, _res) in hash_files_group_workflow::run_parallel(&ctx, chunks).await? {
        let _res = _res?;
        total += _res;
    }

    Ok(WfExitValue::Normal(total))
}
