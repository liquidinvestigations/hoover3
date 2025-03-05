//! Hash files task - go over filesystem file rows and read the file content once.
//! Hash the file content using different algorithms.
//! Also run libmagic on the files to get mime type.
//! Save the results to the database in [FsBlobHashesDbRow].

use charybdis::batch::ModelBatch;
use futures::{pin_mut, StreamExt};
use hoover3_database::{
    client_query::list_disk::read_file_to_stream,
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
    models::collection::DatabaseExtraCallbacks,
};
use hoover3_macro::activity;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

use super::FilesystemScannerQueue;
use crate::models::FsBlobHashesDbRow;

/// Argument for hashing a file
#[derive(Clone, Serialize, Deserialize)]
pub struct HashFileArgs {
    /// Collection to hash the file from
    pub collection_id: CollectionId,
    /// Datasource to hash the file from
    pub datasource_id: DatabaseIdentifier,
    /// File paths
    pub file_paths: Vec<PathBuf>,
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
async fn hash_files(scan_file_args: HashFileArgs) -> anyhow::Result<()> {
    let mut new_hashes = vec![];

    for file_path in scan_file_args.file_paths {
        let (file_size, chunks) = read_file_to_stream(file_path).await?;
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
        new_hashes.push(FsBlobHashesDbRow {
            blob_sha3_256: finished_hashes[&HashType::Sha3_256].clone(),
            blob_sha256: finished_hashes[&HashType::Sha256].clone(),
            blob_md5: finished_hashes[&HashType::Md5].clone(),
            blob_sha1: finished_hashes[&HashType::Sha1].clone(),
            size_bytes: file_size as i64,
        });
    }
    let mut batch = FsBlobHashesDbRow::batch();
    batch.append_inserts(&new_hashes);
    let session = ScyllaDatabaseHandle::collection_session(&scan_file_args.collection_id).await?;
    batch.execute(&session).await?;
    DatabaseExtraCallbacks::new(&scan_file_args.collection_id)
        .await?
        .insert(&new_hashes)
        .await?;

    Ok(())
}
