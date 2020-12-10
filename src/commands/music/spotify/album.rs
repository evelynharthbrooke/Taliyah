use chrono::prelude::*;
use humantime::format_duration;
use itertools::Itertools;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use rspotify::{
    model::{album::SimplifiedAlbum, page::Page, search::SearchResult, track::SimplifiedTrack},
    senum::{AlbumType, SearchType}
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::time::Duration;

use crate::utils::spotify;

#[command]
#[description("Displays information about a specified album on Spotify.")]
async fn album(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message
            .channel_id
            .send_message(&context, |message| {
                message.embed(|embed| {
                    embed.title("Error: No album name provided.");
                    embed.color(0x00FF_0000);
                    embed.description("You did not provide an album name. Please enter one and then try again.")
                })
            })
            .await?;
        return Ok(());
    }

    let album_name = utf8_percent_encode(args.rest(), NON_ALPHANUMERIC).to_string();
    let album_search = spotify().await.search(&album_name, SearchType::Album, 1, 0, None, None).await;
    let album_result = &album_search.unwrap();

    match album_result {
        SearchResult::Albums(albums) => {
            let albums: &Page<SimplifiedAlbum> = albums;
            let items = &albums.items;

            if items.is_empty() {
                message
                    .channel_id
                    .send_message(context, |message| {
                        message.embed(|embed| {
                            embed.title("Error: No album found.");
                            embed.color(0x00FF_0000);
                            embed.description(format!("No albums found matching {}. Try a different search term.", album_name));
                            embed
                        })
                    })
                    .await?;
                return Ok(());
            }

            let album_id = items.first().unwrap().id.as_ref().unwrap();

            let album = spotify().await.album(album_id).await.unwrap();
            let album_name = &album.name;
            let album_artists = &album.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");
            let album_popularity = &album.popularity;
            let album_image = &album.images.first().unwrap().url;
            let album_markets = album.available_markets.len();
            let album_track_count = album.tracks.total;
            let album_url = &album.external_urls["spotify"];

            let album_type = match album.album_type {
                AlbumType::Album => "Album".to_owned(),
                AlbumType::Single => {
                    if album_track_count <= 6 && album_track_count > 1 {
                        "Extended Play (EP)".to_string()
                    } else {
                        "Single".to_owned()
                    }
                }
                AlbumType::AppearsOn => "Appears On".to_owned(),
                AlbumType::Compilation => "Compilation".to_owned()
            };

            let album_date = match NaiveDate::parse_from_str(&album.release_date, "%Y-%m-%d") {
                Ok(date) => date.format("%B %-e, %Y").to_string(),
                Err(_) => album.release_date
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

            let album_fields = vec![
                ("Type", album_type, true),
                ("Length", album_length.to_string(), true),
                ("Artists", album_artists.to_string(), true),
                ("Release Date", album_date, true),
                ("Popularity", format!("{}%", album_popularity), true),
                ("Markets", album_markets.to_string(), true),
                ("Tracks", album_track_count.to_string(), true),
            ];

            message
                .channel_id
                .send_message(context, |message| {
                    message.embed(|embed| {
                        embed.title(album_name);
                        embed.url(album_url);
                        embed.thumbnail(album_image);
                        embed.color(0x001D_B954);
                        embed.fields(album_fields);
                        embed.description(album_tracks);
                        embed.footer(|footer| footer.text(album_copyright))
                    })
                })
                .await?;
        }
        err => println!("Error while retrieving album: {:?}", err)
    }

    Ok(())
}
