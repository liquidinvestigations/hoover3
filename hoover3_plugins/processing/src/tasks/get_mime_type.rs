// only for Rust Edition 2018, see https://doc.rust-lang.org/edition-guide/rust-2021/prelude.html
use std::path::PathBuf;

use hoover3_taskdef::anyhow;
use magic::cookie::Flags;

pub async fn magic_get_mime_type(path: PathBuf) -> anyhow::Result<String> {
    let mime_type = tokio::task::spawn_blocking(move || run_magic(path)).await??;
    Ok(mime_type)
}

fn run_magic(path: PathBuf) -> Result<String, anyhow::Error> {
    let cookie = magic::Cookie::open(Flags::ERROR | Flags::MIME_TYPE | Flags::MIME_ENCODING)?;
    let cookie = cookie
        .load(&Default::default())
        .map_err(|e| anyhow::anyhow!("Failed to load cookie database: {:?}", e))?;
    let mime_type = cookie.file(path)?;

    Ok(mime_type)
}
