use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FsMetadata {
    pub is_dir: bool,
    pub is_file: bool,
    pub size_bytes: u64,
    pub modified: Option<DateTime<Utc>>,
    pub created: Option<DateTime<Utc>>,
    // #[serde(with = "serialize_path")]
    pub path: PathBuf,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct FsScanDatasourceResult {
    pub file_count: u64,
    pub dir_count: u64,
    pub file_size_bytes: u64,
    pub errors: u64,
}

impl std::ops::Add<FsScanDatasourceResult> for FsScanDatasourceResult {
    type Output = FsScanDatasourceResult;
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
