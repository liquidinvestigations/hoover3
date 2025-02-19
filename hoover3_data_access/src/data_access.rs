pub mod file_system_access;
use anyhow::{Context, Result};

use hoover3_types::filesystem::FsMetadata;
use hoover3_types::data_access::DataAccessSettings;
use opendal::layers::LoggingLayer;
use opendal::services;
use opendal::Operator;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};


#[derive(
    Debug, Clone
)]
pub enum DataAccessOperator {
    FileSystemOperator(FileSystemOperator),
    S3Operator(S3Operator),
    WebDavOperator(WebDavOperator),
}

impl DataAccessOperator {
    pub async fn list_directory(&self, path: PathBuf) -> Result<Vec<FsMetadata>> {
        match self {
            DataAccessOperator::FileSystemOperator(operator) => operator.list_directory(path).await,
            DataAccessOperator::S3Operator(..) => Err(anyhow::anyhow!("S3 not implemented")),
            DataAccessOperator::WebDavOperator(..) => {
                Err(anyhow::anyhow!("WebDav not implemented"))
            }
        }
    }
}

#[derive(
    Debug, Clone,
)]
pub struct FileSystemOperator {
    operator: Operator,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct S3Operator {}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct WebDavOperator {}

impl FileSystemOperator {
    pub async fn list_directory(&self, path: PathBuf) -> Result<Vec<FsMetadata>> {
        let path_string = path.to_str().context("non-utf8 filename")?.to_string();
        let list = self.operator.list(&path_string).await?;
        let mut entries = Vec::new();
        let mut entries_iter = list.iter();
        entries_iter.next();
        for entry in entries_iter {
            match self.get_path_metadata(entry.path()).await {
                Ok(metadata) => {
                    entries.push(metadata);
                }
                Err(e) => {
                    eprintln!("error: {:?}", e);
                }
            }
        }

        // Sort entries: directories first, then files, both sorted by name
        entries.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                // If both are directories or both are files, sort by name
                (true, true) | (false, false) => a.path.cmp(&b.path),
                // If a is a directory and b is not, a comes first
                (true, false) => std::cmp::Ordering::Less,
                // If b is a directory and a is not, b comes first
                (false, true) => std::cmp::Ordering::Greater,
            }
        });

        Ok(entries)
    }

    pub async fn get_path_metadata(&self, relative_path: &str) -> Result<FsMetadata> {
        let relative_path_buf = PathBuf::from(relative_path);
        let metadata = self.operator.stat(relative_path).await?;
        Ok(FsMetadata {
            is_dir: metadata.is_dir(),
            is_file: metadata.is_file(),
            size_bytes: metadata.content_length(),
            modified: metadata.last_modified(),
            created: Option::None,
            path: relative_path_buf,
            path_string: relative_path.to_string(),
        })
    }
}

pub async fn create_operator(
    data_access_settings: &DataAccessSettings,
) -> Result<DataAccessOperator> {
    match data_access_settings {
        DataAccessSettings::LocalDisk { root_path } => {
            let root_path = root_path.clone()
                .into_os_string()
                .into_string()
                .map_err(|os_str| anyhow::anyhow!("non-utf8 path: {:?}", os_str))?;
            let builder = services::Fs::default().root(&root_path);
            let operator = Operator::new(builder)?
                .layer(LoggingLayer::default())
                .finish();
            Ok(DataAccessOperator::FileSystemOperator(FileSystemOperator {
                operator,
            }))
        }
        DataAccessSettings::S3 {
            url: _,
            bucket: _,
            access_key: _,
            secret_key: _,
        } => Err(anyhow::anyhow!("S3 not implemented")),
        DataAccessSettings::WebDav {
            url: _,
            username: _,
            password: _,
        } => Err(anyhow::anyhow!("WebDav not implemented")),
        _ => Err(anyhow::anyhow!("Unknown data access settings")),
    }
}

// TODO use enum for data backend type instead of passing the settings
// fetch the settings from the database here or in the create_operator function
pub async fn list_directory_server((data_access_settings, path): (DataAccessSettings, PathBuf)) -> Result<Vec<FsMetadata>> {
    let operator = create_operator(&data_access_settings).await?;
    operator.list_directory(path).await
}
