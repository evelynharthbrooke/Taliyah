// pub mod color_utils;
pub mod git_utils;
pub mod macros;
pub mod locale_utils;
pub mod parsers;

use crate::{config::ConfigurationData, data::DatabasePool, error::EllieError};

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use rspotify::{
    client::Spotify,
    model::{page::Page, search::SearchResult, track::FullTrack},
    oauth2::SpotifyClientCredentials,
    senum::SearchType
};

use serenity::{client::Context, model::id::UserId};
use sqlx::Row;
use std::{env, fs::File, io::prelude::Read};

pub fn read_config(file: &str) -> ConfigurationData {
    let mut file = File::open(file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str::<ConfigurationData>(&contents).unwrap()
}

pub async fn get_profile_field(context: &Context, field: &str, user_id: UserId) -> Result<String, EllieError> {
    let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();
    match sqlx::query(format!("SELECT {} FROM profile_data WHERE user_id = $1", field).as_str())
        .bind(user_id.0 as i64)
        .fetch_one(&pool)
        .await
    {
        Ok(row) => row.try_get(0).map_err(|e| EllieError::DatabaseError(e)),
        Err(err) => Err(EllieError::DatabaseError(err))
    }
}

pub fn format_int(integer: usize) -> String {
    let mut string = String::new();
    let integer_str = integer.to_string();
    let a = integer_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            string.insert(0, ',');
        }
        string.insert(0, val);
    }
    string
}

/// Calculates the average sum of an array of i64's.
pub fn calculate_average_sum(ints: &[i64]) -> f64 {
    ints.iter().sum::<i64>() as f64 / ints.len() as f64
}

/// Retrieves album artwork for a specified song via the Spotify
/// Web API. If it is unable to find any artwork, it will return an
/// empty string.
pub async fn get_album_artwork(artist: &str, track: &str, album: &str) -> String {
    let sp_search_string = format!("artist:{} track:{} album:{}", artist, track, album);
    let sp_search_string_encoded = utf8_percent_encode(&sp_search_string, NON_ALPHANUMERIC).to_string();
    let sp_track_search = spotify().await.search(&sp_search_string_encoded, SearchType::Track, 1, 0, None, None).await;
    let sp_track_result = &sp_track_search.unwrap();

    match sp_track_result {
        SearchResult::Tracks(track) => {
            let track: &Page<FullTrack> = track;
            let items = &track.items;

            let track = items.first().unwrap();
            let image = track.album.images.first().unwrap();
            let album_art = image.url.as_str();
            album_art.to_string()
        }
        _ => String::new()
    }
}

pub async fn spotify() -> Spotify {
    let config = read_config(&env::var("ELLIE_CONFIG_FILE").unwrap());
    let client_id = config.api.music.spotify.client_id;
    let client_secret = config.api.music.spotify.client_secret;
    let credentials = SpotifyClientCredentials::default().client_id(&client_id).client_secret(&client_secret).build();
    Spotify::default().client_credentials_manager(credentials).build()
}
