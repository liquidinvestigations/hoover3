use crate::migrate::get_package_dir;
use anyhow::{Context, Result};
use hoover3_types::filesystem::FsMetadata;
use std::path::PathBuf;
use tokio::fs;
use tracing::info;

pub async fn data_root_path() -> Result<PathBuf> {
    Ok(get_package_dir()
        .parent()
        .context("no parent?")?
        .join("data"))
}

pub async fn get_path_metadata(relative_path: PathBuf) -> Result<FsMetadata> {
    let path = data_root_path().await?.join(relative_path).canonicalize()?;
    let relative_path = path
        .strip_prefix(data_root_path().await?)
        .context("path is not relative to root")?
        .to_path_buf();
    let metadata = fs::metadata(&path)
        .await
        .context(format!("metadata read failed: {:?}", path))?;
    use chrono::DateTime;
    let path_string = relative_path
        .to_str()
        .context("non-utf8 filename")?
        .to_string();

    Ok(FsMetadata {
        is_dir: metadata.is_dir(),
        is_file: metadata.is_file(),
        size_bytes: metadata.len(),
        modified: metadata.modified().ok().map(|t| DateTime::from(t)),
        created: metadata.created().ok().map(|t| DateTime::from(t)),
        path: relative_path,
        path_string,
    })
}

pub async fn list_directory(relative_path: PathBuf) -> Result<Vec<FsMetadata>> {
    let path = data_root_path().await?.join(relative_path);
    info!("list_directory: {:?}", path);
    let mut entries = Vec::new();
    let mut read_dir = fs::read_dir(&path)
        .await
        .context(format!("read_dir failed: {:?}", path))?;

    while let Some(entry) = read_dir
        .next_entry()
        .await
        .context(format!("read_dir entry failed: {:?}", path))?
    {
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
