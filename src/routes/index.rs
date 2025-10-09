use std::sync::Arc;

use actix_web::{get, web, HttpResponse};
use askama::Template;
use async_sqlite::Pool;

use crate::{templates::IndexTemplate, utils, AppState};

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
    let mut organised_experience = match state.store.lock().unwrap().get("experience") {
        Some(value) => {
            serde_json::from_str::<crate::utils::OrganisedExperience>(value.to_owned().as_str())
                .expect("JSON should be valid")
        }
        _ => crate::utils::OrganisedExperience { company: vec![] },
    };
    if organised_experience == (crate::utils::OrganisedExperience { company: vec![] }) {
        let tree = utils::calculate_experience_tree(&db_pool).await.unwrap();
        organised_experience =
            serde_json::from_str::<crate::utils::OrganisedExperience>(tree.to_owned().as_str())
                .unwrap();
        match state
            .store
            .lock()
            .unwrap()
            .insert("experience".to_string(), tree)
        {
            None => log::debug!("KV value updated"),
            Some(_) => log::debug!("KV value created"),
        };
    }
    let mut project = match state.store.lock().unwrap().get("project") {
        Some(value) => value.to_owned(),
        _ => String::new(),
    };
    if project == String::new() {
        project = utils::render_project_tree(&db_pool).await;
        match state
            .store
            .lock()
            .unwrap()
            .insert("project".to_string(), project.clone())
        {
            None => log::debug!("KV value updated"),
            Some(_) => log::debug!("KV value created"),
        };
    };
    let mut guestlog = match state.store.lock().unwrap().get("guestlog") {
        Some(value) => value.to_owned(),
        _ => String::new(),
    };
    if guestlog == String::new() {
        guestlog = utils::render_guestlog_entries(&db_pool).await;
        match state
            .store
            .lock()
            .unwrap()
            .insert("guestlog".to_string(), guestlog.clone())
        {
            None => log::debug!("KV value updated"),
            Some(_) => log::debug!("KV value created"),
        };
    };
    let html = IndexTemplate {
        title: "Sean Outram",
        spotify_widget: spotify,
        experiences: organised_experience
            .company
            .iter()
            .filter(|x| x.e_type == "Work")
            .cloned()
            .collect(),
        education: organised_experience
            .company
            .iter()
            .filter(|x| x.e_type == "Education")
            .cloned()
            .collect(),
        project,
        guestlog,
    };

    HttpResponse::Ok().body(html.render().expect("Template should be valid"))
}
