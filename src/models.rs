use crate::schema::{artists, plays, songs, stations};
use chrono::naive::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Clone)]
#[table_name = "artists"]
pub struct Artist {
    pub id: u64,
    pub name: String,
}

#[belongs_to(Artist)]
#[derive(Debug, Associations, Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "songs"]
pub struct Song {
    pub id: u64,
    pub artist_id: u64,
    pub title: String,
    pub cover_url: Option<String>,
    pub cover_width: Option<u16>,
    pub cover_height: Option<u16>,
    pub asin: Option<String>,
    pub last_cover_check: Option<NaiveDate>,
}

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize)]
#[table_name = "stations"]
pub struct Station {
    pub id: u64,
    pub key: String,
    pub title: String,
}

#[belongs_to(Song)]
#[belongs_to(Station)]
#[derive(Debug, Associations, Queryable, Serialize, Deserialize)]
#[table_name = "plays"]
pub struct Play {
    pub id: u64,
    pub song_id: u64,
    pub date: NaiveDateTime,
    pub station_id: u64,
}

