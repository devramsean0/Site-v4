use std::sync::Arc;

use actix_multipart::Multipart;
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
use futures::StreamExt;

use crate::{
    assets,
    db::{self, Project},
    templates::{AdminProjectEditTemplate, AdminProjectListTemplate, AdminProjectNewTemplate},
    ternary, utils,
    websocket_channel::{ChannelsActor, Publish},
    AppState,
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
    state: web::Data<AppState>,
    channels: web::Data<actix::Addr<ChannelsActor>>,

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
                std::fs::write(&filepath.path, &value).unwrap();
                form_data.preview_img = Some(filepath.filename);
            }
            "technologies" => {
                let raw = String::from_utf8_lossy(&value).to_string();
                let split = raw
                    .split(',')
                    .map(|val| val.trim().to_string())
                    .filter(|val| !val.is_empty())
                    .collect::<Vec<String>>();
                form_data.technologies = split
            }
            _ => {}
        }
    }
    form_data.favourite =
        ternary!(form_data.favourite == "on" => true.to_string(), false.to_string()); // Hack because forms suck :(
    db::Project::insert(
        &db_pool,
        Project {
            id: None,
            name: form_data.name,
            description: form_data.description,
            src: form_data.src.into(),
            docs: form_data.docs.into(),
            demo: form_data.demo.into(),
            preview_img: form_data.preview_img.into(),
            favourite: form_data.favourite.parse().unwrap_or(false),
            technologies: form_data.technologies,
        },
    )
    .await
    .unwrap();
    let projects = utils::render_project_tree(&db_pool).await;
    match state
        .store
        .lock()
        .unwrap()
        .insert("project".to_string(), projects.clone())
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    channels.do_send(Publish {
        channel: "project".to_string(),
        payload: projects,
    });
    Redirect::to("/admin/project")
        .see_other()
        .respond_to(&request)
        .map_into_boxed_body()
}

#[delete("/project/{id}")]
pub async fn project_delete(
    params: web::Path<AdminProjectSelectProps>,
    state: web::Data<AppState>,
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    channels: web::Data<actix::Addr<ChannelsActor>>,
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
    db::Project::delete(&db_pool, params.into_inner())
        .await
        .unwrap();
    let projects = utils::render_project_tree(&db_pool).await;
    match state
        .store
        .lock()
        .unwrap()
        .insert("project".to_string(), projects.clone())
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    channels.do_send(Publish {
        channel: "project".to_string(),
        payload: projects,
    });
    HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
}

#[get("/project/edit/{id}")]
pub async fn project_edit_get(
    params: web::Path<AdminProjectSelectProps>,
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
    let record = db::Project::get_by_id(params.id.clone(), &db_pool)
        .await
        .unwrap()
        .unwrap();
    let html = AdminProjectEditTemplate {
        title: "Project | Admin",
        error: None,
        project: record,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[post("/project/edit/{id}")]
pub async fn project_edit_post(
    params: web::Path<AdminProjectSelectProps>,
    state: web::Data<AppState>,
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    channels: web::Data<actix::Addr<ChannelsActor>>,
    mut payload: Multipart,
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
    let mut form_data: AdminProjectNewProps = AdminProjectNewProps::default();
    let record = db::Project::get_by_id(params.id.clone(), &db_pool)
        .await
        .unwrap()
        .unwrap();
    form_data.preview_img = record.preview_img.into_option();
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
                if String::from_utf8_lossy(&value).to_string() != String::new() {
                    // Save file to disk or buffer
                    let filename = field
                        .content_disposition()
                        .and_then(|cd| cd.get_filename())
                        .unwrap_or("upload.tmp");
                    let filepath = assets::generate_filename(filename.to_string())
                        .await
                        .unwrap();
                    std::fs::write(&filepath.path, &value).unwrap();
                    form_data.preview_img = Some(filepath.filename);
                }
            }
            "technologies" => {
                let raw = String::from_utf8_lossy(&value).to_string();
                let split = raw
                    .split(',')
                    .map(|val| val.trim().to_string())
                    .filter(|val| !val.is_empty())
                    .collect::<Vec<String>>();
                println!("Split: {:#?}", split);
                form_data.technologies = split
            }
            _ => {}
        }
    }
    form_data.favourite =
        ternary!(form_data.favourite == "on" => true.to_string(), false.to_string()); // Hack because forms suck :(
    db::Project::update(
        &db_pool,
        Project {
            id: Some(params.id.parse().unwrap()),
            name: form_data.name,
            description: form_data.description,
            src: form_data.src.into(),
            docs: form_data.docs.into(),
            demo: form_data.demo.into(),
            preview_img: form_data.preview_img.into(),
            favourite: form_data.favourite.parse().unwrap_or(false),
            technologies: form_data.technologies,
        },
    )
    .await
    .unwrap();
    let projects = utils::render_project_tree(&db_pool).await;
    match state
        .store
        .lock()
        .unwrap()
        .insert("project".to_string(), projects.clone())
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    channels.do_send(Publish {
        channel: "project".to_string(),
        payload: projects,
    });
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
    technologies: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct AdminProjectSelectProps {
    pub id: String,
}
