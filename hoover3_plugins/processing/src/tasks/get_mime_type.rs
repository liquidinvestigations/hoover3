// only for Rust Edition 2018, see https://doc.rust-lang.org/edition-guide/rust-2021/prelude.html
use std::path::PathBuf;

use hoover3_taskdef::anyhow;
use magic::cookie::Flags;

#[derive(Debug)]
pub struct MimeTypeResult {
    pub magic_mime_type: String,
    pub magika_result: MagikaResult,
}

#[derive(Debug)]
pub struct MagikaResult {
    pub magika_ruled_mime_type: Option<String>,
    pub magika_inferred_mime_type: Option<String>,
    pub magika_score: Option<f32>,
}

pub async fn magic_get_mime_type(path: PathBuf) -> anyhow::Result<MimeTypeResult> {
    // let magika_result = run_magika(path.clone()).await?;
    let mime_type = tokio::task::spawn_blocking(move || run_magic(path)).await??;

    Ok(MimeTypeResult {
        magic_mime_type: mime_type,
        magika_result: MagikaResult {
            magika_ruled_mime_type: None,
            magika_inferred_mime_type: None,
            magika_score: None,
        },
    })
}

// async fn run_magika(path: PathBuf) -> anyhow::Result<MagikaResult> {
//     let cookie = magika::Session::new()?;
//     let mime_type = cookie.identify_file_async(&path).await?;

//     let mime_type = match mime_type {
//         magika::FileType::Directory => MagikaResult {
//             magika_ruled_mime_type: Some("inode/directory".to_string()),
//             magika_inferred_mime_type: None,
//             magika_score: None,
//         },
//         magika::FileType::Symlink => MagikaResult {
//             magika_ruled_mime_type: Some("inode/symlink".to_string()),
//             magika_inferred_mime_type: None,
//             magika_score: None,
//         },
//         magika::FileType::Inferred(inferred) => MagikaResult {
//             magika_ruled_mime_type: None,
//             magika_inferred_mime_type: Some(inferred.content_type.info().mime_type.to_string()),
//             magika_score: Some(inferred.score),
//         },
//         magika::FileType::Ruled(ruled) => MagikaResult {
//             magika_ruled_mime_type: Some(ruled.content_type.info().mime_type.to_string()),
//             magika_inferred_mime_type: ruled
//                 .overruled
//                 .as_ref()
//                 .map(|inferred| inferred.content_type.info().mime_type.to_string()),
//             magika_score: ruled.overruled.map(|inferred| inferred.score),
//         },
//     };

//     Ok(mime_type)
// }

fn run_magic(path: PathBuf) -> Result<String, anyhow::Error> {
    let cookie = magic::Cookie::open(Flags::ERROR | Flags::MIME_TYPE | Flags::MIME_ENCODING)?;
    let cookie = cookie
        .load(&Default::default())
        .map_err(|e| anyhow::anyhow!("Failed to load cookie database: {:?}", e))?;
    let mime_type = cookie.file(path)?;

    Ok(mime_type)
}

#[cfg(test)]
mod tests {
    use hoover3_database::system_paths::get_data_root;

    use super::*;

    #[tokio::test]
    async fn test_magic_pdf() -> anyhow::Result<()> {
        let data_dir = get_data_root();
        let path = PathBuf::from(data_dir).join("hoover-testdata/data/no-extension/file_pdf");
        let magic  =magic_get_mime_type(path).await?;
        println!("{:?}", magic);
        assert_eq!(magic.magic_mime_type, "application/pdf; charset=binary");
        assert_eq!(magic.magika_result.magika_ruled_mime_type, None);
        assert_eq!(magic.magika_result.magika_inferred_mime_type, Some("application/pdf".to_string()));
        assert!(magic.magika_result.magika_score.unwrap() > 0.9);
        Ok(())
    }

}
