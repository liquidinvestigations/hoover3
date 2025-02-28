//! This module contains the table definitions for all the filesystem related models.

use crate::impl_model_callbacks;
use hoover3_macro::model;
use hoover3_macro::udt_model;
use hoover3_types::filesystem::FsScanDatasourceResult;
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

impl From<FsScanDatasourceResult> for fs_directory_scan_result {
    fn from(value: FsScanDatasourceResult) -> Self {
        Self {
            file_count: value.file_count as i32,
            dir_count: value.dir_count as i32,
            file_size_bytes: value.file_size_bytes as i64,
            errors: value.errors as i32,
        }
    }
}

impl From<fs_directory_scan_result> for FsScanDatasourceResult {
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
    pub datasource_id: String,
    /// Path to the directory
    #[model(primary(clustering))]
    pub path: String,
    /// Size of the directory in bytes
    pub size_bytes: i64,
    /// Timestamp of the most recent modification to the directory
    pub modified: Option<Timestamp>,
    /// Timestamp of the directory's creation
    pub created: Option<Timestamp>,
    /// Scan results for the directory's direct children
    pub scan_children: fs_directory_scan_result,
    /// Scan results for the directory's total contents, including all descendants
    pub scan_total: fs_directory_scan_result,
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

/// Database representation of a filesystem file, as it is found on disk or S3.
#[model]
pub struct FsFileDbRow {
    /// Unique identifier for the datasource
    #[model(primary(partition))]
    pub datasource_id: String,
    /// Path to the file
    #[model(primary(partition))]
    pub path: String,
    /// Size of the file in bytes
    pub size_bytes: i64,
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
