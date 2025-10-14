use actix_web::{get, HttpResponse};
use askama::Template;

use crate::templates::AdminOptionsTemplate;

pub mod authentication;
pub mod blog;
pub mod experience;
pub mod guestlog;
pub mod projects;

#[get("/")]
pub async fn admin_get() -> HttpResponse {
    let html = AdminOptionsTemplate {
        title: "Admin",
        error: None,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}
