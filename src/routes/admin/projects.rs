use std::sync::Arc;

use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpRequest, HttpResponse, Responder,
};
use askama::Template;
use async_sqlite::Pool;

use crate::{
    db::{self, Project},
    templates::{AdminProjectListTemplate, AdminProjectNewTemplate},
    utils,
};

#[get("/project")]
pub async fn project_list(
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
    let records = db::Project::all(&db_pool).await.unwrap();
    let html = AdminProjectListTemplate {
        title: "Project | Admin",
        error: None,
        projects: records,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[get("/project/new")]
pub async fn project_new_get(
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
    let html = AdminProjectNewTemplate {
        title: "Project | Admin",
        error: None,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[post("/project")]
pub async fn project_new_post(
    request: HttpRequest,
    params: web::Form<AdminProjectNewProps>,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
) -> HttpResponse {
    println!("{:#?}", params);
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
    db::Project::insert(
        &db_pool,
        Project {
            id: None,
            name: params.name.clone(),
            description: params.description.clone(),
            src: params.src.clone(),
            docs: params.docs.clone(),
            demo: params.demo.clone(),
            preview_img: params.preview_img.clone(),
            favourite: params.favourite.parse().unwrap(),
            technologies: vec![],
        },
    )
    .await
    .unwrap();
    Redirect::to("/admin/project")
        .see_other()
        .respond_to(&request)
        .map_into_boxed_body()
}

#[derive(serde::Deserialize, Debug)]
struct AdminProjectNewProps {
    name: String,
    description: String,
    src: Option<String>,
    docs: Option<String>,
    demo: Option<String>,
    preview_img: Option<String>,
    favourite: String,
}
