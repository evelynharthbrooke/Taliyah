use crate::spotify;
use crate::utilities::format_int;
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
struct Artist {
    bio: String,
    #[serde(rename = "headerImages")]
    header_images: Vec<HeaderImage>,
    #[serde(rename = "artistInsights")]
    artist_insights: ArtistInsights,
}

#[derive(Deserialize, Debug)]
struct HeaderImage {
    url: String,
}

#[derive(Deserialize, Debug)]
struct ArtistInsights {
    artist_gid: String,
    global_chart_position: usize,
    monthly_listeners: usize,
    monthly_listeners_delta: isize,
    follower_count: usize,
    following_count: usize, // Artists can't follow anyone, so I'm unsure why this value exists.
    playlists: Playlists,
}

#[derive(Deserialize, Debug)]
struct Playlists {
    entries: Vec<Playlist>,
}

#[derive(Deserialize, Debug)]
struct Playlist {
    uri: String,
    name: String,
    image_url: String,
    owner: Owner,
}

#[derive(Deserialize, Debug)]
struct Owner {
    name: String,
    uri: String,
}

#[command]
#[description("Displays information about a specified artist on Spotify.")]
fn artist(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: No artist name provided.");
                embed.description(
                    "\
                    You did not provide a artist name. Please enter one and \
                    then try again.\
                    ",
                )
            })
        })?;
        return Ok(());
    }

    let artist_name = args.rest();
    let artist_name_encoded = utf8_percent_encode(&artist_name, NON_ALPHANUMERIC).to_string();
    let artist_search = spotify().search_artist(&artist_name_encoded, 1, 0, None);
    let artist_result = &artist_search.unwrap().artists.items;
    let artist = artist_result.first().unwrap();

    let user_agent_chunk_1 = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko)";
    let user_agent_chunk_2 = "Chrome/81.0.4041.0 Safari/537.36 Edg/81.0.410.0";
    let user_agent = &[user_agent_chunk_1, user_agent_chunk_2].join(" ");
    let client = Client::builder().user_agent(user_agent).build()?;
    let access_token = get_spotify_token().unwrap();
    let access_token_str = access_token.as_str();
    let artists_url = format!("https://spclient.wg.spotify.com/open-backend-2/v1/artists/{}", artist.id);
    let artists_request: Artist = client.get(&artists_url).bearer_auth(access_token_str).send()?.json()?;

    let artist_name = &artist.name;
    let artist_url = &artist.external_urls["spotify"];
    let artist_genres = &artist.genres.iter().map(|genre| genre).join(", ");
    let artist_image = &artist.images.first().unwrap().url;
    let artist_id = &artist.id;
    let artist_followers = format_int(artists_request.artist_insights.follower_count);
    let artist_listeners = format_int(artists_request.artist_insights.monthly_listeners);
    let artist_position = format_int(artists_request.artist_insights.global_chart_position);

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.color(0x001D_B954);
            embed.title(artist_name);
            embed.url(artist_url);
            embed.thumbnail(artist_image);
            embed.description(format!(
                "\
                **Global chart position**: #{}\n\
                **Monthly listeners**: {}\n\
                **Followers**: {}\n\
                **Genres**: {}\n\
                ",
                artist_position, artist_listeners, artist_followers, artist_genres
            ));
            embed.footer(|footer| footer.text(format!("Spotify ID: {} | Powered by the Spotify API.", artist_id)))
        })
    })?;

    Ok(())
}
