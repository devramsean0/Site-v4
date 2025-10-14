use std::sync::Arc;

use actix_multipart::{form, Multipart};
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
    db::{self, Blog},
    templates::{AdminBlogEditTemplate, AdminBlogListTemplate, AdminBlogNewTemplate},
    ternary, utils,
    websocket_channel::ChannelsActor,
    AppState,
};

#[get("/blog")]
pub async fn blog_list(
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
    let records = db::Blog::all(&db_pool).await.unwrap();
    let html = AdminBlogListTemplate {
        title: "Blog | Admin",
        error: None,
        posts: records,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[get("/blog/new")]
pub async fn blog_new_get(
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
    let html = AdminBlogNewTemplate {
        title: "Blog | Admin",
        error: None,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[post("/blog")]
pub async fn blog_new_post(
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    _state: web::Data<AppState>,
    _channels: web::Data<actix::Addr<ChannelsActor>>,
    mut payload: Multipart,
) -> HttpResponse {
    let mut form_data = AdminBlogNewProps::default();
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
            "title" => form_data.title = String::from_utf8_lossy(&value).to_string(),
            "body" => form_data.body = String::from_utf8_lossy(&value).to_string(),
            "banner_img" => {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("upload.tmp");
                let filepath = assets::generate_filename(filename.to_string())
                    .await
                    .unwrap();
                std::fs::write(&filepath.path, &value).unwrap();
                form_data.banner_img = filepath.filename;
            }
            "published" => form_data.published = String::from_utf8_lossy(&value).to_string(),
            _ => {}
        }
    }
    form_data.published =
        ternary!(form_data.published == "on" => true.to_string(), false.to_string());
    let slug = form_data.title.replace(" ", "-").to_lowercase();
    db::Blog::insert(
        &db_pool,
        Blog {
            created_at: None,
            updated_at: None,
            id: None,
            slug: Some(slug).into(),
            title: form_data.title,
            body: form_data.body,
            banner_img: form_data.banner_img.into(),
            published: form_data.published.parse().unwrap_or(false),
        },
    )
    .await
    .unwrap();

    Redirect::to("/admin/blog")
        .see_other()
        .respond_to(&request)
        .map_into_boxed_body()
}

#[delete("/blog/{id}")]
pub async fn blog_delete(
    params: web::Path<BlogSelectProps>,
    _state: web::Data<AppState>,
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    _channels: web::Data<actix::Addr<ChannelsActor>>,
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
    db::Blog::delete(&db_pool, params.into_inner())
        .await
        .unwrap();
    HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
}

#[get("/blog/edit/{id}")]
pub async fn blog_edit_get(
    params: web::Path<BlogSelectProps>,
    _state: web::Data<AppState>,
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    _channels: web::Data<actix::Addr<ChannelsActor>>,
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
    let blog = db::Blog::get_by_id(params.id.clone(), &db_pool)
        .await
        .unwrap()
        .unwrap();
    let html = AdminBlogEditTemplate {
        title: "Edit Blog | Admin",
        error: None,
        blog: blog,
    }
    .render()
    .expect("Template should be valid");

    HttpResponse::Ok().body(html)
}

#[post("/blog/edit/{id}")]
pub async fn blog_edit_post(
    params: web::Path<BlogSelectProps>,
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    _state: web::Data<AppState>,
    _channels: web::Data<actix::Addr<ChannelsActor>>,
    mut payload: Multipart,
) -> HttpResponse {
    let record = db::Blog::get_by_id(params.id.clone(), &db_pool)
        .await
        .unwrap()
        .unwrap();
    let mut form_data = AdminBlogNewProps::default();
    form_data.banner_img = record.banner_img.unwrap();
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
            "title" => form_data.title = String::from_utf8_lossy(&value).to_string(),
            "body" => form_data.body = String::from_utf8_lossy(&value).to_string(),
            "banner_img" => {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("upload.tmp");
                let filepath = assets::generate_filename(filename.to_string())
                    .await
                    .unwrap();
                std::fs::write(&filepath.path, &value).unwrap();
                form_data.banner_img = filepath.filename;
            }
            "published" => form_data.published = String::from_utf8_lossy(&value).to_string(),
            _ => {}
        }
    }
    form_data.published =
        ternary!(form_data.published == "on" => true.to_string(), false.to_string());
    println!("{:#?}", form_data);
    db::Blog::update(
        &db_pool,
        Blog {
            created_at: None,
            updated_at: None,
            id: Some(params.id.parse().unwrap()),
            slug: None.into(),
            title: form_data.title,
            body: form_data.body,
            banner_img: form_data.banner_img.into(),
            published: form_data.published.parse().unwrap_or(false),
        },
    )
    .await
    .unwrap();
    Redirect::to("/admin/blog")
        .see_other()
        .respond_to(&request)
        .map_into_boxed_body()
}

#[derive(serde::Deserialize, Debug, Default)]
struct AdminBlogNewProps {
    title: String,
    body: String,
    banner_img: String,
    published: String,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct BlogSelectProps {
    pub id: String,
}
