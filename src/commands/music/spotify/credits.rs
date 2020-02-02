use crate::spotify;
use itertools::Itertools;
use kuchiki;
use kuchiki::traits::*;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "accessToken")]
    access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct Credits {
    label: String,
    #[serde(rename = "trackUri")]
    track_uri: String,
    #[serde(rename = "trackTitle")]
    track_title: String,
    #[serde(rename = "roleCredits")]
    role_credits: Vec<Role>,
    source: Source,
}

#[derive(Deserialize, Debug)]
pub struct Role {
    #[serde(rename = "roleTitle")]
    role_title: String,
    artists: Vec<Artist>,
}

#[derive(Deserialize, Debug)]
pub struct Artist {
    uri: String,
    name: String,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    label: String,
    value: String,
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
                        then try again.",
                )
            })
        })?;
        return Ok(());
    }

    let track_name = args.rest();
    let track_name_encoded = utf8_percent_encode(&track_name, NON_ALPHANUMERIC).to_string();
    let track_search = spotify().search_track(&track_name_encoded, 1, 0, None);
    let track_result = &track_search.unwrap().tracks.items;
    let track = track_result.first().unwrap();
    let track_name = &track.name;
    let track_url = &track.external_urls["spotify"];
    let track_artist = &track.artists.first().unwrap().name;
    let track_image = &track.album.images.first().unwrap().url;
    let track_id = &track.id.as_ref().unwrap();

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
    let access_token = config_json.access_token;
    let spclient_url = format!("https://spclient.wg.spotify.com/track-credits-view/v0/track/{}/credits", track_id);
    let credits_request: Credits = client.get(&spclient_url).bearer_auth(access_token).send()?.json()?;
    let credits = credits_request
        .role_credits
        .iter()
        .map(|role: &Role| {
            let name = match role.role_title.as_str() {
                "Performers" => "Performed by",
                "Writers" => "Written by",
                "Producers" => "Produced by",
                _ => "Unknown credit title",
            };

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

            format!("**{}**:\n{}", name, artists)
        })
        .join("\n\n");

    let credit_source = credits_request.source.value;

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title(format!("Credits for {} by {}", track_name, track_artist));
            embed.color(0x001D_B954);
            embed.thumbnail(track_image);
            embed.url(track_url);
            embed.description(credits);
            embed.footer(|footer| footer.text(format!("Credits provided by {} | Powered by the Spotify API.", credit_source)))
        })
    })?;

    Ok(())
}
