use std::{collections::BTreeMap, path::PathBuf};

use anyhow::Context;
use extractous::{Extractor, PdfOcrStrategy, PdfParserConfig, StreamReader};

/// Result of the tika extraction for a single file.
pub struct TikaResult {
    /// The metadata extracted from the file.
    pub metadata: BTreeMap<String, Vec<String>>,
    /// The length of the content of the file.
    pub _original_content_length: Option<u64>,
    /// The type of the content of the file (mime type).
    pub content_type: Option<String>,
    /// A path to the content of the file, or error if we failed to extract the content.
    pub extracted_content: Result<PathBuf, anyhow::Error>,
}

/// Use extractous (tika) to extract metadata from a file.
pub async fn extract_metadata(path: PathBuf, temp_dir: PathBuf) -> anyhow::Result<TikaResult> {
    tokio::task::spawn_blocking(move || run_extract_metadata(path, temp_dir)).await?
}

fn run_extract_metadata(path: PathBuf, temp_dir: PathBuf) -> anyhow::Result<TikaResult> {
    let path = path.to_str().context("invalid path")?;
    let extractor = Extractor::new().set_pdf_config(PdfParserConfig::new().set_ocr_strategy(PdfOcrStrategy::NO_OCR));
    let (content, mut metadata) = extractor.extract_file(&path)?;
    let content_length = metadata
        .remove("Content-Length")
        .map(|v| v.first().cloned())
        .flatten()
        .map(|s| s.parse::<u64>().ok())
        .flatten();
    let content_type = metadata
        .remove("Content-Type")
        .map(|v| v.first().cloned().unwrap_or_default());

    let metadata = metadata
        .into_iter()
        .map(|(k, vs)| {
            (
                k,
                vs.into_iter()
                    .map(|v| truncate_utf8_string(v, 64 * 1024))
                    .collect(),
            )
        })
        .collect();

    Ok(TikaResult {
        metadata,
        extracted_content: download_content(temp_dir, content),
        _original_content_length: content_length,
        content_type,
    })
}

fn truncate_utf8_string(s: String, max_length: usize) -> String {
    if s.len() <= max_length {
        return s;
    }
    let mut chars = s.chars();
    let mut truncated = String::new();
    while let Some(c) = chars.next() {
        if truncated.len() >= max_length - 30 {
            truncated.push_str("\n[ ... LONG TEXT TRUNCATED BY HOOVER ...]\n");
            break;
        }
        truncated.push(c);
    }
    truncated
}

fn download_content(tempdir: PathBuf, mut content: StreamReader) -> Result<PathBuf, anyhow::Error> {
    let file_path = tempdir.join("tika_output_content");
    let mut file = std::fs::File::create(&file_path)?;
    std::io::copy(&mut content, &mut file)?;
    Ok(file_path)
}
