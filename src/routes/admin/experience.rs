use std::sync::Arc;

use crate::{db, templates::AdminExperienceListTemplate};
use actix_session::Session;
use actix_web::{
    get,
    web::{self, Redirect},
    HttpRequest, HttpResponse, Responder,
};
use askama::Template;
use async_sqlite::Pool;

#[get("/admin/experience")]
pub async fn experience_list(
    request: HttpRequest,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
) -> HttpResponse {
    let session_id = match session.get::<String>("session_id") {
        Ok(Some(id)) => id,
        _ => {
            log::debug!("Redirecting for unknown cookie");
            return Redirect::to("/admin/login")
                .see_other()
                .respond_to(&request)
                .map_into_boxed_body();
        }
    };
    log::debug!("Session ID: {session_id}");
    if db::AdminSession::verify(&db_pool, session_id)
        .await
        .unwrap()
        != true
    {
        log::debug!("Redirecting for non-match");
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
