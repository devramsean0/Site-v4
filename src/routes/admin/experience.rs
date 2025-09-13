use std::sync::Arc;

use crate::{
    templates::{AdminExperienceListTemplate, AdminExperienceNewTemplate},
    utils,
};
use actix_session::Session;
use actix_web::{
    get,
    web::{self, Redirect},
    HttpRequest, HttpResponse, Responder,
};
use askama::Template;
use async_sqlite::Pool;

#[get("/experience")]
pub async fn experience_list(
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
) -> HttpResponse {
    if utils::verify_admin_authentication(&session, db_pool)
        .await
        .unwrap_or_default()
        != true
    {
        return Redirect::to("/admin/login")
            .see_other()
            .respond_to(&request)
            .map_into_boxed_body();
    }
    let html = AdminExperienceListTemplate {
        title: "Experience | Admin",
        error: None,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[get("/experience/new")]
pub async fn experience_new_get(
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
) -> HttpResponse {
    if utils::verify_admin_authentication(&session, db_pool)
        .await
        .unwrap_or_default()
        != true
    {
        return Redirect::to("/admin/login")
            .see_other()
            .respond_to(&request)
            .map_into_boxed_body();
    }
    let html = AdminExperienceNewTemplate {
        title: "Experience | Admin",
        error: None,
        e_type: vec![
            "Work Experience - Part Time",
            "Volunteer",
            "Contract",
            "Education",
        ],
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}
