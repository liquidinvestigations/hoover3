use std::path::PathBuf;

use crate::identifier::{CollectionId, DatabaseIdentifier};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct DatasourceUiRow {
    pub collection_id: CollectionId,
    pub datasource_id: DatabaseIdentifier,
    pub datasource_type: String,
    pub datasource_settings: DatasourceSettings,
    pub time_created: chrono::DateTime<chrono::Utc>,
    pub time_modified: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum DatasourceSettings {
    LocalDisk {
        path: PathBuf,
    },
    S3 {
        url: String,
        bucket: String,
        access_key: String,
        secret_key: String,
        path: PathBuf,
    },
    WebDav {
        url: String,
        username: String,
        password: String,
        path: PathBuf,
    },
}

impl DatasourceSettings {
    pub fn type_str(&self) -> String {
        match self {
            DatasourceSettings::LocalDisk { .. } => "LocalDisk".to_string(),
            DatasourceSettings::S3 { .. } => "S3".to_string(),
            DatasourceSettings::WebDav { .. } => "WebDav".to_string(),
        }
    }
}
