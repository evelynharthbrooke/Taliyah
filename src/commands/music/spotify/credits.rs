use crate::spotify;
use crate::utilities::get_spotify_token;

use itertools::Itertools;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use reqwest::blocking::Client;

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[derive(Deserialize, Debug)]
pub struct Credits {
    label: String,
    #[serde(rename = "trackUri")]
    track_uri: String,
    #[serde(rename = "trackTitle")]
    track_title: String,
    #[serde(rename = "roleCredits")]
    role_credits: Vec<Role>,
    source: Source
}

#[derive(Deserialize, Debug)]
pub struct Role {
    #[serde(rename = "roleTitle")]
    role_title: String,
    artists: Vec<Artist>
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    uri: String,
    name: String,
    subroles: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct Source {
    label: String,
    value: String
}

#[command]
#[description("Displays credits for a specific track on Spotify.")]
fn credits(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: No track name provided.");
                embed.description(
                    "You did not provide a track name. Please enter one and \
                        then try again."
                )
            })
        })?;
        return Ok(());
    }

    let track_name = utf8_percent_encode(args.rest(), NON_ALPHANUMERIC).to_string();
    let track_search = spotify().search_track(&track_name, 1, 0, None);
    let track_result = &track_search.unwrap().tracks.items;

    if track_result.is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: No track found.");
                embed.color(0x00FF_0000);
                embed.description(format!(
                    "I was unable to to find a track on Spotify matching the term `{}`. \
                    Please try looking for a different track, or try again later.",
                    track_name
                ))
            })
        })?;

        return Ok(());
    }

    let track = track_result.first().unwrap();
    let track_name = &track.name;
    let track_url = &track.external_urls["spotify"];
    let track_artist = &track.artists.first().unwrap().name;
    let track_image = &track.album.images.first().unwrap().url;
    let track_id = &track.id.as_ref().unwrap();

    let user_agent_chunk_1 = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)";
    let user_agent_chunk_2 = "Chrome/82.0.4051.0 Safari/537.36 Edg/82.0.425.0";
    let user_agent = &[user_agent_chunk_1, user_agent_chunk_2].join(" ");
    let client = Client::builder().user_agent(user_agent).build()?;

    let access_token = get_spotify_token().unwrap();
    let spclient_url = format!("https://spclient.wg.spotify.com/track-credits-view/v0/track/{}/credits", track_id);
    let credits_request: Credits = client.get(&spclient_url).bearer_auth(&access_token).send()?.json()?;
    let credits_response = credits_request.role_credits;
    let credits_source = credits_request.source.value;
    let mut credits_fields = Vec::with_capacity(credits_response.len());

    for role in credits_response {
        if !role.artists.is_empty() {
            let name = role.role_title;
            let artists = role
                .artists
                .iter()
                .map(|artist: &Artist| {
                    let name = &artist.name;
                    let uri = &artist.uri;
                    let artist_id = uri.replace("spotify:artist:", "");
                    let artist_url = format!("https://open.spotify.com/artist/{}", artist_id);
                    format!("[{}]({})", name, artist_url)
                })
                .join("\n");

            credits_fields.push((name, artists, true));
        }
    }

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title(format!("Credits for {} by {}", track_name, track_artist));
            embed.color(0x001D_B954);
            embed.thumbnail(track_image);
            embed.url(track_url);
            embed.fields(credits_fields);
            embed.footer(|footer| footer.text(format!("Credits provided by {} | Powered by the Spotify API.", credits_source)))
        })
    })?;

    Ok(())
}
