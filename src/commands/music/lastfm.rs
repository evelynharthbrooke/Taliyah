//! Last.fm command
//!
//! Retrieves a chosen user's last.fm state, along with various
//! user information such as their most recent tracks.

use chrono::NaiveDateTime;

use crate::utilities;
use crate::utilities::database;

use itertools::Itertools;

use log::{error, warn};

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError;
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
        user = match database::get_user_lastfm_username(&message.author.id) {
            Ok(l) => l,
            Err(e) => {
                error!("Could not get Last.fm username in database: {}", e);
                match args.single::<String>() {
                    Ok(a) => a.to_string(),
                    Err(_) => {
                        return message
                            .channel_id
                            .send_message(&ctx, |m| {
                                m.embed(|e| {
                                    e.title("Error: No Last.fm username was found or provided.");
                                    e.description(
                                        "I could not find a Last.fm username pertaining to your user record, or \
                                        you did not provide a Last.fm username as an argument. Please set a username \
                                        via the profile command, or provide a Last.fm username as an argument.\
                                        ");
                                    e.color(0x00FF_0000);
                                    e
                                });
                                m
                            })
                            .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
                    }
                }
            }
        };
    }

    let api_key: String = env::var("LASTFM_KEY").expect("No API key detected");
    let mut client: Client = Client::new(&api_key);

    let recent_tracks = match client.recent_tracks(&user).with_limit(5).send() {
        Ok(c) => c.tracks,
        Err(e) => match e {
            Error::LastFMError(InvalidParameter(e)) => match e.message.as_str() {
                "User not found" => {
                    return message
                        .channel_id
                        .send_message(&ctx, |m| {
                            m.embed(|e| {
                                e.title("Error: Invalid Last.fm username provided.");
                                e.description("You have entered an invalid username. Please provide a valid one and then try again.");
                                e.color(0x00FF_0000);
                                e
                            });
                            m
                        })
                        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
                }
                _ => {
                    error!("Unknown Last.fm parameter error: {:#?}", e);
                    return message
                        .channel_id
                        .send_message(&ctx, |m| {
                            m.embed(|e| {
                                e.title("Error: Invalid Last.fm parameter provided.");
                                e.description("An invalid last.fm parameter was provided.");
                                e.color(0x00FF_0000);
                                e
                            });
                            m
                        })
                        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
                }
            },
            _ => {
                error!("Unknown Last.fm error encountered: {:#?}", e);
                return message
                    .channel_id
                    .send_message(&ctx, |m| {
                        m.embed(|e| {
                            e.title("Error: Unknown Last.fm Error Encountered.");
                            e.description("An unknown Last.fm error has occured. Please try again later.");
                            e.color(0x00FF_0000);
                            e
                        });
                        m
                    })
                    .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
            }
        },
    };

    let loved_tracks = client.loved_tracks(&user).send().unwrap().attrs.total;
    let user_info = client.user_info(&user).send().unwrap().user;

    let user_country = match user_info.country.clone().unwrap().is_empty() {
        true => "No country available.".to_owned(),
        false => user_info.country.unwrap(),
    };

    let user_display_name = match user_info.display_name.clone().unwrap().is_empty() {
        true => "No display name available.".to_string(),
        false => user_info.display_name.unwrap(),
    };

    let user_url = user_info.url;

    let user_username = match database::get_user_display_name(&message.author.id) {
        Ok(dn) => {
            let lastfm_name = match database::get_user_lastfm_username(&message.author.id) {
                Ok(l) => l,
                Err(_) => user_info.username.to_string(),
            };

            if lastfm_name == user {
                dn.to_string()
            } else {
                user_info.username.to_string()
            }
        }
        Err(_) => user_info.username.to_string(),
    };

    let user_registered = NaiveDateTime::from_timestamp(user_info.registered.friendly_date, 0).format("%B %e, %Y - %I:%M %p");
    let user_scrobbles = utilities::format_int(user_info.total_tracks.parse::<isize>().unwrap());

    let track = recent_tracks.first().unwrap();

    let tracks: String;

    match recent_tracks.is_empty() {
        true => tracks = "No recent tracks available".to_owned(),
        false => {
            tracks = recent_tracks
                .iter()
                .map(|t: &Track| {
                    let mut now_playing: String = "".to_owned();

                    match t.attrs.as_ref().is_none() {
                        true => warn!("No track attributes associated with this track."),
                        false => now_playing = "\x5c▶️".to_owned(),
                    }

                    format!("{} **{}** — {}", now_playing, t.name, t.artist.name)
                })
                .join("\n");
        }
    };

    let track_play_state: String;
    match track.attrs.as_ref().is_none() {
        true => track_play_state = "last listened to".to_owned(),
        false => track_play_state = "is currently listening to".to_owned(),
    }

    let currently_playing: String = format!(
        "{} {} {} by {} on {}.",
        user_username, track_play_state, track.name, track.artist.name, track.album.name
    );

    return message
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.title(format!("{}'s Last.fm", user_username));
                e.url(user_url);
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
                    currently_playing, user_display_name, user_country, user_registered, loved_tracks, user_scrobbles, tracks
                ));
                e.footer(|f| f.text("Powered by the Last.fm API."));
                e
            });
            m
        })
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
}
