use ::playlist_backend_lib::handlers;
use actix_web::{web, App, HttpServer};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/plays").route(web::get().to(handlers::get_plays)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
