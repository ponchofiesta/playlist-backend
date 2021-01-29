table! {
    artists (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

table! {
    plays (id) {
        id -> Unsigned<Bigint>,
        song_id -> Unsigned<Bigint>,
        date -> Timestamp,
        station_id -> Unsigned<Bigint>,
    }
}

table! {
    songs (id) {
        id -> Unsigned<Bigint>,
        artist_id -> Unsigned<Bigint>,
        title -> Varchar,
        cover_url -> Nullable<Varchar>,
        cover_width -> Nullable<Unsigned<Smallint>>,
        cover_height -> Nullable<Unsigned<Smallint>>,
        asin -> Nullable<Varchar>,
        last_cover_check -> Nullable<Date>,
    }
}

table! {
    stations (id) {
        id -> Unsigned<Bigint>,
        key -> Varchar,
        title -> Varchar,
    }
}

joinable!(plays -> songs (song_id));
joinable!(plays -> stations (station_id));
joinable!(songs -> artists (artist_id));

allow_tables_to_appear_in_same_query!(
    artists,
    plays,
    songs,
    stations,
);
