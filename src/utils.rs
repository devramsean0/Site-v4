use std::sync::Arc;

use actix_session::Session;
use actix_web::web;
use async_sqlite::Pool;
use reqwest::StatusCode;

use crate::db;

pub async fn fetch_spotify_endpoint() -> reqwest::Result<String> {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    let api_update_token =
        std::env::var("API_UPDATE_TOKEN").unwrap_or_else(|_| "beans".to_string());
    let client = reqwest::Client::new();
    let http_req = client
        .get(format!("http://{host}:{port}/api/spotify"))
        .bearer_auth(api_update_token)
        .send()
        .await?;
    Ok(match http_req.status() {
        StatusCode::OK => http_req.text().await.unwrap(),
        _ => String::new(),
    })
}

pub async fn verify_admin_authentication(
    session: &Session,
    pool: &web::Data<Arc<Pool>>,
) -> Result<bool, anyhow::Error> {
    let mut session_id = match session.get::<String>("session_id") {
        Ok(Some(id)) => id,
        _ => {
            log::debug!("Missing Session token");
            return Ok(false);
        }
    };
    log::debug!("ID: {}", session_id);
    if db::AdminSession::verify(&pool, session_id).await? {
        log::debug!("Authenticated");
        return Ok(true);
    } else {
        log::debug!("Unknown Session");
        return Ok(false);
    };
}
