use actix_web::{get, web, HttpResponse};
use askama::Template;

use crate::{templates::IndexTemplate, AppState};

#[get("/")]
pub async fn index_get(state: web::Data<AppState>) -> HttpResponse {
    let mut spotify = match state.store.lock().unwrap().get("spotify") {
        Some(value) => value.to_owned(),
        _ => String::new(),
    };
    if spotify == String::new() {
        // We need to fetch the status and then use the KV again
        crate::utils::fetch_spotify_endpoint()
            .await
            .unwrap_or_default();
        spotify = match state.store.lock().unwrap().get("spotify") {
            Some(value) => value.to_owned(),
            _ => String::new(),
        };
    }
    let html = IndexTemplate {
        title: "Sean Outram",
        spotify_widget: spotify,
    }
    .render()
    .expect("Template should be valid");

    HttpResponse::Ok().body(html)
}
