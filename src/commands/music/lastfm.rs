//! Last.fm command
//!
//! Retrieves a chosen user's last.fm state, along with various
//! user information such as their most recent tracks.

use chrono::NaiveDateTime;

use crate::spotify;
use crate::utilities;
use crate::utilities::database;

use itertools::Itertools;

use log::{error, warn};

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use rustfm::error::Error;
use rustfm::error::LastFMErrorResponse::InvalidParameter;
use rustfm::user::recent_tracks::Track;
use rustfm::Client;

use std::env;

#[command]
#[description("Retrieves various Last.fm user stats.")]
#[aliases("fm", "lfm", "lastfm")]
#[usage("<user> <limit>")]
pub fn lastfm(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let user: String;

    if !args.rest().is_empty() {
        user = args.single::<String>().unwrap();
    } else {
        user = match database::get_user_lastfm(&message.author.id) {
            Ok(user) => user,
            Err(e) => {
                error!("Could not get Last.fm username in database: {}", e);
                match args.single::<String>() {
                    Ok(argument) => argument.to_string(),
                    Err(_) => {
                        message.channel_id.send_message(&ctx, |message| {
                            message.embed(|embed| {
                                embed.title("Error: No Last.fm username was found or provided.");
                                embed.description(
                                    "I could not find a Last.fm username pertaining to your user record, or \
                                    you did not provide a Last.fm username as an argument. Please set a username \
                                    via the profile command, or provide a Last.fm username as an argument.",
                                );
                                embed.color(0x00FF_0000)
                            })
                        })?;
                        return Ok(());
                    }
                }
            }
        };
    }

    let api_key: String = env::var("LASTFM_KEY").expect("No API key detected");
    let mut client: Client = Client::new(&api_key);

    let recent_tracks = match client.recent_tracks(&user).with_limit(5).send() {
        Ok(rt) => rt.tracks,
        Err(error) => match error {
            Error::LastFMError(InvalidParameter(error)) => match error.message.as_str() {
                "User not found" => {
                    message.channel_id.send_message(&ctx, |message| {
                        message.embed(|embed| {
                            embed.title("Error: Invalid Last.fm username provided.");
                            embed.description("Invalid username provided. Please provide a valid one and then try again.");
                            embed.color(0x00FF_0000)
                        })
                    })?;

                    return Ok(());
                }
                _ => {
                    error!("Unknown Last.fm parameter error: {:#?}", error);
                    message.channel_id.send_message(&ctx, |message| {
                        message.embed(|embed| {
                            embed.title("Error: Invalid Last.fm parameter provided.");
                            embed.description("An invalid last.fm parameter was provided.");
                            embed.color(0x00FF_0000)
                        })
                    })?;

                    return Ok(());
                }
            },
            _ => {
                error!("Unknown Last.fm error encountered: {:#?}", error);
                message.channel_id.send_message(&ctx, |message| {
                    message.embed(|embed| {
                        embed.title("Error: Unknown Last.fm Error Encountered.");
                        embed.description("An unknown Last.fm error has occured. Please try again later.");
                        embed.color(0x00FF_0000)
                    })
                })?;

                return Ok(());
            }
        },
    };

    let loved_tracks = if client.loved_tracks(&user).send().unwrap().attrs.total == "0" {
        "No loved tracks...:(".to_string()
    } else {
        client.loved_tracks(&user).send().unwrap().attrs.total
    };

    let user_info = client.user_info(&user).send().unwrap().user;

    let display_name = match user_info.display_name.clone().unwrap().is_empty() {
        true => "No display name available.".to_string(),
        false => user_info.display_name.unwrap(),
    };

    let country = user_info.country;
    let url = user_info.url;

    let username = match database::get_user_display_name(&message.author.id) {
        Ok(database_name) => {
            let lastfm_name = match database::get_user_lastfm(&message.author.id) {
                Ok(name) => name,
                Err(_) => user_info.username.to_string(),
            };

            if lastfm_name == user {
                database_name.to_string()
            } else {
                user_info.username.to_string()
            }
        }
        Err(_) => user_info.username.to_string(),
    };

    let registered = NaiveDateTime::from_timestamp(user_info.registered.friendly_date, 0).format("%A, %B %e, %Y @ %l:%M %P");
    let scrobbles = utilities::format_int(user_info.total_tracks.parse::<usize>().unwrap());

    let track = recent_tracks.first().unwrap();

    let name = &track.name;
    let artist = &track.artist.name;

    let album = match track.album.name.as_str().is_empty() {
        true => "",
        false => track.album.name.as_str(),
    };

    let sp_search_string = format!("track:{} artist:{} album:{}", name, artist, album.replace("&", "%26"));
    let sp_track_search = spotify().search_track(sp_search_string.as_str(), 1, 0, None);
    let sp_track_result = &sp_track_search.unwrap();

    let track_art: &str;
    match sp_track_result.tracks.items.first() {
        Some(track) => {
            let image = track.album.images.first().unwrap();
            let album_art = image.url.as_str();
            track_art = album_art
        }
        None => track_art = track.images.get(3).unwrap().image_url.as_str(),
    };

    let tracks = match recent_tracks.is_empty() {
        true => "No recent tracks available".to_owned(),
        false => recent_tracks
            .iter()
            .map(|track: &Track| {
                let mut now_playing: String = "".to_owned();
                let track_name = &track.name.replace("**", "\x5c**");
                let track_artist = &track.artist.name;

                match track.attrs.as_ref().is_none() {
                    true => warn!("No track attributes associated with this track."),
                    false => now_playing = "\x5c▶️".to_owned(),
                }

                format!("{} **{}** — {}", now_playing, track_name, track_artist)
            })
            .join("\n"),
    };

    let play_state = match track.attrs.as_ref().is_none() {
        true => "last listened to".to_owned(),
        false => "is currently listening to".to_owned(),
    };

    let currently_playing: String = format!("{} {} {} by {} on {}.", username, play_state, name, artist, album);

    message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title(format!("{}'s Last.fm", username));
            e.url(url);
            e.thumbnail(track_art);
            e.color(0x00d5_1007);
            e.description(format!(
                "{}\n\n\
                **__User information:__**\n\
                **Display name**: {}\n\
                **Country**: {}\n\
                **Join date**: {}\n\
                **Loved tracks**: {}\n\
                **Total track plays**: {}\n\n\
                **__Recent tracks:__**\n\
                {}",
                currently_playing, display_name, country, registered, loved_tracks, scrobbles, tracks
            ));
            e.footer(|f| f.text("Powered by the Last.fm API."))
        })
    })?;

    return Ok(());
}
