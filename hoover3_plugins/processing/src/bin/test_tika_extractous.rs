//! Debug multi-threaded tika

use extractous::{Extractor, PdfParserConfig};

// #[tokio::main]
fn main() -> anyhow::Result<()> {
    hoover3_tracing::init_tracing();
    let data_dir = std::env::args().nth(1).unwrap();
    let files = std::fs::read_dir(data_dir).unwrap();
    let extractor = Extractor::new().set_extract_string_max_length(1024 * 1024).set_xml_output(false).set_pdf_config(PdfParserConfig::new().set_ocr_strategy(extractous::PdfOcrStrategy::NO_OCR));
    for file in files {
        let Ok(file) = file else {
            continue;
        };
        let Ok(metadata) = file.metadata() else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        let file_size = metadata.len();
        let path = file.path();

        let t0 = std::time::Instant::now();
        let path2 = path.clone();

        // let (mut text, metadata) = tokio::task::spawn_blocking(move ||
        //  extractor.extract_file(path2.to_str().unwrap()).unwrap()).await?;
        let (mut text, metadata) = extractor.extract_file(path2.to_str().unwrap()).unwrap();
        let meta_len = format!("{:?}", metadata).len();
        let _ = std::io::copy(&mut text, &mut std::io::sink());
        // println!("text: {}", &text[0..20]);
        // println!("metadata: {:?}", metadata);
        println!("{}: meta_len: {}K, file size: {}K, after {} ms", path.file_name().unwrap().to_str().unwrap(), meta_len as f32 / 1024.0, file_size / 1024, t0.elapsed().as_millis());
    }
    Ok(())
}
