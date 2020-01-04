use chrono::prelude::*;

use humantime::format_duration;

use itertools::Itertools;

use rspotify::spotify::model::track::SimplifiedTrack;

use std::time::Duration;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::prelude::Message;

use crate::spotify;

#[command]
#[description("Displays information about a specified album on Spotify.")]
fn album(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        return msg.channel_id.send_message(&ctx, move |m| {
                m.embed(move |e| {
                    e.title("Error: No album name provided.");
                    e.description(
                        "You did not provide an album name. Please enter one and \
                        then try again.",
                    );
                    e
                })
            })
            .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
    }

    let album_name = args.rest();

    let album_search = spotify().search_album(&album_name, 1, 0, None);
    let album_result = &album_search.unwrap().albums.items;
    let album_id = album_result.first().unwrap().id.as_ref().unwrap();

    let album = spotify().album(album_id).unwrap();
    let album_type = match album.album_type.clone().as_str() {
        "album" => "Album".to_owned(),
        "single" => "Single".to_owned(),
        "appears_on" => "Appears On".to_owned(),
        "compilation" => "Compilation".to_owned(),
        &_ => album.album_type.as_str().to_owned()
    };
    let album_name = &album.name;
    let album_url = &album.external_urls["spotify"];
    let album_image = &album.images.first().unwrap().url;
    
    let mut album_markets = album.available_markets.len().to_string();

    // This will have to be updated as Spotify is launched
    // in more markets / countries.
    if album_markets == "79" {
        album_markets.push_str(" (Worldwide)");
    }
    
    let album_artists = &album.artists.iter().map(|a| {
        format!("[{}]({})", &a.name, &a.external_urls["spotify"])
    }).join(", ");
    
    let album_date = NaiveDate::parse_from_str(&album.release_date, "%Y-%m-%d").map_or(album.release_date, move |d| {
        let formatted_string = d.format("%B %-e, %Y").to_string();
        format!("{}", formatted_string.trim())
    });

    let album_copyright = match album.copyrights.is_empty() {
        true => album.label,
        false => format!("{} ({})", album.copyrights.first().unwrap()["text"], album.label)
    };

    let album_tracks_total = album.tracks.total;
    let album_tracks = album.tracks.items.iter().map(|track: &SimplifiedTrack| {
        let name = &track.name;
        let position = &track.track_number;
        let external_link = &track.external_urls["spotify"];
        let length = format_duration(Duration::from_millis(track.duration_ms as u64 / 1000 * 1000));

        let explicit = match track.explicit {
            true => "— Explicit".to_string(),
            false => "".to_string()
        };

        return format!("**{}.** [{}]({}) {} — {}", position, name, external_link, explicit, length);
    }).join("\n");
    
    msg.channel_id.send_message(&ctx, move |m| {
            m.embed(move |e| {
                e.title(album_name);
                e.url(album_url);
                e.color(0x1DB954);
                e.thumbnail(album_image);
                e.description(format!(
                    "\
                    **Album type**: {}\n\
                    **Artist(s)**: {}\n\
                    **Release date**: {}\n\
                    **Markets**: {}\n\
                    **Track count**: {}\n\n\
                    **Tracks**: \n{}\n\
                    ",
                    album_type, album_artists, album_date, album_markets, album_tracks_total,
                    album_tracks
                ));
                e.footer(|f| {
                    f.text(format!("{}", album_copyright))
                })
            })
        }).map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}
