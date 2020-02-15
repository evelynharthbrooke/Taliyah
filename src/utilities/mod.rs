pub mod built_info;
pub mod color_utils;
pub mod database;
pub mod geo_utils;
pub mod git_utils;
pub mod parsing_utils;

use crate::spotify;

use itertools::Itertools;

use kuchiki::traits::*;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC as AlphaNumSet};

use reqwest::blocking::Client;
use reqwest::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "accessToken")]
    pub access_token: String,
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

/// Retrieves album artwork for a specified song via the Spotify
/// Web API. If it is unable to find any artwork, it will return an
/// empty string.
pub fn get_album_artwork(artist: &str, track: &str, album: &str) -> String {
    let sp_search_string = format!("artist:{} track:{} album:{}", artist, track, album);
    let sp_search_string_encoded = utf8_percent_encode(&sp_search_string, AlphaNumSet).to_string();
    let sp_track_search = spotify().search_track(&sp_search_string_encoded, 1, 0, None);
    let sp_track_result = &sp_track_search.unwrap();
    let sp_results = &sp_track_result.tracks.items;
    match sp_results.first() {
        Some(track) => {
            let image = track.album.images.first().unwrap();
            let album_art = image.url.as_str();
            album_art.to_string()
        }
        None => "".to_string(),
    }
}

/// Takes a string as input and formats it to convert various
/// ASCII codes / HTML shorthands to their proper text forms.
///
/// This will be gradually updated to add new replacers when
/// necessary.
pub fn format_string(string: String) -> String {
    string.replace("&amp;", "&").replace("&quot;", "\"")
}

/// Calculates the average sum of an array of i64's.
pub fn calculate_average_sum(ints: &[i64]) -> f64 {
    ints.iter().sum::<i64>() as f64 / ints.len() as f64
}

/// Gets an anonymous access token from the Spotify Web Player.
///
/// This uses a user agent string that will spoof the Spotify Web Player
/// into giving us an access token without giving off any weird signals
/// at Spotify's API backend. All Spotify knows is that the API is receiving
/// an access token from the Web Player on each request.
///
/// This access token, when retrieved, is used to access certain API
/// endpoints from Spotify's backend that isn't publicly available to
/// the normal Spotify Web API, such as the Credits API used by Spotify's
/// mobile and desktop clients.
pub fn get_spotify_token() -> Result<String, Error> {
    let user_agent_chunk_1 = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)";
    let user_agent_chunk_2 = "Chrome/81.0.4041.0 Safari/537.36 Edg/81.0.410.0";
    let user_agent = &[user_agent_chunk_1, user_agent_chunk_2].join(" ");

    let spotify_open_url = "https://open.spotify.com";
    let client = Client::builder().user_agent(user_agent).build()?;
    let request = client.get(spotify_open_url).send()?.text()?;
    let html = kuchiki::parse_html().one(request);

    let config = html
        .select("#config")
        .unwrap()
        .map(|c| {
            let as_node = c.as_node();
            let text_node = as_node.first_child().unwrap();
            let text = text_node.as_text().unwrap().borrow();
            text.clone()
        })
        .join("");

    let config_json: Config = serde_json::from_str(config.replace("\n", "").trim()).unwrap();
    Ok(config_json.access_token)
}
