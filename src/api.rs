use chrono::{NaiveDate, NaiveDateTime};
use crate::models::{Artist, Song, Play};
// use crate::models::FullPlay;
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
pub struct FullSong {
    pub id: u64,
    pub artist: Artist,
    pub title: String,
    pub cover_url: Option<String>,
    pub cover_width: Option<u16>,
    pub cover_height: Option<u16>,
    pub asin: Option<String>,
    pub last_cover_check: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct FullPlay {
    pub id: u64,
    pub song: FullSong,
    pub date: NaiveDateTime,
    pub station_id: u64,
}

impl FullPlay {
    pub fn new(play: &Play, song: &Song, artist: &Artist) -> Self {
        FullPlay {
            id: play.id,
            song: FullSong {
                id: song.id,
                artist: artist.clone(),
                title: song.title.clone(),
                cover_url: song.cover_url.clone(),
                cover_width: song.cover_width,
                cover_height: song.cover_height,
                asin: song.asin.clone(),
                last_cover_check: song.last_cover_check,
            },
            date: play.date,
            station_id: play.station_id,
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

#[derive(Serialize)]
pub struct StatsLast {
    pub plays: Vec<StatsLastPlay>,
}

#[derive(Serialize)]
pub struct StatsLastPlay {
    pub id: u64,
    pub song: FullSong,
    pub song_count: i32,
}

impl StatsLastPlay {
    pub fn new(play: &Play, song: &Song, artist: &Artist, song_count: i32) -> Self {
        StatsLastPlay {
            id: play.id,
            song: FullSong {
                id: song.id,
                artist: artist.clone(),
                title: song.title.clone(),
                cover_url: song.cover_url.clone(),
                cover_width: song.cover_width,
                cover_height: song.cover_height,
                asin: song.asin.clone(),
                last_cover_check: song.last_cover_check,
            },
            song_count: song_count,
        }
    }
}