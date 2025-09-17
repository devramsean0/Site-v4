use std::sync::Arc;

use crate::{
    db,
    templates::{
        AdminExperienceEditTemplate, AdminExperienceListTemplate, AdminExperienceNewTemplate,
    },
    utils, AppState,
};
use actix_session::Session;
use actix_web::{
    delete, get,
    http::StatusCode,
    post,
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
    let records = db::Experience::all(&db_pool, false).await.unwrap();
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
    state: web::Data<AppState>,
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
    db::Experience::insert(
        &db_pool,
        db::Experience {
            id: None,
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
    let tree = utils::calculate_experience_tree(&db_pool).await.unwrap();
    match state
        .store
        .lock()
        .unwrap()
        .insert("experience".to_string(), tree)
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    Redirect::to("/admin/experience")
        .see_other()
        .respond_to(&request)
        .map_into_boxed_body()
}

#[delete("/experience/{id}")]
pub async fn experience_delete(
    params: web::Path<AdminExperienceSelectProps>,
    state: web::Data<AppState>,
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
    db::Experience::delete(&db_pool, params.into_inner())
        .await
        .unwrap();
    let tree = utils::calculate_experience_tree(&db_pool).await.unwrap();
    match state
        .store
        .lock()
        .unwrap()
        .insert("experience".to_string(), tree)
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
}

#[get("/experience/edit/{id}")]
pub async fn experience_edit_get(
    params: web::Path<AdminExperienceSelectProps>,
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
    let experience = db::Experience::get_by_id(params.id.clone(), &db_pool)
        .await
        .unwrap()
        .unwrap();
    let html = AdminExperienceEditTemplate {
        title: "Experience | Admin",
        error: None,
        e_type: vec![
            "Work Experience - Part Time",
            "Volunteer",
            "Contract",
            "Education",
        ],
        experience,
    }
    .render()
    .expect("Template Should be valid");
    HttpResponse::Ok().body(html)
}

#[post("/experience/edit/{id}")]
pub async fn experience_edit_post(
    params: web::Path<AdminExperienceSelectProps>,
    form: web::Form<AdminExperienceNewProps>,
    state: web::Data<AppState>,
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
    db::Experience::update(
        &db_pool,
        db::Experience {
            id: Some(params.id.parse::<i64>().unwrap()),
            name: form.name.clone(),
            company: form.company.clone(),
            description: form.description.clone(),
            start_date: form.start_date.clone(),
            end_date: form.end_date.clone().unwrap_or_default(),
            e_type: form.e_type.clone(),
        },
    )
    .await
    .unwrap();
    let tree = utils::calculate_experience_tree(&db_pool).await.unwrap();
    match state
        .store
        .lock()
        .unwrap()
        .insert("experience".to_string(), tree)
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
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

#[derive(serde::Deserialize)]
pub struct AdminExperienceSelectProps {
    pub id: String,
}
