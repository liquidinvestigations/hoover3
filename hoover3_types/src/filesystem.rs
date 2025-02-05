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
    pub scan_children: FsScanDatasourceResult,
    /// Scan results for all descendants
    pub scan_total: FsScanDatasourceResult,
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
pub struct FsScanDatasourceResult {
    /// Number of files found
    pub file_count: u64,
    /// Number of directories found
    pub dir_count: u64,
    /// Total size of all files in bytes
    pub file_size_bytes: u64,
    /// Number of errors encountered
    pub errors: u64,
}

impl std::ops::Add<FsScanDatasourceResult> for FsScanDatasourceResult {
    type Output = FsScanDatasourceResult;
    /// Adds two scan results together
    fn add(self, rhs: FsScanDatasourceResult) -> Self::Output {
        FsScanDatasourceResult {
            file_count: self.file_count + rhs.file_count,
            dir_count: self.dir_count + rhs.dir_count,
            file_size_bytes: self.file_size_bytes + rhs.file_size_bytes,
            errors: self.errors + rhs.errors,
        }
    }
}

impl std::ops::AddAssign<FsScanDatasourceResult> for FsScanDatasourceResult {
    /// Adds another scan result to this one in place
    fn add_assign(&mut self, rhs: FsScanDatasourceResult) {
        *self = *self + rhs;
    }
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
