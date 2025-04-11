//! List files and directories on disk.
use anyhow::{Context, Result};
use futures::stream::{self, Stream};
use hoover3_types::datasource::DatasourceSettings;
use hoover3_types::identifier::{CollectionId, DatabaseIdentifier};
use std::io::SeekFrom;
use std::path::PathBuf;
use std::pin::Pin;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use hoover3_database::system_paths::get_data_root;

/// Read a file from disk and return it as an async stream of 4MB byte chunks.
///
/// # Returns
/// * The size of the file in bytes
/// * A stream of `Result<Vec<u8>>` where each Vec is a chunk of the file
pub async fn read_file_to_stream(
    collection_id: CollectionId,
    datasource_id: DatabaseIdentifier,
    parent_dir_path: String,
    file_name: String,
) -> Result<(usize, Pin<Box<dyn Stream<Item = Result<Vec<u8>>> + Send>>)> {
    // get datasource dir
    let ds_row = crate::api::get_datasource((collection_id.clone(), datasource_id.clone())).await?;

    let DatasourceSettings::LocalDisk { path: root_path } = &ds_row.datasource_settings else {
        anyhow::bail!("Datasource is not a local disk");
    };
    let root_path = root_path.to_path_buf();

    let file_path = root_path.join(&parent_dir_path).join(&file_name);

    fs_read_file_to_stream(file_path).await
}

async fn fs_read_file_to_stream(
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
