use crate::api::Error;
use crate::db;
use actix_web::{get, web, HttpResponse, Responder};
use chrono::NaiveDate;

#[get("/{station}/{date:[0-9]{4}-[0-9]{2}-[0-9]{2}}")]
pub async fn get_plays(
    pool: web::Data<db::Pool>,
    web::Path(params): web::Path<(String, String)>,
) -> impl Responder {
    let station = params.0;
    let date = NaiveDate::parse_from_str(&params.1, "%Y-%m-%d").map_err(|e| {
        HttpResponse::BadRequest().json(Error {
            error: true,
            message: format!("Date is invalid: {}", e),
        })
    })?;

    db::get_full_plays(pool, &station, &date)
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|e| {
            let error = Error {
                error: true,
                message: format!("Could not load plays: {}", e),
            };
            HttpResponse::InternalServerError().json(error)
        })
}

#[get("/{station}/around/{date:[0-9]{4}-[0-9]{2}-[0-9]{2}}")]
pub async fn get_around(
    pool: web::Data<db::Pool>,
    web::Path(params): web::Path<(String, String)>,
) -> impl Responder {
    let station = params.0;
    let date = NaiveDate::parse_from_str(&params.1, "%Y-%m-%d").map_err(|e| {
        HttpResponse::BadRequest().json(Error {
            error: true,
            message: format!("Date is invalid: {}", e),
        })
    })?;

    db::get_full_plays(pool, &station, &date)
        .map(|users| HttpResponse::Ok().json(users))
        .map_err(|e| {
            let error = Error {
                error: true,
                message: format!("Could not load plays: {}", e),
            };
            HttpResponse::InternalServerError().json(error)
        })
}
