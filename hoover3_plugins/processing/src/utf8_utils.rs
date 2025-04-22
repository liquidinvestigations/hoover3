use std::path::PathBuf;

use futures::Stream;
use text_splitter::ChunkConfig;

fn read_utf8_file_chunked_by_bytes(
    content_path: PathBuf,
    chunk_size_bytes: i32,
) -> impl Stream<Item = anyhow::Result<String>> {
    async_stream::try_stream! {
        let file = tokio::fs::File::open(content_path).await?;
        let file = tokio::io::BufReader::new(file);
        let file = tokio_util::compat::TokioAsyncReadCompatExt::compat(file);
        use futures::StreamExt;
        let reader = async_utf8_decoder::Utf8Decoder::new(file);
        futures::pin_mut!(reader);

        let mut buf = String::new();
        while let Some(chunk) = reader.next().await {
            buf.push_str(&chunk?);
            if buf.len() >= chunk_size_bytes as usize {
                yield buf;
                buf = String::new();
            }
        }
        if !buf.is_empty() {
            yield buf;
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ByteTextSizer;

impl text_splitter::ChunkSizer for ByteTextSizer {
    fn size(&self, chunk: &str) -> usize {
        chunk.len()
    }
}

/// Reads a UTF-8 file and splits it into paragraphs under a given size in bytes.
pub fn read_utf8_file_paragraphs(
    content_path: PathBuf,
    paragraph_size_bytes: i32,
) -> impl Stream<Item = anyhow::Result<String>> {
    async_stream::try_stream! {
        use text_splitter::TextSplitter;
        use futures::StreamExt;

        // Create a text splitter with the desired paragraph size range
        let min_size = paragraph_size_bytes as usize * 3 / 4;
        let max_size = paragraph_size_bytes as usize;
        let chunk_size_bytes = paragraph_size_bytes * 2;
        let splitter_config = ChunkConfig::new(min_size..max_size).with_sizer(ByteTextSizer);
        let splitter = TextSplitter::new(splitter_config);

        // Read the file in chunks using the existing function
        let chunk_stream = read_utf8_file_chunked_by_bytes(content_path, chunk_size_bytes);
        futures::pin_mut!(chunk_stream);
        // Keep track of any leftover text from the previous chunk
        let mut leftover = String::new();

        while let Some(chunk_result) = chunk_stream.next().await {
            let chunk = chunk_result?;

            // Combine leftover text from previous chunk with current chunk
            leftover.push_str(&chunk);

            if leftover.len() < min_size {
                continue;
            }

            // Split the combined text into paragraphs
            let leftover2 = leftover.clone();
            let paragraphs = splitter.chunk_indices(&leftover2).filter(|(i, _)| *i < max_size).collect::<Vec<_>>();
            if paragraphs.is_empty() || paragraphs.len() == 1 {
                continue;
            }
            let split_offset = paragraphs.last().unwrap().0;
            let (left, right) = leftover.split_at(split_offset);
            yield left.to_string();
            leftover = right.to_string();
        }

        while leftover.len() > max_size {
            let chunk = leftover.clone();
            let paragraphs = splitter.chunk_indices(&chunk).filter(|(i, _)| *i < max_size).collect::<Vec<_>>();
            if paragraphs.len() <= 1 {
                break;
            }
            let split_offset = paragraphs.last().unwrap().0;
            let (left, right) = leftover.split_at(split_offset);
            yield left.to_string();
            leftover = right.to_string();
        }

        if !leftover.is_empty() {
            yield leftover;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_read_utf8_file_paragraphs() -> anyhow::Result<()> {
        // Create a sample paragraph that will be repeated
        let paragraph = "This is a test paragraph with some content.\n\n";

        // Create a long text by repeating the paragraph (approximately 4KB)
        let repetitions = 4096 / paragraph.len() + 1;
        let long_text = paragraph.repeat(repetitions);

        // Write the text to a temporary file
        let mut temp_file = NamedTempFile::new()?;
        temp_file.write_all(long_text.as_bytes())?;
        let file_path = temp_file.into_temp_path();

        // Use small chunk sizes to test the chunking logic
        let paragraph_size_bytes = 128; // Small paragraph size

        // Read the file using our function
        let paragraph_stream =
            read_utf8_file_paragraphs(file_path.to_path_buf(), paragraph_size_bytes);
        futures::pin_mut!(paragraph_stream);

        // Collect all paragraphs from the stream
        let mut paragraphs = vec![];
        while let Some(paragraph) = paragraph_stream.next().await {
            let paragraph = paragraph?;
            if paragraph.len() > paragraph_size_bytes as usize {
                anyhow::bail!("Paragraph is too long: {}", paragraph.len());
            }
            if paragraph.len() < paragraph_size_bytes as usize / 2 {
                anyhow::bail!("Paragraph is too short: {}", paragraph.len());
            }
            paragraphs.push(paragraph);
        }

        // Join the paragraphs back together
        let reconstructed_text = paragraphs.join("");

        // Verify the output matches the original text
        assert_eq!(reconstructed_text, long_text);

        // Clean up
        file_path.close()?;

        Ok(())
    }
}
