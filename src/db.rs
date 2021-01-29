use crate::models;
use crate::schema::*;
use actix_web::web;
use chrono::NaiveDate;
use diesel::prelude::*;
use diesel::MysqlConnection;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DieselResult<T> = Result<T, diesel::result::Error>;

pub fn get_full_plays(pool: web::Data<Pool>) -> DieselResult<Vec<models::FullPlay>> {
    let conn = pool.get().unwrap();
    let date_from = NaiveDate::from_ymd(2020, 12, 30).and_hms(0, 0, 0);
    let date_to = NaiveDate::from_ymd(2020, 12, 30).and_hms(23, 59, 59);
    let items = plays::table
        .filter(plays::date.between(date_from, date_to))
        .inner_join(songs::table.inner_join(artists::table))
        .select((plays::all_columns, songs::all_columns, artists::all_columns))
        .load::<(models::Play, models::Song, models::Artist)>(&*conn)?
        .iter()
        .map(|item| models::FullPlay::new(&item.0, &item.1, &item.2))
        .collect();
    Ok(items)
}
