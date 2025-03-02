//! List files and directories on disk.

use anyhow::{Context, Result};
use hoover3_types::filesystem::FsMetadataBasic;
use std::path::PathBuf;
use tokio::fs;
use tracing::info;

use crate::system_paths::get_data_root;


/// Get metadata for a single file or directory on disk.
pub async fn get_path_metadata(relative_path: PathBuf) -> Result<FsMetadataBasic> {
    let path = get_data_root().join(relative_path).canonicalize()?;
    let relative_path = path
        .strip_prefix(get_data_root())
        .context("path is not relative to root")?
        .to_path_buf();
    let metadata = fs::metadata(&path)
        .await
        .context(format!("metadata read failed: {:?}", path))?;
    use chrono::DateTime;
    let _path_string = relative_path
        .to_str()
        .context("non-utf8 filename")?
        .to_string();

    Ok(FsMetadataBasic {
        is_dir: metadata.is_dir(),
        is_file: metadata.is_file(),
        size_bytes: metadata.len(),
        modified: metadata.modified().ok().map(DateTime::from),
        created: metadata.created().ok().map(DateTime::from),
        path: relative_path,
    })
}

/// List all files and directories in the given directory on disk.
pub async fn list_directory(relative_path: PathBuf) -> Result<Vec<FsMetadataBasic>> {
    let path = get_data_root().join(relative_path);
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
