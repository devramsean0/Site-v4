use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer};
use async_sqlite::PoolBuilder;
use std::{
    io::{Error, ErrorKind},
    sync::Arc,
};

mod db;
mod routes;
mod templates;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    let db_url = std::env::var("DB_URL").unwrap_or_else(|_| String::from("./db.sqlite3"));

    // Create DB pool
    let pool = match PoolBuilder::new().path(db_url).open().await {
        Ok(pool) => {
            log::info!("Established DB pool");
            pool
        }
        Err(e) => {
            log::error!("Error estalishing DB pool {e}");
            return Err(Error::new(
                ErrorKind::Other,
                "database pool could not be established",
            ));
        }
    };

    match db::create_tables(&pool).await {
        Ok(_) => log::info!("DB migrations ran"),
        Err(err) => log::error!("Database migration error {err}"),
    };

    let db_pool_arc = Arc::new(pool.clone());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(Files::new("compiled_assets/", "compiled_assets/"))
            .service(Files::new("assets/", "assets/"))
            .app_data(web::Data::new(db_pool_arc.clone()))
            .service(routes::index::index_get)
            .service(routes::api::api_spotify_get)
    })
    .bind((host.clone(), port))?
    .run()
    .await
}
