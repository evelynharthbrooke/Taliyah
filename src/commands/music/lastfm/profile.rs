//! Last.fm profile command
//!
//! Retrieves a given user's Last.fm profile, including some user statistics
//! like top artists, and recently played tracks, as well as some general user
//! information.

use itertools::Itertools;

use lastfm_rs::{
    error::{
        Error,
        LastFMErrorResponse::{InvalidParameters, OperationFailed}
    },
    user::top_artists::Period
};

use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use tracing::error;

use crate::utils::{format_int, get_profile_field, net::*};

#[command]
#[description("Retrieves various Last.fm user stats.")]
#[aliases("p", "prof", "pf")]
#[usage("<user>")]
async fn profile(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let user = if !arguments.rest().is_empty() {
        if !message.mentions.is_empty() {
            get_profile_field(context, "user_lastfm_id", message.mentions.first().unwrap().id).await.unwrap()
        } else {
            arguments.single::<String>().unwrap()
        }
    } else {
        match get_profile_field(context, "user_lastfm_id", message.author.id).await {
            Ok(user) => user,
            Err(_) => match arguments.single::<String>() {
                Ok(argument) => argument,
                Err(_) => {
                    message.channel_id.say(context, "No username found. Please set one via `profile set` or provide one.").await?;
                    return Ok(());
                }
            }
        }
    };

    let mut client = get_lastfm_client(context).await;

    let recent_tracks = match client.recent_tracks(&user).await.with_limit(5).send().await {
        Ok(recent) => recent.tracks,
        Err(error) => match error {
            Error::LastFMError(OperationFailed(error)) => match error.message.as_str() {
                "Operation failed - Most likely the backend service failed. Please try again." => {
                    message.channel_id.say(context, "Last.fm's servers are currently offline. Please try again later.").await?;
                    return Ok(());
                }
                _ => {
                    error!("Last.fm operation failed: {:#?}", error);
                    message.channel_id.say(context, "An unknown Last.fm operation error occurred. Try again later.").await?;
                    return Ok(());
                }
            },
            Error::LastFMError(InvalidParameters(error)) => match error.message.as_str() {
                "User not found" => {
                    message.channel_id.say(context, "Invalid username provided. Please provide a valid one and try again.").await?;
                    return Ok(());
                }
                _ => {
                    error!("Unknown Last.fm parameter error: {:#?}", error);
                    message.channel_id.say(context, "An invalid Last.fm parameter was provided.").await?;
                    return Ok(());
                }
            },
            _ => {
                error!("Unrecognized Last.fm error encountered: {:#?}", error);
                message.channel_id.say(context, "An unrecognized Last.fm error was detected. Please try again later.").await?;
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
    let total_artists = format_int(top_artists.attrs.total.parse::<u64>().unwrap());

    let artists = top_artists
        .artists
        .iter()
        .map(|artist| {
            let name = &artist.name;
            let plays = format_int(artist.scrobbles.parse::<u64>().unwrap());
            format!("**{name}** — {plays} scrobbles")
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

    let registered = user_info.registered.date.format("%B %e, %Y");
    let scrobbles = format_int(user_info.scrobbles.parse::<u64>().unwrap());

    let track = recent_tracks.first().unwrap();

    let name = &track.name;
    let artist = &track.artist.name;
    let album = if track.album.name.is_empty() { String::new() } else { track.album.name.to_owned() };
    let artwork = get_album_artwork(context, artist, name, &album).await;

    let tracks = if recent_tracks.is_empty() {
        "Unknown".to_owned()
    } else {
        recent_tracks
            .iter()
            .map(|track| {
                let status = if track.attrs.is_none() { "" } else { "\x5c▶️" };
                let name = &track.name.replace("**", "\x5c**");
                let url = &track.url.replace("**", "\x5c**");
                let artist = &track.artist.name;
                format!("{status} **[{name}]({url})** — {artist}")
            })
            .join("\n")
    };

    let play_state = if track.attrs.as_ref().is_none() { "last listened to" } else { "is currently listening to" };
    let now_playing = format!("{username} {play_state} **{name}** by **{artist}** on **{album}**.");
    let fields = vec![
        ("**Display Name**", display_name, true),
        ("**Country**", country, true),
        ("**Join Date**", registered.to_string(), true),
        ("**Loved Tracks**", loved_tracks, true),
        ("**Total Artists**", total_artists, true),
        ("**Total Scrobbles**", scrobbles, true),
        ("**Top Artists:**", artists, false),
        ("**Recently Played:**", tracks, false),
    ];

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(username).url(url).icon_url(avatar))
        .thumbnail(artwork)
        .color(0x00d5_1007)
        .description(now_playing)
        .fields(fields)
        .footer(CreateEmbedFooter::new("Powered by Last.fm."));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

    Ok(())
}
