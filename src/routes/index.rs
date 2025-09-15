use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use askama::Template;
use async_sqlite::Pool;

use crate::{templates::IndexTemplate, AppState};

#[get("/")]
pub async fn index_get(state: web::Data<AppState>, db_pool: web::Data<Arc<Pool>>) -> HttpResponse {
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
    let experience = crate::db::Experience::all(&db_pool).await.unwrap();
    let html = IndexTemplate {
        title: "Sean Outram",
        spotify_widget: spotify,
        experiences: experience
            .iter()
            .filter(|x| x.e_type != "education")
            .cloned()
            .collect(),
        education: experience
            .iter()
            .filter(|x| x.e_type == "education")
            .cloned()
            .collect(),
    }
    .render()
    .expect("Template should be valid");

    HttpResponse::Ok().body(html)
}
