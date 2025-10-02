use std::sync::Arc;

use crate::{
    assets,
    db::{self, Guestlog},
    templates::AdminGuestlogListTemplate,
    utils,
    websocket_channel::{ChannelsActor, Publish},
    AppState,
};
use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{
    delete, get,
    http::StatusCode,
    post, put,
    web::{self, Redirect},
    HttpRequest, HttpResponse, Responder,
};
use askama::Template;
use async_sqlite::Pool;
use futures::StreamExt;

#[get("/guestlog")]
pub async fn guestlog_list(
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
    let records = db::Guestlog::all(&db_pool).await.unwrap();
    let html = AdminGuestlogListTemplate {
        title: "Guestlog | Admin",
        error: None,
        guestlogs: records,
    }
    .render()
    .expect("template should be valid");

    HttpResponse::Ok().body(html)
}

#[post("/guestlog")]
pub async fn experience_new_post(
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
    state: web::Data<AppState>,
    channels: web::Data<actix::Addr<ChannelsActor>>,
    mut payload: Multipart,
) -> HttpResponse {
    let mut form_data = GuestlogNewProps::default();
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
        println!("Payload Name: {name}");
        let mut value = Vec::new();
        while let Some(chunk) = field.next().await {
            value.extend_from_slice(&chunk.unwrap());
        }
        match name.as_str() {
            "name" => form_data.name = String::from_utf8_lossy(&value).to_string(),
            "email" => form_data.email = String::from_utf8_lossy(&value).to_string(),
            "message" => form_data.message = String::from_utf8_lossy(&value).to_string(),
            "avatar" => {
                let filename = field
                    .content_disposition()
                    .and_then(|cd| cd.get_filename())
                    .unwrap_or("upload.tmp");
                let filepath = assets::generate_filename(filename.to_string())
                    .await
                    .unwrap();
                std::fs::write(&filepath.path, &value).unwrap();
                form_data.avatar = Some(filepath.filename);
            }
            _ => {}
        }
    }
    println!("Guestlog: {:#?}", form_data);
    db::Guestlog::insert(
        &db_pool,
        Guestlog {
            id: None,
            name: form_data.clone().name,
            email: form_data.clone().email,
            message: form_data.clone().message,
            avatar: form_data.clone().avatar.into(),
            active: true,
        },
    )
    .await
    .unwrap();
    let guestlog = utils::render_guestlog_entries(&db_pool).await;
    match state
        .store
        .lock()
        .unwrap()
        .insert("guestlog".to_string(), guestlog.clone())
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    channels.do_send(Publish {
        channel: "project".to_string(),
        payload: guestlog,
    });
    HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
}

#[delete("/guestlog/{id}")]
pub async fn guestlog_delete(
    params: web::Path<GuestlogSelectProps>,
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
    db::Guestlog::delete(&db_pool, params.into_inner())
        .await
        .unwrap();
    let tree = utils::render_guestlog_entries(&db_pool).await;
    match state
        .store
        .lock()
        .unwrap()
        .insert("guestlog".to_string(), tree)
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
}

#[put("/guestlog/{id}/activestate")]
pub async fn guestlog_activestate(
    params: web::Path<GuestlogSelectProps>,
    state: web::Data<AppState>,
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
) -> HttpResponse {
    if !utils::verify_admin_authentication(&session, &db_pool)
        .await
        .unwrap_or(false)
    {
        return Redirect::to("/admin/login")
            .see_other()
            .respond_to(&request)
            .map_into_boxed_body();
    }
    db::Guestlog::set_activestate(&db_pool, params.id.clone())
        .await
        .unwrap();
    let tree = utils::render_guestlog_entries(&db_pool).await;
    match state
        .store
        .lock()
        .unwrap()
        .insert("guestlog".to_string(), tree)
    {
        None => log::debug!("KV value updated"),
        Some(_) => log::debug!("KV value created"),
    };
    HttpResponse::Ok().status(StatusCode::NO_CONTENT).finish()
}
#[derive(serde::Deserialize, Debug, Default, Clone)]
struct GuestlogNewProps {
    name: String,
    email: String,
    avatar: Option<String>,
    message: String,
}

#[derive(serde::Deserialize, Debug, Default, Clone)]
pub struct GuestlogSelectProps {
    pub id: String,
}
