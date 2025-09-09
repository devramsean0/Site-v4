use actix_web::{get, post, web, HttpResponse};
use askama::Template;
use async_sqlite::Pool;
use serde::Deserialize;

use crate::{db::AdminUser, templates::AdminLoginTemplate};

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
    params: web::Form<AdminLoginProps>,
    db_pool: web::Data<Pool>,
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
            HttpResponse::Ok().body("Hello")
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
