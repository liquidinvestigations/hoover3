//! This module contains the table definitions for all the filesystem related models.

use crate::impl_model_callbacks;
use charybdis::macros::charybdis_model;
use charybdis::macros::charybdis_udt_model;
use charybdis::types::{BigInt, Int, Text, Timestamp};
use hoover3_types::filesystem::FsScanDatasourceResult;
use hoover3_types::filesystem::{FsDirectoryUiRow, FsFileUiRow, FsMetadataBasic};
use hoover3_types::identifier::DatabaseIdentifier;
use serde::Serialize;

/// Scylla User Defined Type for the result of a directory scan.
#[charybdis_udt_model(type_name = FsDirectoryScanResultDb)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Default)]
pub struct FsDirectoryScanResultDb {
    /// Number of files in the directory
    pub file_count: Int,
    /// Number of subdirectories in the directory
    pub dir_count: Int,
    /// Total size of all files in the directory
    pub file_size_bytes: BigInt,
    /// Number of errors encountered during the scan
    pub errors: Int,
}

impl From<FsScanDatasourceResult> for FsDirectoryScanResultDb {
    fn from(value: FsScanDatasourceResult) -> Self {
        Self {
            file_count: value.file_count as i32,
            dir_count: value.dir_count as i32,
            file_size_bytes: value.file_size_bytes as i64,
            errors: value.errors as i32,
        }
    }
}

impl From<FsDirectoryScanResultDb> for FsScanDatasourceResult {
    fn from(value: FsDirectoryScanResultDb) -> Self {
        Self {
            file_count: value.file_count as u64,
            dir_count: value.dir_count as u64,
            file_size_bytes: value.file_size_bytes as u64,
            errors: value.errors as u64,
        }
    }
}

/// Database representation of a filesystem directory, as it is found on disk or S3.
#[charybdis_model(
    table_name = filesystem_directory,
    partition_keys = [datasource_id, path],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Default)]
pub struct FsDirectoryDbRow {
    /// Unique identifier for the datasource
    pub datasource_id: Text,
    /// Path to the directory
    pub path: Text,
    /// Size of the directory in bytes
    pub size_bytes: BigInt,
    /// Timestamp of the most recent modification to the directory
    pub modified: Option<Timestamp>,
    /// Timestamp of the directory's creation
    pub created: Option<Timestamp>,
    /// Scan results for the directory's direct children
    pub scan_children: FsDirectoryScanResultDb,
    /// Scan results for the directory's total contents, including all descendants
    pub scan_total: FsDirectoryScanResultDb,
}

impl FsDirectoryDbRow {
    /// Convert a `FsDirectoryDbRow` to frontend representation.
    pub fn to_ui_row(self) -> anyhow::Result<FsDirectoryUiRow> {
        Ok(FsDirectoryUiRow {
            datasource_id: DatabaseIdentifier::new(&self.datasource_id)?,
            path: self.path.as_str().into(),
            size_bytes: self.size_bytes as u64,
            modified: self.modified,
            created: self.created,
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
            modified: meta.modified,
            created: meta.created,
            scan_children: Default::default(),
            scan_total: Default::default(),
        }
    }
}

impl_model_callbacks!(FsDirectoryDbRow);

/// Database representation of a filesystem file, as it is found on disk or S3.
#[charybdis_model(
    table_name = filesystem_file,
    partition_keys = [datasource_id, path],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct FsFileDbRow {
    /// Unique identifier for the datasource
    pub datasource_id: Text,
    /// Path to the file
    pub path: Text,
    /// Size of the file in bytes
    pub size_bytes: BigInt,
    /// Timestamp of the file's last modification
    pub modified: Option<Timestamp>,
    /// Timestamp of the file's creation
    pub created: Option<Timestamp>,
}
impl FsFileDbRow {
    /// Convert a `FsFileDbRow` to frontend representation.
    pub fn to_ui_row(self) -> anyhow::Result<FsFileUiRow> {
        Ok(FsFileUiRow {
            datasource_id: DatabaseIdentifier::new(&self.datasource_id)?,
            path: self.path.as_str().into(),
            size_bytes: self.size_bytes as u64,
            modified: self.modified,
            created: self.created,
        })
    }
    /// Create a `FsFileDbRow` from a `FsMetadataBasic` which comes from a datasource scan.
    pub fn from_basic_meta(ds: &DatabaseIdentifier, meta: &FsMetadataBasic) -> Self {
        assert!(!meta.is_dir);
        assert!(meta.is_file);
        Self {
            datasource_id: ds.to_string(),
            path: meta.path.to_str().unwrap().into(),
            size_bytes: meta.size_bytes as i64,
            modified: meta.modified,
            created: meta.created,
        }
    }
}

impl_model_callbacks!(FsFileDbRow);
