//! Types and structures related to filesystems, scanning.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::identifier::DatabaseIdentifier;

/// Basic metadata for a filesystem entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FsMetadataBasic {
    /// Whether the entry is a directory
    pub is_dir: bool,
    /// Whether the entry is a file
    pub is_file: bool,
    /// Size of the entry in bytes
    pub size_bytes: u64,
    /// Last modification timestamp
    pub modified: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created: Option<DateTime<Utc>>,
    /// Path to the filesystem entry
    pub path: PathBuf,
}

/// UI representation of a directory entry with scan results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FsDirectoryUiRow {
    /// ID of the datasource containing this directory
    pub datasource_id: DatabaseIdentifier,
    /// Path to the directory
    pub path: PathBuf,
    /// Size of the directory in bytes
    pub size_bytes: u64,
    /// Last modification timestamp
    pub modified: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created: Option<DateTime<Utc>>,
    /// Scan results for immediate children
    pub scan_children: FsScanDatasourceDirsResult,
    /// Scan results for all descendants
    pub scan_total: FsScanDatasourceDirsResult,
}

/// UI representation of a file entry
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FsFileUiRow {
    /// ID of the datasource containing this file
    pub datasource_id: DatabaseIdentifier,
    /// Path to the file
    pub path: PathBuf,
    /// Size of the file in bytes
    pub size_bytes: u64,
    /// Last modification timestamp
    pub modified: Option<DateTime<Utc>>,
    /// Creation timestamp
    pub created: Option<DateTime<Utc>>,
}

/// Results from scanning a datasource
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FsScanDatasourceDirsResult {
    /// Number of files found
    pub file_count: u64,
    /// Number of directories found
    pub dir_count: u64,
    /// Total size of all files in bytes
    pub file_size_bytes: u64,
    /// Number of errors encountered
    pub errors: u64,
}

impl std::ops::Add<FsScanDatasourceDirsResult> for FsScanDatasourceDirsResult {
    type Output = FsScanDatasourceDirsResult;
    /// Adds two scan results together
    fn add(self, rhs: FsScanDatasourceDirsResult) -> Self::Output {
        FsScanDatasourceDirsResult {
            file_count: self.file_count + rhs.file_count,
            dir_count: self.dir_count + rhs.dir_count,
            file_size_bytes: self.file_size_bytes + rhs.file_size_bytes,
            errors: self.errors + rhs.errors,
        }
    }
}

impl std::ops::AddAssign<FsScanDatasourceDirsResult> for FsScanDatasourceDirsResult {
    /// Adds another scan result to this one in place
    fn add_assign(&mut self, rhs: FsScanDatasourceDirsResult) {
        *self = *self + rhs;
    }
}

/// Results from counting distinct hashes
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FsScanHashesResult {
    /// Number of files found
    pub file_count: u64,
    /// Number of distinct hashes found
    pub hash_count: u64,
}

impl std::ops::Add<FsScanHashesResult> for FsScanHashesResult {
    type Output = FsScanHashesResult;
    /// Adds two scan results together
    fn add(self, rhs: FsScanHashesResult) -> Self::Output {
        FsScanHashesResult {
            file_count: self.file_count + rhs.file_count,
            hash_count: self.hash_count + rhs.hash_count,
        }
    }
}

impl std::ops::AddAssign<FsScanHashesResult> for FsScanHashesResult {
    /// Adds another scan result to this one in place
    fn add_assign(&mut self, rhs: FsScanHashesResult) {
        *self = *self + rhs;
    }
}

/// Results from all scanning tasks
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FsScanResult {
    /// Number of files found
    pub dir_scan_result: FsScanDatasourceDirsResult,
    /// Number of distinct hashes found
    pub hash_scan_result: FsScanHashesResult,
    /// Number of processing plan pages created for the blobs
    pub processing_plan_result: ProcessingPlanResult,
}

/// Results from chunking the unique blobs into processing plan pages
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ProcessingPlanResult {
    /// Total number of processing plan pages created
    pub new_page_count: u32,
    /// Total number of blobs in the processing plan
    pub total_blob_count: u32,
    /// Total size of the blobs in the processing plan
    pub total_blob_size_bytes: u64,
}



// This does not work on
// mod serialize_path {
//     use std::ffi::OsStr;
//     use std::os::unix::ffi::OsStrExt;

//     use super::*;
//     use serde::de::Deserializer;
//     use serde::ser::Serializer;

//     pub fn serialize<S>(p: &PathBuf, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         serializer.serialize_bytes(p.as_os_str().as_bytes())
//     }
//     pub fn deserialize<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let data = <&[u8]>::deserialize(deserializer)?;
//         Ok(OsStr::from_bytes(data).into())
//     }
// }
