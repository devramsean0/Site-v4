use actix_web::{get, http::StatusCode, web, HttpResponse};
use askama::Template;
use async_sqlite::Pool;
use std::sync::Arc;

use crate::{
    db,
    templates::{BlogListTemplate, BlogSlugTemplate},
};

#[get("/blog")]
pub async fn blog_list(db_pool: web::Data<Arc<Pool>>) -> HttpResponse {
    let articles = db::Blog::published(&db_pool).await.unwrap();
    let html = BlogListTemplate {
        title: "Blog Articles",
        articles: articles,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[get("/blog/{slug}")]
pub async fn blog_get(
    db_pool: web::Data<Arc<Pool>>,
    params: web::Path<BlogGetParam>,
) -> HttpResponse {
    let article = match db::Blog::get_by_slug(params.slug.clone(), &db_pool)
        .await
        .unwrap()
    {
        Some(article) => article,
        None => {
            return HttpResponse::Ok().status(StatusCode::NOT_FOUND).finish();
        }
    };
    let article_contents =
        markdown::to_html_with_options(article.body.as_str(), &markdown::Options::gfm()).unwrap();
    let html = BlogSlugTemplate {
        title: article.clone().title.as_str(),
        article,
        article_contents,
    }
    .render()
    .expect("template should be valid");
    HttpResponse::Ok().body(html)
}

#[derive(serde::Deserialize)]
struct BlogGetParam {
    slug: String,
}
