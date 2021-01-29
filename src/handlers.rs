use crate::db;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;

pub async fn get_plays(pool: web::Data<db::Pool>) -> impl Responder {
    db::get_full_plays(pool)
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|e| HttpResponse::InternalServerError().body(format!("{}", e)))
}
