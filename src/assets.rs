use random_str as random;
use tokio::fs;

pub async fn generate_filename(filename: String) -> Result<AssetFilenameParts, anyhow::Error> {
    let upload_folder: String =
        std::env::var("UPLOADS_PATH").unwrap_or_else(|_| "./uploads".to_string());
    fs::create_dir_all(upload_folder.clone()).await.unwrap();
    /*     let mut file_path = "/".to_string();
    while fs::try_exists(file_path.clone()).await? {
        let prefix = random::get_string(16, true, false, true, false);
        file_path = format!("{upload_folder}/{prefix}-{filename}");
    } */
    let mut filepath = "/".to_string();
    let mut filename = filename;

    while fs::try_exists(filepath.clone()).await? {
        let prefix = random::get_string(16, true, false, true, false);
        filename = format!("{prefix}-{filename}");
        filepath = format!("{upload_folder}/{filename}");
    }
    log::info!("Generaated filename {filename} for {filepath}");
    Ok(AssetFilenameParts {
        filename,
        path: filepath,
    })
}

pub struct AssetFilenameParts {
    pub filename: String,
    pub path: String,
}
