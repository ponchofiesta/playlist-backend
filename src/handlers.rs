use actix_http::ResponseBuilder;
use crate::api;
use crate::db;
use actix_web::{get, http, web, HttpResponse};
use chrono::NaiveDate;
use diesel::MysqlConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use serde::Deserialize;
use std::fmt::Display;

type HttpResult = Result<HttpResponse, HttpResponse>;

fn get_connection(
    pool: &db::Pool,
) -> Result<PooledConnection<ConnectionManager<MysqlConnection>>, HttpResponse> {
    let connection = pool.get().or_httperror(
        http::StatusCode::INTERNAL_SERVER_ERROR,
        "Database connection failed: {}",
    )?;
    Ok(connection)
}

fn parse_date(date: &str) -> Result<NaiveDate, HttpResponse> {
    let date = NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .or_httperror(http::StatusCode::BAD_REQUEST, "Date is invalid: {}")?;
    Ok(date)
}

#[get("/{station}/{date:[0-9]{4}-[0-9]{2}-[0-9]{2}}")]
pub async fn get_plays(
    pool: web::Data<db::Pool>,
    web::Path((station, date)): web::Path<(String, String)>,
) -> HttpResult {
    let connection = get_connection(pool.as_ref())?;
    let date = parse_date(&date)?;
    // let previous_date = db::get_previous_day(&connection, &station, &date).map_err(|e| {
    //     HttpResponse::InternalServerError()
    //         .json(Error::new(&format!("Could not load previous date: {}", e)))
    // })?;
    // let next_date = db::get_next_day(&connection, &station, &date).map_err(|e| {
    //     HttpResponse::InternalServerError()
    //         .json(Error::new(&format!("Could not load next date: {}", e)))
    // })?;
    let plays = db::get_full_plays(&connection, &station, &date).or_httperror(
        http::StatusCode::INTERNAL_SERVER_ERROR,
        "Could not load plays: {}",
    )?;
    Ok(HttpResponse::Ok().json(api::PlaysDay {
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
    let connection = get_connection(pool.as_ref())?;
    let plays = db::search(&connection, &station, &params).or_httperror(
        http::StatusCode::INTERNAL_SERVER_ERROR,
        "Could not search: {}",
    )?;
    Ok(HttpResponse::Ok().json(api::SearchResults { plays: plays }))
}

#[get("/{station}/month/{date:[0-9]{4}-[0-9]{2}-[0-9]{2}}")]
pub async fn month(
    pool: web::Data<db::Pool>,
    web::Path((station, date)): web::Path<(String, String)>,
) -> HttpResult {
    let connection = get_connection(pool.as_ref())?;
    let date = parse_date(&date)?;
    let days = db::get_month(&connection, &station, &date).or_httperror(
        http::StatusCode::INTERNAL_SERVER_ERROR,
        "Could not get month: {}",
    )?;
    Ok(HttpResponse::Ok().json(api::Month { days: days }))
}

#[get("/{station}/stats/{date:[0-9]{4}-[0-9]{2}-[0-9]{2}}/last/{last}")]
pub async fn stats_last(
    pool: web::Data<db::Pool>,
    web::Path((station, date, last)): web::Path<(String, String, i8)>,
) -> HttpResult {
    let connection = get_connection(pool.as_ref())?;
    let date = parse_date(&date)?;
    let stats = db::stats_last(&connection, &station, &date, last).or_httperror(
        http::StatusCode::INTERNAL_SERVER_ERROR,
        "Could not get stats: {}",
    )?;
    Ok(HttpResponse::Ok().json(stats))
}

use diesel;
trait HttpError<T, S> {
    fn or_httperror(
        self,
        code: actix_web::http::StatusCode,
        message: &str,
    ) -> Result<T, HttpResponse>;
}
impl<T, S> HttpError<T, S> for Result<T, S>
where
    S: Display,
{
    fn or_httperror(
        self,
        code: actix_web::http::StatusCode,
        message: &str,
    ) -> Result<T, HttpResponse> {
        match self {
            Ok(value) => Ok(value),
            Err(e) => Err(ResponseBuilder::new(code)
                .json(api::Error::new(&message.replace("{}", &format!("{}", e))))),
        }
    }
}
