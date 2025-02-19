pub mod file_system_access;
use anyhow::{Context, Result};

use hoover3_types::filesystem::FsMetadata;
use hoover3_types::data_access::{LocalDiskSettings, DataBackend, WebDavSettings};
use opendal::layers::LoggingLayer;
use opendal::services;
use opendal::Operator;
use std::path::PathBuf;
use hoover3_database::client_query::data_access_settings::get_data_access_settings;
use serde::{Deserialize, Serialize};


#[derive(
    Debug, Clone
)]
pub enum DataAccessOperator {
    FileSystemOperator(FileSystemOperator),
    S3Operator(S3Operator),
    WebDavOperator(WebDavOperator),
}

impl DataAccess for DataAccessOperator {
    fn operator(&self) -> &Operator {
        match self {
            DataAccessOperator::FileSystemOperator(operator) => &operator.operator,
            DataAccessOperator::S3Operator(..) => unimplemented!(),
            DataAccessOperator::WebDavOperator(operator) => &operator.operator,
        }
    }
}

pub trait DataAccess {
    fn operator(&self) -> &Operator;

    async fn list_directory(&self, path: PathBuf) -> Result<Vec<FsMetadata>> {
        let path_string = path.to_str().context("non-utf8 filename")?.to_string();
        let list = self.operator().list(&path_string).await?;
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

    async fn get_path_metadata(&self, relative_path: &str) -> Result<FsMetadata> {
        let relative_path_buf = PathBuf::from(relative_path);
        let metadata = self.operator().stat(relative_path).await?;
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
    Debug, Clone
)]
pub struct WebDavOperator {
    operator: Operator
}

pub async fn create_operator(
    data_backend: &DataBackend,
) -> Result<DataAccessOperator> {

    let data_access_settings = get_data_access_settings(()).await?;

    match data_backend {
       DataBackend::LocalDisk  => {
            let local_disk_settings = &data_access_settings.local_disk;
            let operator = create_local_disk_operator(local_disk_settings)?;
            Ok(DataAccessOperator::FileSystemOperator(operator))
        }
        DataBackend::S3 => {
            Err(anyhow::anyhow!("S3 not implemented"))
        },
        DataBackend::WebDav => {
            let webdav_settings = &data_access_settings.webdav;
            let operator = create_webdav_operator(webdav_settings)?;
            Ok(DataAccessOperator::WebDavOperator(operator))
        }
    }
}

fn create_local_disk_operator(settings: &Option<LocalDiskSettings>) -> Result<FileSystemOperator> {
    match settings {
        Some(LocalDiskSettings { root_path }) => {
            let root_path = root_path.clone()
                .into_os_string()
                .into_string()
                .map_err(|os_str| anyhow::anyhow!("non-utf8 path: {:?}", os_str))?;
            let builder = services::Fs::default().root(&root_path);
            let operator = Operator::new(builder)?
                .layer(LoggingLayer::default())
                .finish();
            Ok(FileSystemOperator {
                operator,
            })
        }
        None => Err(anyhow::anyhow!("LocalDiskSettings not found")),
    }
}

fn create_webdav_operator(settings: &Option<WebDavSettings>) -> Result<WebDavOperator> {
    match settings {
        Some(WebDavSettings { url, username, password }) => {
            let builder = services::Webdav::default()
                .endpoint(url)
                .username(username)
                .password(password);
            let operator = Operator::new(builder)?
                .layer(LoggingLayer::default())
                .finish();
            Ok(WebDavOperator {
                operator,
            })
        }
        None => Err(anyhow::anyhow!("WebDavSettings not found")),
    }
}

pub async fn list_directory_server((data_backend, path): (DataBackend, PathBuf)) -> Result<Vec<FsMetadata>> {
    let operator = create_operator(&data_backend).await?;
    operator.list_directory(path).await
}
