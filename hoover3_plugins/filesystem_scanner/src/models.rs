//! This module contains the table definitions for all the filesystem related models.

#![allow(missing_docs)]

use std::path::PathBuf;

use hoover3_data_access::models::DatasourceDbRow;
use hoover3_database::declare_implicit_graph_edge;
use hoover3_database::declare_stored_graph_edge;
use hoover3_macro::model;
use hoover3_macro::udt_model;
use hoover3_taskdef::anyhow;
use hoover3_types::filesystem::FsScanDatasourceDirsResult;
use hoover3_types::filesystem::{FsDirectoryUiRow, FsFileUiRow, FsMetadataBasic};
use hoover3_types::identifier::DatabaseIdentifier;

/// Scylla User Defined Type for the result of a directory scan.
#[udt_model]
#[allow(non_camel_case_types)]
pub struct fs_directory_scan_result {
    /// Number of files in the directory
    pub file_count: i32,
    /// Number of subdirectories in the directory
    pub dir_count: i32,
    /// Total size of all files in the directory
    pub file_size_bytes: i64,
    /// Number of errors encountered during the scan
    pub errors: i32,
}

impl Default for fs_directory_scan_result {
    fn default() -> Self {
        Self {
            file_count: 0,
            dir_count: 0,
            file_size_bytes: 0,
            errors: 0,
        }
    }
}

impl From<FsScanDatasourceDirsResult> for fs_directory_scan_result {
    fn from(value: FsScanDatasourceDirsResult) -> Self {
        Self {
            file_count: value.file_count as i32,
            dir_count: value.dir_count as i32,
            file_size_bytes: value.file_size_bytes as i64,
            errors: value.errors as i32,
        }
    }
}

impl From<fs_directory_scan_result> for FsScanDatasourceDirsResult {
    fn from(value: fs_directory_scan_result) -> Self {
        Self {
            file_count: value.file_count as u64,
            dir_count: value.dir_count as u64,
            file_size_bytes: value.file_size_bytes as u64,
            errors: value.errors as u64,
        }
    }
}

/// Database representation of a filesystem directory, as it is found on disk or S3.
#[model]
pub struct FsDirectoryDbRow {
    /// Unique identifier for the datasource
    #[model(primary(partition))]
    #[model(search(facet))]
    pub datasource_id: String,

    /// Path to the directory
    #[model(primary(clustering))]
    #[model(search(index))]
    pub path: String,

    /// Size of the directory in bytes
    #[model(search(facet))]
    pub size_bytes: i64,

    /// Timestamp of the most recent modification to the directory
    #[model(search(facet))]
    pub fs_modified: Option<Timestamp>,

    /// Timestamp of the directory's creation
    #[model(search(facet))]
    pub fs_created: Option<Timestamp>,

    /// Scan results for the directory's direct children
    #[model(search(facet))]
    pub scan_children: fs_directory_scan_result,

    /// Scan results for the directory's total contents, including all descendants
    #[model(search(facet))]
    pub scan_total: fs_directory_scan_result,
}

impl FsDirectoryDbRow {
    /// Convert a `FsDirectoryDbRow` to frontend representation.
    pub fn to_ui_row(self) -> anyhow::Result<FsDirectoryUiRow> {
        Ok(FsDirectoryUiRow {
            datasource_id: DatabaseIdentifier::new(&self.datasource_id)?,
            path: self.path.as_str().into(),
            size_bytes: self.size_bytes as u64,
            modified: self.fs_modified,
            created: self.fs_created,
            scan_children: self.scan_children.into(),
            scan_total: self.scan_total.into(),
        })
    }
    /// Create a `FsDirectoryDbRow` from a `FsMetadataBasic` which comes from a datasource scan.
    pub fn from_basic_meta(ds: &DatabaseIdentifier, meta: &FsMetadataBasic) -> Self {
        assert!(meta.is_dir);
        assert!(!meta.is_file);
        Self {
            datasource_id: ds.to_string(),
            path: meta.path.to_str().unwrap().into(),
            size_bytes: meta.size_bytes as i64,
            fs_modified: meta.modified,
            fs_created: meta.created,
            scan_children: Default::default(),
            scan_total: Default::default(),
        }
    }
}

/// Database representation of a filesystem file, as it is found on disk or S3.
#[model]
pub struct FsFileDbRow {
    /// Unique identifier for the datasource
    #[model(primary(partition))]
    #[model(search(facet))]
    pub datasource_id: String,

    /// Path to the file
    #[model(primary(partition))]
    #[model(search(index))]
    pub parent_dir_path: String,

    /// Name of the file
    #[model(primary(clustering))]
    #[model(search(index))]
    pub file_name: String,

    /// Size of the file in bytes
    #[model(search(facet))]
    pub size_bytes: i64,

    /// Timestamp of the file's last modification
    #[model(search(facet))]
    pub fs_modified: Option<Timestamp>,

    /// Timestamp of the file's creation
    #[model(search(facet))]
    pub fs_created: Option<Timestamp>,
}

impl FsFileDbRow {
    /// Convert a `FsFileDbRow` to frontend representation.
    pub fn to_ui_row(self) -> anyhow::Result<FsFileUiRow> {
        Ok(FsFileUiRow {
            datasource_id: DatabaseIdentifier::new(&self.datasource_id)?,
            path: PathBuf::from(self.parent_dir_path).join(self.file_name.as_str()),
            size_bytes: self.size_bytes as u64,
            modified: self.fs_modified,
            created: self.fs_created,
        })
    }
    /// Create a `FsFileDbRow` from a `FsMetadataBasic` which comes from a datasource scan.
    pub fn from_basic_meta(ds: &DatabaseIdentifier, meta: &FsMetadataBasic) -> Self {
        assert!(!meta.is_dir);
        assert!(meta.is_file);
        Self {
            datasource_id: ds.to_string(),
            parent_dir_path: meta.path.parent().unwrap().to_str().unwrap().into(),
            file_name: meta.path.file_name().unwrap().to_str().unwrap().into(),
            size_bytes: meta.size_bytes as i64,
            fs_modified: meta.modified,
            fs_created: meta.created,
        }
    }
}

declare_implicit_graph_edge!(
    FsDatasourceToDirectory,
    "fs_directory_datasource",
    DatasourceDbRow,
    FsDirectoryDbRow
);
declare_implicit_graph_edge!(
    FsDirectoryToFile,
    "fs_directory_file",
    FsDirectoryDbRow,
    FsFileDbRow
);

/// Model for storing the different types of hashes for a blob.
#[model]
pub struct FsBlobHashesDbRow {
    /// The SHA3-256 hash of the blob.
    #[model(primary(partition))]
    #[model(search(index))]
    pub blob_sha3_256: String,

    /// The SHA256 hash of the blob.
    #[model(search(index))]
    pub blob_sha256: String,

    /// The md5 hash of the blob.
    #[model(search(index))]
    pub blob_md5: String,

    /// The sha1 hash of the blob.
    #[model(search(index))]
    pub blob_sha1: String,

    /// The size of the blob in bytes.
    #[model(search(facet))]
    pub size_bytes: i64,

    /// Unique identifier for the datasource where this was first found
    pub datasource_id: String,

    /// Path to the file where this was first found
    pub parent_dir_path: String,

    /// Name of the file where this was first found
    pub file_name: String,
}

/// Model for storing the different types of hashes for a blob.
#[model]
pub struct FsBlobPlanPageDbRow {
    /// The sha3-256 hash of the blob.
    #[model(primary(partition))]
    pub blob_sha3_256: String,
    /// The plan page id.
    pub plan_page_id: i32,
}

/// Model for storing the mime type of a blob.
#[model]
pub struct FsBlobMimeTypeDbRow {
    /// The sha3-256 hash of the blob.
    #[model(primary(partition))]
    pub blob_sha3_256: String,
    /// The mime type of the blob, from libmagic.
    pub magic_mime: String,
    /// The mime type of the blob, from magika, from rules.
    pub magika_ruled_mime: Option<String>,
    /// The mime type of the blob, from magika, from deep learning.
    pub magika_inferred_mime: Option<String>,
    /// The score of the magika mime type.
    pub magika_score: Option<f32>,
    /// The mime type of the blob, from tika.
    pub tika_mime: String,
    /// Tika metadata extraction was successful.
    pub tika_metadata_success: bool,
    /// Tika Content extraction was successful.
    pub tika_content_success: bool,
}

declare_stored_graph_edge!(
    FsFileToHashes,
    "fs_file_hashes",
    FsFileDbRow,
    FsBlobHashesDbRow
);

/// Model for listing the pages of processing plans.
#[model]
pub struct BlobProcessingPlan {
    /// Plan page id.
    #[model(primary(partition))]
    pub plan_page_id: i32,
    /// The number of files that will be processed in this plan page.
    pub file_count: i32,
    /// The number of bytes that will be processed in this plan page.
    pub size_bytes: i64,
    /// Whether the plan has been started.
    pub is_started: bool,
}

/// Model for storing a page of processing plans.
#[model]
pub struct BlobProcessingPlanPageBlobs {
    /// Plan page id.
    #[model(primary(partition))]
    pub plan_page_id: i32,
    /// The sha3-256 hash of the blob.
    #[model(primary(clustering))]
    pub blob_sha3_256: String,
}

declare_implicit_graph_edge!(
    BlobProcessingPlanToPage,
    "blob_processing_plan_to_page",
    BlobProcessingPlan,
    BlobProcessingPlanPageBlobs
);

/// Model for storing the pages for the plans for hashing files.
/// Actual plan data is stored in the `FsFileHashPlanDbRow` model.
#[model]
pub struct FsFileHashPlanPageDbRow {
    /// The unique identifier for the datasource
    #[model(primary(partition))]
    pub datasource_id: String,
    /// The unique identifier for the plan chunk
    #[model(primary(clustering))]
    pub plan_chunk_id: i32,
}

declare_implicit_graph_edge!(
    FsDatasourceToFileHashPlanPages,
    "fs_datasource_to_file_hash_plan_pages",
    DatasourceDbRow,
    FsFileHashPlanPageDbRow
);

/// Model for storing the plan for hashing a single chunk of files.
#[model]
pub struct FsFileHashPlanDbRow {
    /// The unique identifier for the datasource
    #[model(primary(partition))]
    pub datasource_id: String,
    /// The unique identifier for the plan chunk
    #[model(primary(partition))]
    pub plan_chunk_id: i32,
    /// The actual plan data, json encoded
    pub plan_data: String,
}

declare_implicit_graph_edge!(
    FsFileHashPlanPageToPlans,
    "fs_file_hash_page_to_plans",
    FsFileHashPlanPageDbRow,
    FsFileHashPlanDbRow
);
