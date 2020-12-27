//! Network Utilities
//!
//! These utilities help with various network-related tasks
//! and functions.

use aspotify::{CountryCode::CAN, ItemType, Market::Country};
use lastfm_rs::Client;

use serenity::client::Context;

use crate::data::{ReqwestContainer, SpotifyContainer};

use super::read_config;

pub async fn get_lastfm_client(context: &Context) -> Client {
    let config = read_config("config.toml");
    let api_key = config.api.music.lastfm.api_key;
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    Client::from_reqwest_client(client, &api_key)
}

pub async fn get_album_artwork(context: &Context, artist: &str, track: &str, album: &str) -> String {
    let data = context.data.read().await;
    let spotify = data.get::<SpotifyContainer>().unwrap();

    let search_string = format!("artist:\"{}\" track:\"{}\" album:\"{}\"", artist, track, album);
    let track_search = spotify.search().search(&search_string, [ItemType::Track].iter().copied(), false, 1, 0, Some(Country(CAN))).await;
    let track_result = &track_search.unwrap().data.tracks.unwrap();

    let items = &track_result.items;
    let track = items.first().unwrap();
    let image = track.album.images.first().unwrap();
    let album_art = image.url.as_str();
    album_art.to_string()
}
