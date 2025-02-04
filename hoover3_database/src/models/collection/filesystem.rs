use crate::impl_model_callbacks;
use charybdis::macros::charybdis_model;
use charybdis::macros::charybdis_udt_model;
use charybdis::types::{BigInt, Int, Text, Timestamp};
use hoover3_types::filesystem::FsScanDatasourceResult;
use hoover3_types::filesystem::{FsDirectoryUiRow, FsFileUiRow, FsMetadataBasic};
use hoover3_types::identifier::DatabaseIdentifier;

#[charybdis_udt_model(type_name = FsDirectoryScanResultDb)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Default)]
pub struct FsDirectoryScanResultDb {
    pub file_count: Int,
    pub dir_count: Int,
    pub file_size_bytes: BigInt,
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

use serde::Serialize;
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
    pub datasource_id: Text,
    pub path: Text,
    pub size_bytes: BigInt,
    pub modified: Option<Timestamp>,
    pub created: Option<Timestamp>,

    pub scan_children: FsDirectoryScanResultDb,
    pub scan_total: FsDirectoryScanResultDb,
}

impl FsDirectoryDbRow {
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
    pub datasource_id: Text,
    pub path: Text,
    pub size_bytes: BigInt,
    pub modified: Option<Timestamp>,
    pub created: Option<Timestamp>,
}
impl FsFileDbRow {
    pub fn to_ui_row(self) -> anyhow::Result<FsFileUiRow> {
        Ok(FsFileUiRow {
            datasource_id: DatabaseIdentifier::new(&self.datasource_id)?,
            path: self.path.as_str().into(),
            size_bytes: self.size_bytes as u64,
            modified: self.modified,
            created: self.created,
        })
    }
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
