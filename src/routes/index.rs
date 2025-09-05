use actix_web::{get, HttpResponse};
use askama::Template;

use crate::templates::IndexTemplate;

#[get("/")]
pub async fn index_get() -> HttpResponse {
    let html = IndexTemplate {
        title: "Sean Outram",
    }
    .render()
    .expect("Template should be valid");

    HttpResponse::Ok().body(html)
}
