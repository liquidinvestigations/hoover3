use std::{path::PathBuf, time::Instant};

use charybdis::batch::ModelBatch;
use futures::{pin_mut, Stream, StreamExt};
use hoover3_data_access::list_disk::read_file_to_stream;
use hoover3_database::{
    charybdis::operations::Update,
    db_management::{DatabaseSpaceManager, ScyllaDatabaseHandle},
    models::collection::DatabaseExtraCallbacks,
};
use hoover3_filesystem_scanner::models::{
    BlobProcessingPlan, BlobProcessingPlanPageBlobs, FsBlobHashesDbRow, FsBlobMimeTypeDbRow,
};
use hoover3_taskdef::{activity, anyhow, WORKER_TEMPDIR_ENV_VAR_BIG, WORKER_TEMPDIR_ENV_VAR_SMALL};
use hoover3_tracing::tracing::{info, warn};
use hoover3_types::{
    identifier::{CollectionId, DatabaseIdentifier},
    processing::ProcessPageResult,
};
use tokio::io::AsyncWriteExt;

use crate::{
    models::{BlobExtractedContentRow, BlobExtractedMetadataRow},
    utf8_utils::read_utf8_file_paragraphs,
};

use super::{
    get_mime_type::magic_get_mime_type, process_group::ProcessPageArgs, ProcessingQueueBigPage,
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
    info!(
        "Processing {} page: {}",
        args.collection_id, args.plan_page_id
    );
    let tempdir_env = match args.page_is_small {
        true => WORKER_TEMPDIR_ENV_VAR_SMALL,
        false => WORKER_TEMPDIR_ENV_VAR_BIG,
    };
    let tempdir = std::env::var(tempdir_env)?;
    let tempdir = PathBuf::from(tempdir).canonicalize()?;
    let tempdir = tempdir
        .join("processing_tmp")
        .join(args.collection_id.to_string())
        .join(args.plan_page_id.to_string());
    tokio::fs::create_dir_all(&tempdir).await?;

    let session = ScyllaDatabaseHandle::collection_session(&args.collection_id).await?;
    let mut plan = BlobProcessingPlan::find_by_plan_page_id(args.plan_page_id)
        .execute(&session)
        .await?;
    plan.is_started = true;
    plan.update().execute(&session).await?;

    let (model_tx, mut model_rx) = tokio::sync::mpsc::channel(16);
    let _tempdir = tempdir.clone();
    let _args = args.clone();
    let _model_reader_task = async move {
        let model_stream = BlobProcessingPlanPageBlobs::find_by_plan_page_id(_args.plan_page_id)
            .execute(&session)
            .await?;
        pin_mut!(model_stream);
        while let Some(model) = model_stream.next().await {
            let model = model?;
            let blob_sha3_256 = model.blob_sha3_256.clone();
            let segment1 = &blob_sha3_256[0..3];
            let segment2 = &blob_sha3_256[3..6];
            let tempdir = _tempdir.join(segment1).join(segment2).join(&blob_sha3_256);
            tokio::fs::create_dir_all(&tempdir).await?;
            let tempdir = tokio::fs::canonicalize(&tempdir).await?;
            model_tx.send((model, tempdir)).await?;
        }
        drop(model_tx);
        anyhow::Ok(())
    };

    let (download_tx, mut download_rx) = tokio::sync::mpsc::channel(2);
    let _args = args.clone();
    let _download_task = async move {
        while let Some((model, tempdir)) = model_rx.recv().await {
            let filepath =
                download_item(_args.clone(), model.blob_sha3_256.clone(), tempdir.clone()).await?;
            download_tx.send((model, tempdir, filepath)).await?;
        }
        drop(download_tx);
        anyhow::Ok(())
    };

    let (item_result_tx, mut item_result_rx) = tokio::sync::mpsc::channel(2);
    let _item_process_task = async move {
        while let Some((model, tempdir, filepath)) = download_rx.recv().await {
            let r = process_item(
                model.blob_sha3_256.clone(),
                filepath.clone(),
                tempdir.clone(),
            )
            .await;
            item_result_tx.send((r, tempdir)).await?;
            tokio::fs::remove_file(&filepath).await?;
        }
        drop(item_result_tx);
        anyhow::Ok(())
    };

    let _args = args.clone();
    let _item_save_task = async move {
        let session = ScyllaDatabaseHandle::collection_session(&_args.collection_id).await?;
        let extra = DatabaseExtraCallbacks::new(&_args.collection_id).await?;
        let mut process_page_results = ProcessPageResult::default();
        let mut batches =
            ProcessItemsWriteBatches::new(&_args.collection_id, session, extra).await?;

        while let Some((r, tempdir)) = item_result_rx.recv().await {
            process_page_results.item_count += 1;
            match r {
                Ok(_r) => {
                    process_page_results.item_success += 1;
                    batches.accept(_r).await?;
                }
                Err(_e) => {
                    warn!("Error processing item: {:?}", _e);
                    process_page_results.item_errors += 1;
                }
            }
            tokio::fs::remove_dir_all(&tempdir).await?;
        }
        batches.finalize().await?;
        anyhow::Ok(process_page_results)
    };

    let _model_reader_task = tokio::spawn(_model_reader_task);
    let _download_task = tokio::spawn(_download_task);
    let _item_process_task = tokio::spawn(_item_process_task);
    let _item_save_task = tokio::spawn(_item_save_task);

    let _ = _model_reader_task.await?;
    let _ = _download_task.await?;
    let _ = _item_process_task.await?;
    let process_page_results = _item_save_task.await??;

    tokio::fs::remove_dir_all(&tempdir).await?;
    info!(
        "ProcessItemsPage {}/{}: done",
        args.collection_id, args.plan_page_id
    );
    Ok(process_page_results)
}

struct ProcessItemsWriteBatches {
    collection_id: CollectionId,
    mime_type_rows: Vec<FsBlobMimeTypeDbRow>,
    tika_meta_rows: Vec<BlobExtractedMetadataRow>,
    tika_content_rows: Vec<BlobExtractedContentRow>,
    tika_content_total_size: i32,
    extra: DatabaseExtraCallbacks,
    session: std::sync::Arc<ScyllaDatabaseHandle>,
}

impl ProcessItemsWriteBatches {
    async fn new(
        collection_id: &CollectionId,
        session: std::sync::Arc<ScyllaDatabaseHandle>,
        extra: DatabaseExtraCallbacks,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            collection_id: collection_id.clone(),
            mime_type_rows: vec![],
            tika_meta_rows: vec![],
            tika_content_rows: vec![],
            tika_content_total_size: 0,
            extra,
            session,
        })
    }
    async fn finalize(&mut self) -> anyhow::Result<()> {
        info!(
            "ProcessItemsWriteBatches: finalize, collection_id: {}",
            self.collection_id
        );
        self.write_mime_type_rows().await?;
        self.write_tika_meta_rows().await?;
        self.write_tika_content_rows().await?;
        anyhow::Ok(())
    }

    async fn write_mime_type_rows(&mut self) -> anyhow::Result<()> {
        if self.mime_type_rows.is_empty() {
            return anyhow::Ok(());
        }
        let t0 = Instant::now();
        info!(
            "ProcessItemsWriteBatches: write_mime_type_rows: {} items, collection_id: {}",
            self.mime_type_rows.len(),
            self.collection_id
        );
        let mut batch = FsBlobMimeTypeDbRow::batch();
        batch.append_inserts(&self.mime_type_rows);
        batch.execute(&self.session).await?;
        self.extra.insert(&self.mime_type_rows).await?;
        self.mime_type_rows.clear();
        info!("ProcessItemsWriteBatches: write_mime_type_rows: {} items, collection_id: {}, time: {:?}",self.mime_type_rows.len(), self.collection_id, t0.elapsed());
        anyhow::Ok(())
    }
    async fn write_tika_meta_rows(&mut self) -> anyhow::Result<()> {
        if self.tika_meta_rows.is_empty() {
            return anyhow::Ok(());
        }
        let t0 = Instant::now();
        info!(
            "ProcessItemsWriteBatches: write_tika_meta_rows: {} items, collection_id: {}",
            self.tika_meta_rows.len(),
            self.collection_id
        );
        let mut batch = BlobExtractedMetadataRow::batch();
        batch.append_inserts(&self.tika_meta_rows);
        batch.execute(&self.session).await?;
        self.extra.insert(&self.tika_meta_rows).await?;
        self.tika_meta_rows.clear();
        info!("ProcessItemsWriteBatches: write_tika_meta_rows: {} items, collection_id: {}, time: {:?}",self.tika_meta_rows.len(), self.collection_id, t0.elapsed());
        anyhow::Ok(())
    }
    async fn write_tika_content_rows(&mut self) -> anyhow::Result<()> {
        if self.tika_content_rows.is_empty() {
            return anyhow::Ok(());
        }
        let t0 = Instant::now();
        info!(
            "ProcessItemsWriteBatches: write_tika_content_rows: {} items, collection_id: {}",
            self.tika_content_rows.len(),
            self.collection_id
        );
        info!(
            "batch size: {} rows = {} bytes",
            self.tika_content_rows.len(),
            self.tika_content_total_size
        );
        let batch = BlobExtractedContentRow::batch();
        batch
            .chunked_insert(&self.session, &self.tika_content_rows, 1)
            .await?;
        self.extra.insert(&self.tika_content_rows).await?;
        self.tika_content_rows.clear();
        self.tika_content_total_size = 0;
        info!("ProcessItemsWriteBatches: write_tika_content_rows: {} items, collection_id: {}, time: {:?}",self.tika_content_rows.len(), self.collection_id, t0.elapsed());
        anyhow::Ok(())
    }

    async fn accept(&mut self, item: ProcessItemResultRows) -> anyhow::Result<()> {
        self.mime_type_rows.push(item.mime_type_row);
        if self.mime_type_rows.len() >= 500 {
            self.write_mime_type_rows().await?;
        }
        self.tika_meta_rows.extend(item.tika_meta_rows);
        if self.tika_meta_rows.len() >= 300 {
            self.write_tika_meta_rows().await?;
        }
        if let Some(tika_content_path) = item.tika_content_path {
            let paragraphs =
                read_tika_content_rows(tika_content_path.clone(), item.blob_sha3_256.clone());
            pin_mut!(paragraphs);
            while let Some(row) = paragraphs.next().await {
                let row = row?;
                self.tika_content_total_size += row.content_length + 1024;
                self.tika_content_rows.push(row);
                if self.tika_content_rows.len() >= 100
                    || self.tika_content_total_size >= 50 * 1024 * 1024
                {
                    self.write_tika_content_rows().await?;
                }
            }
            tokio::fs::remove_file(tika_content_path).await?;
        }
        anyhow::Ok(())
    }
}
struct ProcessItemResultRows {
    blob_sha3_256: String,
    mime_type_row: FsBlobMimeTypeDbRow,
    tika_meta_rows: Vec<BlobExtractedMetadataRow>,
    tika_content_path: Option<PathBuf>,
}

async fn download_item(
    args: ProcessPageArgs,
    blob_sha3_256: String,
    tempdir: PathBuf,
) -> anyhow::Result<PathBuf> {
    let session = ScyllaDatabaseHandle::collection_session(&args.collection_id).await?;
    let blob = FsBlobHashesDbRow::find_by_blob_sha3_256(blob_sha3_256.clone())
        .execute(&session)
        .await?;
    drop(session);

    let temp_file_path = tempdir.join("the_file");
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
    Ok(temp_file_path)
}

async fn process_item(
    blob_sha3_256: String,
    file_path: PathBuf,
    temp_dir: PathBuf,
) -> anyhow::Result<ProcessItemResultRows> {
    let magic_mime_type = magic_get_mime_type(file_path.clone()).await?;
    let tika_result = crate::tasks::tika::extract_metadata(file_path, temp_dir).await;
    let tika_metadata = tika_result
        .as_ref()
        .ok()
        .map(|r| r.metadata.clone())
        .unwrap_or_default();
    let tika_type = tika_result
        .as_ref()
        .ok()
        .map(|r| r.content_type.clone())
        .flatten();
    let tika_metadata_success = tika_result.is_ok();
    let tika_content_path = match tika_result {
        Ok(r) => r.extracted_content,
        Err(_e) => Err(_e),
    };
    let tika_content_success = tika_content_path.is_ok();

    let mime_type_row = FsBlobMimeTypeDbRow {
        blob_sha3_256: blob_sha3_256.clone(),
        magic_mime: magic_mime_type.magic_mime_type,
        magika_ruled_mime: magic_mime_type.magika_result.magika_ruled_mime_type,
        magika_inferred_mime: magic_mime_type.magika_result.magika_inferred_mime_type,
        magika_score: magic_mime_type.magika_result.magika_score,
        tika_metadata_success,
        tika_content_success,
        tika_mime: tika_type.unwrap_or_default(),
    };
    // write tika metadata to table
    let mut tika_meta_rows = vec![];
    for (key, values) in tika_metadata.iter() {
        for (i, value) in values.iter().enumerate() {
            let tika_metadata_row = BlobExtractedMetadataRow {
                blob_sha3_256: blob_sha3_256.clone(),
                meta_provider: "tika".to_string(),
                meta_key: key.to_string(),
                list_index: i as i32,
                value: value.to_string(),
            };
            tika_meta_rows.push(tika_metadata_row);
        }
    }

    anyhow::Ok(ProcessItemResultRows {
        blob_sha3_256,
        mime_type_row,
        tika_meta_rows,
        tika_content_path: tika_content_path.ok(),
    })
}

fn read_tika_content_rows(
    content_path: PathBuf,
    blob_sha3_256: String,
) -> impl Stream<Item = anyhow::Result<BlobExtractedContentRow>> {
    async_stream::try_stream! {
        let content_stream = read_utf8_file_paragraphs(content_path, 768 * 1024);
        pin_mut!(content_stream);
        let mut list_index = 0;
        while let Some(chunk) = content_stream.next().await {
            let chunk = chunk?;
            if chunk.trim().is_empty() {
                continue;
            }
            let row = BlobExtractedContentRow {
                blob_sha3_256: blob_sha3_256.clone(),
                content_source: "tika".to_string(),
                list_index,
                content_length: chunk.len() as i32,
                content: chunk,
            };
            list_index += 1;
            yield row;
        }
    }
}
