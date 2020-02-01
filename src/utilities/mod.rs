pub mod built_info;
pub mod color_utils;
pub mod database;
pub mod geo_utils;
pub mod git_utils;

use crate::spotify;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC as AlphaNumSet};

use serenity::model::prelude::{GuildId, UserId};
use serenity::prelude::Context;
use serenity::utils::parse_username;

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

pub fn parse_user(name: &str, guild_id: Option<&GuildId>, context: Option<&Context>) -> Option<UserId> {
    if let Some(x) = parse_username(&name) {
        return Some(UserId(x));
    } else if guild_id.is_none() || context.is_none() {
        return None;
    }

    let guild_id = guild_id.unwrap();
    let context = context.unwrap();

    let cached_guild = match guild_id.to_guild_cached(&context) {
        Some(guild) => guild,
        None => return None,
    };

    let guild = cached_guild.read();

    if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(context, id) {
            return Some(m.user.read().id);
        }
    }

    if let Some(m) = guild.member_named(name) {
        return Some(m.user.read().id);
    } else if let Some(m) = guild.members_starting_with(name, false, true).get(0) {
        return Some(m.user.read().id);
    } else if let Some(m) = guild.members_containing(name, false, true).get(0) {
        return Some(m.user.read().id);
    }

    None
}
