pub async fn fetch_spotify_endpoint() -> reqwest::Result<()> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    let api_update_token =
        std::env::var("API_UPDATE_TOKEN").unwrap_or_else(|_| "beans".to_string());
    let client = reqwest::Client::new();
    client
        .get(format!("http://{host}:{port}/api/spotify"))
        .bearer_auth(api_update_token)
        .send()
        .await?;

    Ok(())
}
