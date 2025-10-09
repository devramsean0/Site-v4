use std::sync::Arc;

use actix_session::Session;
use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpRequest, HttpResponse, Responder,
};
use askama::Template;
use async_sqlite::Pool;
use serde::Deserialize;

use crate::{
    db::{AdminSession, AdminUser},
    templates::AdminLoginTemplate,
};

#[get("/admin/login")]
pub async fn admin_login_get() -> HttpResponse {
    let html = AdminLoginTemplate {
        title: "Sign In | Admin",
        error: None,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[post("/admin/login")]
pub async fn admin_login_post(
    request: HttpRequest,
    params: web::Form<AdminLoginProps>,
    db_pool: web::Data<Arc<Pool>>,
    session: Session,
) -> HttpResponse {
    let user = AdminUser::find_by_email(params.email.clone(), &db_pool).await;
    match user {
        Ok(user) => {
            let Some(admin_user) = user else {
                let html = AdminLoginTemplate {
                    title: "Sign In | Admin",
                    error: Some("No Account found"),
                }
                .render()
                .expect("template should be valid");
                return HttpResponse::Ok().body(html);
            };
            if admin_user.password == params.password {
                let db_session = AdminSession::new();
                session
                    .insert("session_id", db_session.session.clone())
                    .unwrap();
                db_session.insert(&db_pool).await.unwrap();
                Redirect::to("/")
                    .see_other()
                    .respond_to(&request)
                    .map_into_boxed_body()
            } else {
                let html = AdminLoginTemplate {
                    title: "Sign In | Admin",
                    error: Some("No Password"),
                }
                .render()
                .expect("template should be valid");
                return HttpResponse::Ok().body(html);
            }
        }
        Err(err) => {
            log::error!("Error getting AdminUser: {err}");
            let html = AdminLoginTemplate {
                title: "Sign In | Admin",
                error: Some("No Account found"),
            }
            .render()
            .expect("template should be valid");
            HttpResponse::Ok().body(html)
        }
    }
}

#[derive(Deserialize)]
struct AdminLoginProps {
    email: String,
    password: String,
}
