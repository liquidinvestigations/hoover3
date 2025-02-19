use anyhow::{Context, Result};
use hoover3_database::client_query::list_disk::data_root_path;
use hoover3_types::filesystem::FsMetadata;
use opendal::layers::LoggingLayer;
use opendal::services;
use opendal::Operator;
use std::path::PathBuf;
use tracing::info;

async fn get_filesystem_operator() -> Result<Operator> {
    let root_path = data_root_path()
        .await?
        .into_os_string()
        .into_string()
        .map_err(|os_str| anyhow::anyhow!("non-utf8 path: {:?}", os_str))?;
    println!("root_path: {:?}", root_path);
    let builder = services::Fs::default().root(&root_path);
    let operator = Operator::new(builder)?
        .layer(LoggingLayer::default())
        .finish();
    Ok(operator)
}

pub async fn get_path_metadata(relative_path: &str) -> Result<FsMetadata> {
    let operator = get_filesystem_operator().await?;
    let _path = data_root_path().await?.join(relative_path).canonicalize()?;
    let relative_path_buf = PathBuf::from(relative_path);
    let metadata = operator.stat(relative_path).await?;
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

pub async fn list_directory(relative_path: PathBuf) -> Result<Vec<FsMetadata>> {
    let operator = get_filesystem_operator().await?;
    let _path = data_root_path().await?.join(&relative_path);
    let path_string = relative_path
        .to_str()
        .context("non-utf8 filename")?
        .to_string();
    let mut entries = Vec::new();
    let list = operator.list(&path_string).await?;

    let mut entries_iter = list.iter();
    entries_iter.next();

    for entry in entries_iter {
        match get_path_metadata(entry.path()).await {
            Ok(metadata) => {
                entries.push(metadata);
            }
            Err(e) => {
                info!("Skipping entry due to error: {:?} - {}", entry.path(), e);
                continue;
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
