use crate::models;
use crate::schema::{artists, plays, songs, stations};
use chrono::{NaiveDate, NaiveDateTime};
// use diesel::dsl::{max, min};
use diesel::prelude::*;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DieselResult<T> = Result<T, diesel::result::Error>;

pub fn get_full_plays(
    connection: &MysqlConnection,
    station: &str,
    date: &NaiveDate,
) -> DieselResult<Vec<models::FullPlay>> {
    let date_from = date.and_hms(0, 0, 0);
    let date_to = date.and_hms(23, 59, 59);
    let items = plays::table
        .filter(plays::date.between(date_from, date_to))
        .inner_join(songs::table.inner_join(artists::table))
        .inner_join(stations::table)
        .filter(stations::key.eq(station))
        .select((plays::all_columns, songs::all_columns, artists::all_columns))
        .load::<(models::Play, models::Song, models::Artist)>(connection)?
        .iter()
        .map(|item| models::FullPlay::new(&item.0, &item.1, &item.2))
        .collect();
    Ok(items)
}

// pub fn get_previous_day(
//     connection: &MysqlConnection,
//     station: &str,
//     from_date: &NaiveDate,
// ) -> DieselResult<Option<NaiveDate>> {
//     let date_time = from_date.and_hms(0, 0, 0);
//     let previous_date = plays::table
//         .filter(plays::date.lt(date_time))
//         .inner_join(stations::table)
//         .filter(stations::key.eq(station))
//         .select(max(plays::date))
//         .first::<Option<NaiveDateTime>>(connection)?;
//     Ok(match previous_date {
//         Some(date) => Some(date.date()),
//         None => None,
//     })
// }

// pub fn get_next_day(
//     connection: &MysqlConnection,
//     station: &str,
//     from_date: &NaiveDate,
// ) -> DieselResult<Option<NaiveDate>> {
//     let date_time = from_date.and_hms(23, 59, 59);
//     let query = plays::table
//         .filter(plays::date.gt(date_time))
//         .inner_join(stations::table)
//         .filter(stations::key.eq(station))
//         .select(min(plays::date))
//         .first::<Option<NaiveDateTime>>(connection);
//     let sql = debug_query::<diesel::mysql::Mysql, _>(&query);
//     println!("{:?}", sql);
//     let previous_date = query?;
//     Ok(match previous_date {
//         Some(date) => Some(date.date()),
//         None => None,
//     })
// }

use diesel::sql_types::SqlType;
use diesel::expression::TypedExpressionType;
diesel::infix_operator!(SoundsLike, " SOUNDS LIKE ");
use diesel::expression::AsExpression;

fn sounds_like<T, U, ST>(left: T, right: U) -> SoundsLike<T, U::Expression>
where
    T: Expression<SqlType = ST>,
    U: AsExpression<ST>,
    ST: SqlType + TypedExpressionType,
{
    SoundsLike::new(left, right.as_expression())
}


pub fn search(
    connection: &MysqlConnection,
    station: &str,
    term: &str,
) -> DieselResult<Vec<models::FullPlay>> {
    let items = plays::table
        .inner_join(songs::table.inner_join(artists::table))
        .filter(stations::key.eq(station))
        .filter(sounds_like(songs::title, term).or(sounds_like(artists::name, term)))
        .select((plays::all_columns, songs::all_columns, artists::all_columns))
        .load::<(models::Play, models::Song, models::Artist)>(connection)?
        .iter()
        .map(|item| models::FullPlay::new(&item.0, &item.1, &item.2))
        .collect();
}
