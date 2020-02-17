use chrono::prelude::*;

use humantime::format_duration;

use itertools::Itertools;

use rspotify::spotify::model::track::SimplifiedTrack;

use std::time::Duration;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use crate::spotify;

#[command]
#[description("Displays information about a specified album on Spotify.")]
fn album(context: &mut Context, message: &Message, args: Args) -> CommandResult {
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

    let album_name = args.rest();

    let album_search = spotify().search_album(&album_name, 1, 0, None);
    let album_result = &album_search.unwrap().albums.items;
    let album_id = album_result.first().unwrap().id.as_ref().unwrap();

    let album = spotify().album(album_id).unwrap();
    let album_name = &album.name;
    let album_popularity = &album.popularity;
    let album_url = &album.external_urls["spotify"];
    let album_image = &album.images.first().unwrap().url;

    let mut album_type = match album.album_type.clone().as_str() {
        "album" => "Album".to_owned(),
        "single" => "Single".to_owned(),
        "appears_on" => "Appears On".to_owned(),
        "compilation" => "Compilation".to_owned(),
        &_ => album.album_type.as_str().to_owned(),
    };

    let mut album_markets = album.available_markets.len().to_string();

    let album_genres = if album.genres.is_empty() {
        "No genres available.".to_string()
    } else {
        album.genres.iter().map(|g| g).join(", ")
    };

    // This will have to be updated as Spotify is launched
    // in more markets / countries.
    if album_markets == "79" {
        album_markets.push_str(" (Worldwide)");
    }

    let album_artists = &album.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");

    let album_date = match NaiveDate::parse_from_str(&album.release_date, "%Y-%m-%d") {
        Ok(date) => date.format("%B %-e, %Y").to_string(),
        Err(_) => album.release_date,
    };

    let album_copyright = if album.copyrights.is_empty() {
        album.label
    } else {
        format!("{} ({})", album.copyrights.first().unwrap()["text"], album.label)
    };

    let album_tracks_total = album.tracks.total;

    if album_tracks_total <= 6 && album_tracks_total > 1 {
        album_type = "Extended Play (EP)".to_string()
    }

    let album_track_items = &album.tracks.items;

    let mut album_length: u32 = 0;

    // Iterate through an album's tracks, adding each track's
    // length in milliseconds to the album_length variable, in 
    // order to get the total length of the album.
    for item in album_track_items {
        album_length += item.duration_ms;
    }

    let album_tracks = album_track_items
        .iter()
        .map(|track: &SimplifiedTrack| {
            let name = &track.name;
            let position = &track.track_number;
            let external_link = &track.external_urls["spotify"];
            let length = format_duration(Duration::from_millis(track.duration_ms as u64 / 1000 * 1000));
            let explicit = if track.explicit { "(explicit)".to_string() } else { "".to_string() };
            return format!("**{}.** [{}]({}) — {} {}", position, name, external_link, length, explicit);
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
            embed.description(format!(
                "\
                **Type**: {}\n\
                **Length**: {}\n\
                **Artist(s)**: {}\n\
                **Released**: {}\n\
                **Genres**: {}\n\
                **Popularity**: {}\n\
                **Markets**: {}\n\
                **Tracks**: {}\n\n\
                **Tracklist**:\n{}
                ",
                album_type,
                format_duration(Duration::from_millis(album_length as u64 / 1000 * 1000)),
                album_artists,
                album_date,
                album_genres,
                album_popularity,
                album_markets,
                album_tracks_total,
                album_tracks
            ));
            embed.footer(|footer| footer.text(album_copyright))
        })
    })?;

    Ok(())
}
