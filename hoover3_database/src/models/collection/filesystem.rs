use charybdis::macros::charybdis_model;
use charybdis::types::{Text, Timestamp, BigInt};
use hoover3_types::datasource::DatasourceUiRow;
use hoover3_types::identifier::CollectionId;
use hoover3_types::filesystem::FsMetadata;
use hoover3_types::identifier::DatabaseIdentifier;

#[charybdis_model(
    table_name = filesystem_directory,
    partition_keys = [datasource_id, path],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FsDirectoryDbRow {
    pub datasource_id: Text,
    pub path: Text,
    pub size_bytes: BigInt,
    pub modified: Option<Timestamp>,
    pub created: Option<Timestamp>,
}
impl FsDirectoryDbRow {
    pub fn to_ui_row(self) -> FsMetadata {
        FsMetadata {
            is_dir: true,
            is_file: false,
            size_bytes: self.size_bytes as u64,
            modified: self.modified,
            created: self.created,
            path: self.path.as_str().into(),
        }
    }
    pub fn from_meta(ds: &DatabaseIdentifier, meta: &FsMetadata) -> Self {
        assert!(meta.is_dir);
        assert!(!meta.is_file);
        Self {
            datasource_id: ds.to_string(),
            path: meta.path.to_str().unwrap().into(),
            size_bytes: meta.size_bytes as i64,
            modified: meta.modified,
            created: meta.created,
        }
    }
}


#[charybdis_model(
    table_name = filesystem_file,
    partition_keys = [datasource_id, path],
    clustering_keys = [],
    global_secondary_indexes = [],
    local_secondary_indexes = [],
    static_columns = []
)]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct FsFileDbRow {
    pub datasource_id: Text,
    pub path: Text,
    pub size_bytes: BigInt,
    pub modified: Option<Timestamp>,
    pub created: Option<Timestamp>,
}
impl FsFileDbRow {
    pub fn to_ui_row(self) -> FsMetadata {
        FsMetadata {
            is_dir: false,
            is_file: true,
            size_bytes: self.size_bytes as u64,
            modified: self.modified,
            created: self.created,
            path: self.path.as_str().into(),
        }
    }
    pub fn from_meta(ds: &DatabaseIdentifier, meta: &FsMetadata) -> Self {
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