use ::playlist_backend_lib::handlers;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http, App, HttpServer};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use env_logger::Env;
use r2d2_diesel::ConnectionManager;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::permissive()
                    // .send_wildcard()
                    // .allowed_methods(vec!["GET", "POST"])
                    // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    // .allowed_header(http::header::CONTENT_TYPE)
                    // .max_age(3600),
            )
            .data(pool.clone())
            .service(handlers::get_plays)
            .service(handlers::search)
            .service(handlers::month)
            .service(handlers::stats_last)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
