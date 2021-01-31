use crate::models::FullPlay;
use chrono::NaiveDate;
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
    pub previous_date: Option<NaiveDate>,
    pub next_date: Option<NaiveDate>,
    pub plays: Vec<FullPlay>,
}