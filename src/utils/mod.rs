// pub mod color_utils;
pub mod git_utils;
pub mod locale_utils;
pub mod parsers;

use aspotify::ItemType;
use serenity::{client::Context, model::id::UserId};
use sqlx::Row;
use std::{fs::File, io::prelude::Read};
use tracing::error;

use crate::{config::ConfigurationData, data::{DatabasePool, SpotifyContainer}, error::EllieError};

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
        Ok(row) => match row.try_get(0).map_err(EllieError::DatabaseError) {
            Ok(row) => Ok(row),
            Err(err) => {
                error!("Field not set in database: {}", err);
                Ok("Field not set.".to_string())
            }
        },
        Err(err) => {
            error!("Error querying database: {}", err);
            Ok("Database unsuccessfully queried.".to_string())
        }
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
pub async fn get_album_artwork(context: &Context, artist: &str, track: &str, album: &str) -> String {
    let data = context.data.read().await;
    let spotify = data.get::<SpotifyContainer>().unwrap();

    let search_string = format!("artist:{} track:{} album:{}", artist, track, album);
    let track_search = spotify.search().search(&search_string, [ItemType::Track].iter().copied(), false, 1, 0, None).await;
    let track_result = &track_search.unwrap().data.tracks.unwrap();

    let items = &track_result.items;
    let track = items.first().unwrap();
    let image = track.album.images.first().unwrap();
    let album_art = image.url.as_str();
    album_art.to_string()
}
