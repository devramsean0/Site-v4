use std::sync::Arc;

use crate::{
    db,
    templates::{AdminExperienceListTemplate, AdminExperienceNewTemplate},
    utils,
};
use actix_session::Session;
use actix_web::{
    get, post,
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
    if utils::verify_admin_authentication(&session, &db_pool)
        .await
        .unwrap_or_default()
        != true
    {
        return Redirect::to("/admin/login")
            .see_other()
            .respond_to(&request)
            .map_into_boxed_body();
    }
    let records = db::Experience::all(&db_pool).await.unwrap();
    let html = AdminExperienceListTemplate {
        title: "Experience | Admin",
        error: None,
        experiences: records,
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
    if utils::verify_admin_authentication(&session, &db_pool)
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

#[post("/experience")]
pub async fn experience_new_post(
    params: web::Form<AdminExperienceNewProps>,
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
) -> HttpResponse {
    log::info!("Form Submission Hit");
    if utils::verify_admin_authentication(&session, &db_pool)
        .await
        .unwrap_or_default()
        != true
    {
        return Redirect::to("/admin/login")
            .see_other()
            .respond_to(&request)
            .map_into_boxed_body();
    }
    db::Experience::insert(
        &db_pool,
        db::Experience {
            name: params.name.clone(),
            company: params.company.clone(),
            description: params.description.clone(),
            start_date: params.start_date.clone(),
            end_date: params.end_date.clone().unwrap_or_default(),
            e_type: params.e_type.clone(),
        },
    )
    .await
    .unwrap();
    Redirect::to("/admin/experience")
        .see_other()
        .respond_to(&request)
        .map_into_boxed_body()
}

#[derive(serde::Deserialize)]
struct AdminExperienceNewProps {
    name: String,
    company: String,
    description: String,
    start_date: String,
    end_date: Option<String>,
    e_type: String,
}
