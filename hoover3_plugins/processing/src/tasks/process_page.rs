use std::path::PathBuf;

use futures::{pin_mut, stream::FuturesUnordered, StreamExt};
use hoover3_data_access::list_disk::read_file_to_stream;
use hoover3_database::{
    charybdis::operations::Insert,
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
};
use hoover3_filesystem_scanner::models::{
    BlobProcessingPlanPage, FsBlobHashesDbRow, PartialUpdateFsBlobHashesMimeTypeDbRow,
};
use hoover3_taskdef::{activity, anyhow, WORKER_TEMPDIR_ENV_VAR};
use hoover3_tracing::tracing::warn;
use hoover3_types::{identifier::DatabaseIdentifier, processing::ProcessPageResult};
use tokio::io::AsyncWriteExt;

use super::{
    get_mime_type::get_mime_type, process_group::ProcessPageArgs, ProcessingQueueBigPage,
    ProcessingQueueSmallPage,
};

/// Activity for processing a page.
#[activity(ProcessingQueueSmallPage)]
async fn process_small_page(_args: ProcessPageArgs) -> anyhow::Result<ProcessPageResult> {
    process_page(_args).await
}

/// Activity for processing a big page.
#[activity(ProcessingQueueBigPage)]
async fn process_big_page(_args: ProcessPageArgs) -> anyhow::Result<ProcessPageResult> {
    process_page(_args).await
}

async fn process_page(args: ProcessPageArgs) -> anyhow::Result<ProcessPageResult> {
    let tempdir = std::env::var(WORKER_TEMPDIR_ENV_VAR)?;
    let tempdir = PathBuf::from(tempdir).canonicalize()?;
    let tempdir = tempdir
        .join(args.collection_id.to_string())
        .join(args.plan_page_id.to_string());
    tokio::fs::create_dir_all(&tempdir).await?;

    let session = ScyllaDatabaseHandle::collection_session(&args.collection_id).await?;
    let model_stream = BlobProcessingPlanPage::find_by_plan_page_id(args.plan_page_id)
        .execute(&session)
        .await?;
    pin_mut!(model_stream);
    let mut fut = FuturesUnordered::new();
    while let Some(model) = model_stream.next().await {
        let model = model?;
        let blob_sha3_256 = model.blob_sha3_256;
        let segment1 = &blob_sha3_256[0..3];
        let segment2 = &blob_sha3_256[3..6];
        let tempdir = tempdir.join(segment1).join(segment2).join(&blob_sha3_256);
        tokio::fs::create_dir_all(&tempdir).await?;
        let args2 = args.clone();
        fut.push(async move {
            let r = download_and_process_item(args2, blob_sha3_256.clone(), tempdir).await;
            (blob_sha3_256, r)
        });
    }
    let mut process_page_results = ProcessPageResult::default();

    while let Some((_blob_sha3_256, r)) = fut.next().await {
        process_page_results.item_count += 1;
        match r {
            Ok(_r) => process_page_results.item_success += 1,
            Err(_e) => {
                warn!("Error processing item: {:?}", _e);
                process_page_results.item_errors += 1;
            }
        }
    }
    tokio::fs::remove_dir_all(&tempdir).await?;
    Ok(process_page_results)
}

async fn download_and_process_item(
    args: ProcessPageArgs,
    blob_sha3_256: String,
    tempdir: PathBuf,
) -> anyhow::Result<()> {
    let session = ScyllaDatabaseHandle::collection_session(&args.collection_id).await?;
    let blob = FsBlobHashesDbRow::find_by_blob_sha3_256(blob_sha3_256.clone())
        .execute(&session)
        .await?;
    drop(session);

    let temp_file_path = tempdir.join(&blob_sha3_256);
    let mut file = tokio::fs::File::create(&temp_file_path).await?;
    // let file = FsFileDbRow::find_by_datasource_id_and_parent_dir_path_and_file_name(blob.datasource_id.clone(), blob.parent_dir_path.clone(), blob.file_name.clone()).execute(&session).await?;
    let ds = DatabaseIdentifier::new(blob.datasource_id)?;

    let (file_size, stream) = read_file_to_stream(
        args.collection_id.clone(),
        ds,
        blob.parent_dir_path.clone(),
        blob.file_name.clone(),
    )
    .await?;
    if file_size as i64 != blob.size_bytes {
        anyhow::bail!("File size mismatch for stream file: {:?}", blob.file_name);
    }

    pin_mut!(stream);
    while let Some(Ok(chunk)) = stream.next().await {
        file.write_all(&chunk).await?;
    }
    file.flush().await?;
    file.sync_all().await?;
    drop(file);

    let downloaded_size = tokio::fs::metadata(&temp_file_path).await?.len();
    if downloaded_size as i64 != blob.size_bytes {
        anyhow::bail!(
            "File size mismatch for downloaded file: {:?}",
            blob.file_name
        );
    }

    let r = process_item(args, blob_sha3_256, temp_file_path.clone()).await;

    tokio::fs::remove_file(&temp_file_path).await?;

    r
}

async fn process_item(
    args: ProcessPageArgs,
    blob_sha3_256: String,
    temp_path: PathBuf,
) -> anyhow::Result<()> {
    let mime_type = get_mime_type(temp_path).await?;

    let row = PartialUpdateFsBlobHashesMimeTypeDbRow {
        blob_sha3_256,
        mime_type,
    };
    let session = ScyllaDatabaseHandle::collection_session(&args.collection_id).await?;

    row.insert().execute(&session).await?;

    anyhow::Ok(())
}
