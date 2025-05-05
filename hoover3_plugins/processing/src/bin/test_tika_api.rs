//! Debug multi-threaded tika

use extractous::Extractor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hoover3_tracing::init_tracing();
    let data_dir = std::env::args().nth(1).unwrap();
    let files = std::fs::read_dir(data_dir).unwrap();
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
        let body = run_tika_api(&path).await?;
        println!("{}: filesize = {}k, after {} ms, resp len: {}k", path.file_name().unwrap().to_str().unwrap(), file_size / 1024, t0.elapsed().as_millis(), body.len() / 1024);
    }
    Ok(())
}


async fn run_tika_api(path: &std::path::Path) -> anyhow::Result<String> {
    let mime = hoover3_processing::tasks::get_mime_type::magic_get_mime_type(path.to_path_buf()).await?;
    let client = reqwest::Client::new();
    let url = "http://localhost:9998/rmeta/text";
    let body = tokio::fs::File::open(path).await?;
    let response = client.put(url).body(body).header("Content-Type", mime.magic_mime_type.split(";").into_iter().next().unwrap()).send().await?;
    let code = response.status().as_u16();
    if code != 200 {
        return Err(anyhow::anyhow!("HTTP code: {}", code));
    }
    let body = response.text().await?;
    Ok(body)
}

