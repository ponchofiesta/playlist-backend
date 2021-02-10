use crate::handlers::SearchParams;
use crate::models;
use crate::schema::{artists, plays, songs, stations};
use chrono::NaiveDate;
use diesel::dsl::sql;
use diesel::prelude::*;
use diesel::sql_types::Text;
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

pub fn search(
    connection: &MysqlConnection,
    station: &str,
    params: &SearchParams,
) -> DieselResult<Vec<models::FullPlay>> {
    let mut query = plays::table
        .inner_join(songs::table.inner_join(artists::table))
        .inner_join(stations::table)
        .filter(stations::key.eq(station))
        .select((plays::all_columns, songs::all_columns, artists::all_columns))
        .into_boxed();

    // let grouping = match params.grouping {
    //     Some(grouping) => grouping,
    //     None => false,
    // };
    if params.advanced.is_some() && params.advanced.unwrap() {
        if params.artist.is_some() {
            let artist = params.artist.as_deref().unwrap();
            query = query.filter(sql("`artists`.`name` SOUNDS LIKE ").bind::<Text, _>(artist));
        }
        if params.title.is_some() {
            let title = params.title.as_deref().unwrap();
            query = query.filter(sql("`songs`.`title` SOUNDS LIKE ").bind::<Text, _>(title));
        }
        if params.date_from.is_some() && params.date_to.is_some() {
            let date_from = params.date_from.unwrap().and_hms(0, 0, 0);
            let date_to = params.date_to.unwrap().and_hms(23, 59, 59);
            query = query.filter(plays::date.between(date_from, date_to));
        }
    } else {
        if params.term.is_some() {
            let term = params.term.as_deref().unwrap();
            query = query.filter(
                sql("`artists`.`name` SOUNDS LIKE ")
                    .bind::<Text, _>(term)
                    .sql(" OR `songs`.`title` SOUNDS LIKE ")
                    .bind::<Text, _>(term),
            );
        }
    }

    // let items = plays::table
    //     .inner_join(songs::table.inner_join(artists::table))
    //     .inner_join(stations::table)
    //     .filter(stations::key.eq(station))
    //     .filter(
    //         sql("`artists`.`name` SOUNDS LIKE ")
    //             .bind::<Text, _>(&params.term)
    //             .sql(" OR `songs`.`title` SOUNDS LIKE ")
    //             .bind::<Text, _>(&params.term),
    //     )
    //     .select((plays::all_columns, songs::all_columns, artists::all_columns))
    //     .load::<(models::Play, models::Song, models::Artist)>(connection)?
    //     .iter()
    //     .map(|item| models::FullPlay::new(&item.0, &item.1, &item.2))
    //     .collect();

    let items = query
        .order(plays::date.desc())
        .limit(100)
        .load::<(models::Play, models::Song, models::Artist)>(connection)?
        .iter()
        .map(|item| models::FullPlay::new(&item.0, &item.1, &item.2))
        .collect();
    Ok(items)
}
