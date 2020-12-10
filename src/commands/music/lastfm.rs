//! Last.fm command
//!
//! Retrieves a chosen user's last.fm state, along with various
//! user information such as their most recent tracks.

use crate::{data::ReqwestContainer, utils::{format_int, get_album_artwork, get_profile_field, read_config}};

use chrono::NaiveDateTime;
use itertools::Itertools;

use lastfm_rs::{
    error::{
        Error,
        LastFMErrorResponse::{InvalidParameter, OperationFailed}
    },
    user::{
        recent_tracks::Track,
        top_artists::{Artist, Period}
    },
    Client
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::env;
use tracing::error;

#[command]
#[description("Retrieves various Last.fm user stats.")]
#[aliases("fm", "lfm", "lastfm")]
#[usage("<user> <limit>")]
pub async fn lastfm(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let user = if !arguments.rest().is_empty() {
        arguments.single::<String>().unwrap()
    } else {
        match get_profile_field(context, "user_lastfm_id", message.author.id).await {
            Ok(user) => user,
            Err(e) => {
                error!("Could not get Last.fm username in database: {}", e);
                match arguments.single::<String>() {
                    Ok(argument) => argument,
                    Err(_) => {
                        message
                            .channel_id
                            .send_message(&context, |message| {
                                message.embed(|embed| {
                                    embed.title("Error: No Last.fm username was found or provided.");
                                    embed.color(0x00FF_0000);
                                    embed.description("No username found. Please provide one or set one via `profile set`.")
                                })
                            })
                            .await?;
                        return Ok(());
                    }
                }
            }
        }
    };

    let config = read_config(&env::var("ELLIE_CONFIG_FILE").unwrap());
    let api_key = config.api.music.lastfm.api_key;
    let reqwest_client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();

    let mut client = Client::from_reqwest_client(reqwest_client, &api_key);

    let recent_tracks = match client.recent_tracks(&user).await.with_limit(5).send().await {
        Ok(rt) => rt.tracks,
        Err(error) => match error {
            Error::LastFMError(OperationFailed(error)) => match error.message.as_str() {
                "Operation failed - Most likely the backend service failed. Please try again." => {
                    message
                        .channel_id
                        .send_message(&context, |message| {
                            message.embed(|embed| {
                                embed.title("Error: Last.fm is currently offline.");
                                embed.description("Last.fm's servers are currently offline. Please try again later.");
                                embed.color(0x00FF_0000)
                            })
                        })
                        .await?;
                    return Ok(());
                }
                _ => {
                    error!("Last.fm operation failed: {:#?}", error);
                    message
                        .channel_id
                        .send_message(&context, |message| {
                            message.embed(|embed| {
                                embed.title("Error: Unknown Last.fm operation error.");
                                embed.description("An unknown Last.fm operation error was encountered. Please try again later.");
                                embed.color(0x00FF_0000)
                            })
                        })
                        .await?;
                    return Ok(());
                }
            },
            Error::LastFMError(InvalidParameter(error)) => match error.message.as_str() {
                "User not found" => {
                    message
                        .channel_id
                        .send_message(&context, |message| {
                            message.embed(|embed| {
                                embed.title("Error: Invalid Last.fm username provided.");
                                embed.description("Invalid username provided. Please provide a valid one and then try again.");
                                embed.color(0x00FF_0000)
                            })
                        })
                        .await?;
                    return Ok(());
                }
                _ => {
                    error!("Unknown Last.fm parameter error: {:#?}", error);
                    message
                        .channel_id
                        .send_message(&context, |message| {
                            message.embed(|embed| {
                                embed.title("Error: Invalid Last.fm parameter provided.");
                                embed.description("An invalid Last.fm parameter was provided.");
                                embed.color(0x00FF_0000)
                            })
                        })
                        .await?;

                    return Ok(());
                }
            },
            _ => {
                error!("Unrecognized Last.fm error encountered: {:#?}", error);
                message
                    .channel_id
                    .send_message(&context, |message| {
                        message.embed(|embed| {
                            embed.title("Error: Unrecognized Last.fm error encountered.");
                            embed.description("An unrecognized Last.fm error was detected. Please try again later.");
                            embed.color(0x00FF_0000)
                        })
                    })
                    .await?;

                return Ok(());
            }
        }
    };

    let loved_tracks = client.loved_tracks(&user).await.send().await.unwrap().attrs.total;
    let top_artists = client.top_artists(&user).await.within_period(Period::Overall).with_limit(5).send().await.unwrap();
    let user_info = client.user_info(&user).await.send().await.unwrap().user;

    let display_name = if user_info.display_name.is_empty() { "None".to_string() } else { user_info.display_name };

    let avatar = user_info.images[3].image_url.as_str();
    let country = user_info.country;
    let url = user_info.url;
    let total_artists = format_int(top_artists.attrs.total.parse::<usize>().unwrap());

    let artists = top_artists
        .artists
        .iter()
        .map(|a: &Artist| {
            let name = &a.name;
            let plays = format_int(a.playcount.parse::<usize>().unwrap());
            format!("**{}** — {} plays", name, plays)
        })
        .join("\n");

    let username = match get_profile_field(context, "user_name", message.author.id).await {
        Ok(database_name) => {
            let lastfm_name = match get_profile_field(context, "user_lastfm_id", message.author.id).await {
                Ok(name) => name,
                Err(_) => user_info.username.to_string()
            };

            if lastfm_name == user {
                database_name
            } else {
                user_info.username.to_string()
            }
        }
        Err(_) => user_info.username.as_str().to_string()
    };

    let registered = NaiveDateTime::from_timestamp(user_info.registered.friendly_date, 0).format("%B %e, %Y");
    let scrobbles = format_int(user_info.total_tracks.parse::<usize>().unwrap());

    let track = recent_tracks.first().unwrap();

    let name = &track.name;
    let artist = &track.artist.name;
    let album = if track.album.name.is_empty() { "".to_owned() } else { track.album.name.to_owned() };
    let artwork = get_album_artwork(artist, name, &album).await;

    let tracks = if recent_tracks.is_empty() {
        "No recent tracks available".to_owned()
    } else {
        recent_tracks
            .iter()
            .map(|track: &Track| {
                let track_status = if track.attrs.is_none() { "" } else { "\x5c▶️" };
                let track_name = &track.name.replace("**", "\x5c**");
                let track_url = &track.url;
                let track_artist = &track.artist.name;
                format!("{} **[{}]({})** — {}", track_status, track_name, track_url, track_artist)
            })
            .join("\n")
    };

    let play_state = if track.attrs.as_ref().is_none() { "last listened to" } else { "is currently listening to" };
    let now_playing = format!("{} {} **{}** by **{}** on **{}**.", username, play_state, name, artist, album.to_string());

    let lastfm_fields = vec![
        ("**Name**", display_name, true),
        ("**Country**", country, true),
        ("**Join Date**", registered.to_string(), true),
        ("**Loved Tracks**", loved_tracks, true),
        ("**Total Artists**", total_artists, true),
        ("**Total Scrobbles**", scrobbles, true),
        ("**Top Artists:**", artists, false),
        ("**Recently Played:**", tracks, false),
    ];

    message
        .channel_id
        .send_message(context, |message| {
            message.embed(|embed| {
                embed.author(|author| author.name(username).url(url).icon_url(avatar));
                embed.thumbnail(artwork);
                embed.color(0x00d5_1007);
                embed.description(now_playing);
                embed.fields(lastfm_fields);
                embed.footer(|f| f.text("Powered by Last.fm."));
                embed
            })
        })
        .await?;

    Ok(())
}
