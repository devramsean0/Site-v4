use random_str as random;
use tokio::fs;

pub async fn generate_filename(filename: String) -> Result<String, anyhow::Error> {
    let upload_folder = std::env::var("UPLOADS_PATH").unwrap_or_else(|_| "./uploads".to_string());
    fs::create_dir_all(upload_folder.clone()).await.unwrap();
    let mut file_path = "/".to_string();
    while fs::try_exists(file_path.clone()).await? {
        let prefix = random::get_string(16, true, false, true, false);
        file_path = format!("{upload_folder}/{prefix}-{filename}");
    }
    Ok(file_path)
}
