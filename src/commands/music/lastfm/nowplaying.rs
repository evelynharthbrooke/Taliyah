use lastfm_rs::error::{
    Error,
    LastFMErrorResponse::{InvalidParameters, OperationFailed}
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use tracing::error;

use crate::utils::{get_profile_field, net_utils::*};

#[command]
#[description("Retrieves the Last.fm now playing state of a given user.")]
#[aliases("np")]
#[usage("<user>, or leave blank")]
pub async fn nowplaying(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    message.channel_id.broadcast_typing(context).await?;

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

    let recent_tracks = match client.recent_tracks(&user).await.with_limit(1).send().await {
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

    let user_info = client.user_info(&user).await.send().await.unwrap().user;
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

    let avatar = user_info.images[3].image_url.as_str();
    let url = user_info.url;

    let track = recent_tracks.first().unwrap();
    let track_url = &track.url;

    let name = &track.name;
    let artist = &track.artist.name;
    let album = if track.album.name.is_empty() { "".to_owned() } else { track.album.name.to_owned() };
    let artwork = get_album_artwork(context, artist, name, &album).await;
    let header = format!("{} is currently playing:", username);

    message
        .channel_id
        .send_message(context, |msg| {
            msg.embed(|embed| {
                embed.author(|author| author.name(header).url(url).icon_url(avatar));
                embed.title(name);
                embed.url(track_url);
                embed.description(format!("**{}** | {}", artist, album));
                embed.thumbnail(artwork);
                embed.color(0x00d5_1007);
                embed
            })
        })
        .await?;

    Ok(())
}
