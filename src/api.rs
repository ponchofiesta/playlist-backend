use chrono::NaiveDate;
use crate::models::FullPlay;
// use chrono::NaiveDate;
use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    pub error: bool,
    pub message: String,
}

impl Error {
    pub fn new(message: &str) -> Self {
        Error {
            error: true,
            message: message.into(),
        }
    }
}

#[derive(Serialize)]
pub struct PlaysDay {
    // pub previous_date: Option<NaiveDate>,
    // pub next_date: Option<NaiveDate>,
    pub plays: Vec<FullPlay>,
}

#[derive(Serialize)]
pub struct SearchResults {
    // pub previous_date: Option<NaiveDate>,
    // pub next_date: Option<NaiveDate>,
    pub plays: Vec<FullPlay>,
}

#[derive(Serialize)]
pub struct Month {
    pub days: Vec<Day>,
}

#[derive(Serialize, Queryable)]
pub struct Day {
    pub date: NaiveDate,
    pub songs_count: u32,
}
