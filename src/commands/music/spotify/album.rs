use crate::spotify;

use chrono::prelude::*;

use humantime::format_duration;

use itertools::Itertools;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use rspotify::spotify::model::track::SimplifiedTrack;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use std::time::Duration;

#[command]
#[description("Displays information about a specified album on Spotify.")]
fn album(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    message.channel_id.broadcast_typing(&context)?;

    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: No album name provided.");
                embed.color(0x00FF_0000);
                embed.description("You did not provide an album name. Please enter one and then try again.")
            })
        })?;

        return Ok(());
    }

    let album_name = utf8_percent_encode(args.rest(), NON_ALPHANUMERIC).to_string();
    let album_search = spotify().search_album(&album_name, 1, 0, None);
    let album_result = &album_search.unwrap().albums.items;

    if album_result.is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: No album found.");
                embed.color(0x00FF_0000);
                embed.description(format!(
                    "I was unable to to find an album on Spotify matching the term `{}`. \
                    Please try looking for a different album, or try again later.",
                    album_name
                ))
            })
        })?;

        return Ok(());
    }

    let album_id = album_result.first().unwrap().id.as_ref().unwrap();

    let album = spotify().album(album_id).unwrap();
    let album_name = &album.name;
    let album_artists = &album.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");
    let album_popularity = &album.popularity;
    let album_image = &album.images.first().unwrap().url;
    let album_markets = album.available_markets.len();
    let album_track_count = album.tracks.total;
    let album_url = &album.external_urls["spotify"];

    let mut album_type = match album.album_type.clone().as_str() {
        "album" => "Album".to_owned(),
        "single" => "Single".to_owned(),
        "appears_on" => "Appears On".to_owned(),
        "compilation" => "Compilation".to_owned(),
        &_ => album.album_type.as_str().to_owned(),
    };

    if album_track_count <= 6 && album_track_count > 1 {
        album_type = "Extended Play (EP)".to_string()
    }

    let album_date = match NaiveDate::parse_from_str(&album.release_date, "%Y-%m-%d") {
        Ok(date) => date.format("%B %-e, %Y").to_string(),
        Err(_) => album.release_date,
    };

    let album_copyright = if album.copyrights.is_empty() {
        album.label
    } else {
        format!("{} ({})", album.copyrights.first().unwrap()["text"], album.label)
    };

    let album_track_items = &album.tracks.items;
    let album_track_lengths: u32 = album_track_items.iter().map(|track| track.duration_ms).sum();
    let album_length = format_duration(Duration::from_millis(u64::from(album_track_lengths) / 1000 * 1000));

    let album_tracks = album_track_items
        .iter()
        .map(|track: &SimplifiedTrack| {
            let name = &track.name;
            let position = &track.track_number;
            let url = &track.external_urls["spotify"];
            let length = format_duration(Duration::from_millis(u64::from(track.duration_ms) / 1000 * 1000));
            let explicit = if track.explicit { "(explicit)" } else { "" };
            format!("**{}.** [{}]({}) — {} {}", position, name, url, length, explicit)
        })
        .join("\n");

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(album_name);
                author.url(album_url);
                author.icon_url(album_image)
            });
            embed.color(0x001D_B954);
            embed.fields(vec![
                ("Type", album_type, true),
                ("Length", album_length.to_string(), true),
                ("Artists", album_artists.to_string(), true),
                ("Release Date", album_date, true),
                ("Popularity", format!("{}%", album_popularity), true),
                ("Markets", album_markets.to_string(), true),
                ("Tracks", album_track_count.to_string(), true),
            ]);
            embed.description(album_tracks);
            embed.footer(|footer| footer.text(album_copyright))
        })
    })?;

    Ok(())
}
