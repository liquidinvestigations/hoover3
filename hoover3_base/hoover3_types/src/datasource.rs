//! Types and structures related to datasources - scan / mount settings.

use std::path::PathBuf;

use crate::identifier::{CollectionId, DatabaseIdentifier};

/// Represents a row in the datasource UI table with configuration and metadata
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct DatasourceUiRow {
    /// Unique identifier of the collection this datasource belongs to
    pub collection_id: CollectionId,
    /// Unique identifier of the datasource
    pub datasource_id: DatabaseIdentifier,
    /// String representation of the datasource type
    pub datasource_type: String,
    /// Configuration settings specific to this datasource
    pub datasource_settings: DatasourceSettings,
    /// Timestamp when the datasource was created
    pub time_created: chrono::DateTime<chrono::Utc>,
    /// Timestamp when the datasource was last modified
    pub time_modified: chrono::DateTime<chrono::Utc>,
}

/// Configuration settings for different types of datasources - S3, WebDAV, LocalDisk, etc.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum DatasourceSettings {
    /// Local filesystem datasource configuration
    LocalDisk {
        /// Path to the local directory
        path: PathBuf,
    },
    /// Amazon S3 or compatible storage configuration
    S3 {
        /// Endpoint URL for the S3 service
        url: String,
        /// Name of the S3 bucket
        bucket: String,
        /// Access key for authentication
        access_key: String,
        /// Secret key for authentication
        secret_key: String,
        /// Path prefix within the bucket
        path: PathBuf,
    },
    /// WebDAV server configuration
    WebDav {
        /// URL of the WebDAV server
        url: String,
        /// Username for authentication
        username: String,
        /// Password for authentication
        password: String,
        /// Path on the WebDAV server
        path: PathBuf,
    },
}

impl DatasourceSettings {
    /// Returns a string representation of the datasource type
    pub fn type_str(&self) -> String {
        match self {
            DatasourceSettings::LocalDisk { .. } => "LocalDisk".to_string(),
            DatasourceSettings::S3 { .. } => "S3".to_string(),
            DatasourceSettings::WebDav { .. } => "WebDav".to_string(),
        }
    }
}
