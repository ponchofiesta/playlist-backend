use crate::api::Error;
use crate::api::Month;
use crate::api::PlaysDay;
use crate::api::SearchResults;
use crate::db;
use actix_web::{get, web, HttpResponse};
use chrono::NaiveDate;
use serde::Deserialize;

type HttpResult = Result<HttpResponse, HttpResponse>;

#[get("/{station}/{date:[0-9]{4}-[0-9]{2}-[0-9]{2}}")]
pub async fn get_plays(
    pool: web::Data<db::Pool>,
    web::Path((station, date)): web::Path<(String, String)>,
) -> HttpResult {
    let connection = pool.get().map_err(|e| {
        HttpResponse::InternalServerError()
            .json(Error::new(&format!("Database connection failed: {}", e)))
    })?;
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| {
        HttpResponse::BadRequest().json(Error::new(&format!("Date is invalid: {}", e)))
    })?;
    // let previous_date = db::get_previous_day(&connection, &station, &date).map_err(|e| {
    //     HttpResponse::InternalServerError()
    //         .json(Error::new(&format!("Could not load previous date: {}", e)))
    // })?;
    // let next_date = db::get_next_day(&connection, &station, &date).map_err(|e| {
    //     HttpResponse::InternalServerError()
    //         .json(Error::new(&format!("Could not load next date: {}", e)))
    // })?;
    let plays = db::get_full_plays(&connection, &station, &date).map_err(|e| {
        HttpResponse::InternalServerError()
            .json(Error::new(&format!("Could not load plays: {}", e)))
    })?;
    Ok(HttpResponse::Ok().json(PlaysDay {
        // previous_date: previous_date,
        // next_date: next_date,
        plays: plays,
    }))
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub advanced: Option<bool>,
    pub term: Option<String>,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub date_from: Option<NaiveDate>,
    pub date_to: Option<NaiveDate>,
    pub grouping: Option<bool>,
}

#[get("/{station}/search")]
pub async fn search(
    pool: web::Data<db::Pool>,
    web::Path(station): web::Path<String>,
    params: web::Query<SearchParams>,
) -> HttpResult {
    let connection = pool.get().map_err(|e| {
        HttpResponse::InternalServerError()
            .json(Error::new(&format!("Database connection failed: {}", e)))
    })?;
    let plays = db::search(&connection, &station, &params).map_err(|e| {
        HttpResponse::InternalServerError().json(Error::new(&format!("Could not search: {}", e)))
    })?;
    Ok(HttpResponse::Ok().json(SearchResults { plays: plays }))
}

#[get("/{station}/month/{date:[0-9]{4}-[0-9]{2}-[0-9]{2}}")]
pub async fn month(
    pool: web::Data<db::Pool>,
    web::Path((station, date)): web::Path<(String, String)>,
) -> HttpResult {
    let connection = pool.get().map_err(|e| {
        HttpResponse::InternalServerError()
            .json(Error::new(&format!("Database connection failed: {}", e)))
    })?;
    let date = NaiveDate::parse_from_str(&date, "%Y-%m-%d").map_err(|e| {
        HttpResponse::BadRequest().json(Error::new(&format!("Date is invalid: {}", e)))
    })?;
    let days = db::get_month(&connection, &station, &date).map_err(|e| {
        HttpResponse::InternalServerError().json(Error::new(&format!("Could not get month: {}", e)))
    })?;
    Ok(HttpResponse::Ok().json(Month { days: days }))
}
