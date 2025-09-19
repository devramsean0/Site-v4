use std::sync::Arc;

use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpRequest, HttpResponse, Responder,
};
use askama::Template;
use async_sqlite::Pool;
use futures::StreamExt;

use crate::{
    assets,
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
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    mut payload: Multipart,
) -> HttpResponse {
    let mut form_data = AdminProjectNewProps::default();
    if !utils::verify_admin_authentication(&session, &db_pool)
        .await
        .unwrap_or(false)
    {
        return Redirect::to("/admin/login")
            .see_other()
            .respond_to(&request)
            .map_into_boxed_body();
    }
    while let Some(item) = payload.next().await {
        let mut field = item.unwrap();
        let name = field.name().unwrap_or_default().to_string();

        let mut value = Vec::new();
        while let Some(chunk) = field.next().await {
            value.extend_from_slice(&chunk.unwrap());
        }

        match name.as_str() {
            "name" => form_data.name = String::from_utf8_lossy(&value).to_string(),
            "description" => form_data.description = String::from_utf8_lossy(&value).to_string(),
            "src" => form_data.src = Some(String::from_utf8_lossy(&value).to_string()),
            "docs" => form_data.docs = Some(String::from_utf8_lossy(&value).to_string()),
            "demo" => form_data.demo = Some(String::from_utf8_lossy(&value).to_string()),
            "favourite" => form_data.favourite = String::from_utf8_lossy(&value).to_string(),
            "preview_img" => {
                // Save file to disk or buffer
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("upload.tmp");
                let filepath = assets::generate_filename(filename.to_string())
                    .await
                    .unwrap();
                log::info!("Generated filepath: {filepath}");
                std::fs::write(&filepath, &value).unwrap();
                form_data.preview_img = Some(filepath);
            }
            _ => {}
        }
    }

    db::Project::insert(
        &db_pool,
        Project {
            id: None,
            name: form_data.name,
            description: form_data.description,
            src: form_data.src,
            docs: form_data.docs,
            demo: form_data.demo,
            preview_img: form_data.preview_img,
            favourite: form_data.favourite.parse().unwrap_or(false),
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

#[derive(serde::Deserialize, Debug, Default)]
struct AdminProjectNewProps {
    name: String,
    description: String,
    src: Option<String>,
    docs: Option<String>,
    demo: Option<String>,
    preview_img: Option<String>,
    favourite: String,
}
