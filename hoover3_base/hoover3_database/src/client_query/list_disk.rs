//! List files and directories on disk.

use anyhow::{Context, Result};
use futures::stream::{self, Stream};
use hoover3_types::filesystem::FsMetadataBasic;
use std::io::SeekFrom;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
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

/// Read a file from disk and return it as an async stream of 4MB byte chunks.
///
/// # Arguments
/// * `relative_path` - Path to the file, relative to the data root
///
/// # Returns
/// * The size of the file in bytes
/// * A stream of `Result<Vec<u8>>` where each Vec is a chunk of the file
pub async fn read_file_to_stream(
    relative_path: PathBuf,
) -> Result<(usize, Pin<Box<dyn Stream<Item = Result<Vec<u8>>> + Send>>)> {
    const CHUNK_SIZE: usize = 4 * 1024 * 1024; // 4MB chunks

    let path = get_data_root().join(relative_path);

    let file = tokio::fs::File::open(&path)
        .await
        .context(format!("Failed to open file: {:?}", path))?;

    let file_size = file.metadata().await?.len() as usize;

    // Create a stream that reads chunks on demand
    let stream = stream::unfold(
        (file, 0usize, file_size),
        |(mut file, chunk_index, file_size)| async move {
            if chunk_index * CHUNK_SIZE >= file_size {
                return None; // We've read the entire file
            }

            // Calculate the size of this chunk (might be smaller for the last chunk)
            let remaining = file_size - (chunk_index * CHUNK_SIZE);
            let this_chunk_size = std::cmp::min(CHUNK_SIZE, remaining);

            // Seek to the correct position and read the chunk
            let mut buffer = vec![0u8; this_chunk_size];
            match file
                .seek(SeekFrom::Start((chunk_index * CHUNK_SIZE) as u64))
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    return Some((
                        Err(anyhow::Error::new(e).context("Failed to seek in file")),
                        (file, chunk_index + 1, file_size),
                    ))
                }
            }

            match file.read_exact(&mut buffer).await {
                Ok(_) => Some((Ok(buffer), (file, chunk_index + 1, file_size))),
                Err(e) => Some((
                    Err(anyhow::Error::new(e).context("Failed to read chunk from file")),
                    (file, chunk_index + 1, file_size),
                )),
            }
        },
    );

    Ok((file_size, Box::pin(stream)))
}
