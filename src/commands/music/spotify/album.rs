use aspotify::{AlbumType, ItemType};
use humantime::format_duration;
use itertools::Itertools;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::time::Duration;

use crate::data::SpotifyContainer;

#[command]
#[description("Displays information about a specified album on Spotify.")]
async fn album(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.say(context, "No album name provided. Please provide one & try again.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let spotify = data.get::<SpotifyContainer>().unwrap();

    let album_search = spotify.search().search(args.rest(), [ItemType::Album].iter().copied(), false, 1, 0, None);
    let album_result = &album_search.await.unwrap().data;
    let albums = album_result.albums.clone();
    let items = albums.unwrap().items;

    if items.is_empty() {
        message.channel_id.say(context, format!("No album found for `{}`. Try a different name.", args.rest())).await?;
        return Ok(());
    }

    let album_id = items.first().unwrap().id.as_ref().unwrap();
    let album = spotify.albums().get_album(album_id, None).await.unwrap().data;

    let album_name = &album.name;
    let album_date = album.release_date.to_string();
    let album_artists = &album.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");
    let album_popularity = &album.popularity;
    let album_image = &album.images.first().unwrap().url;
    let album_markets = album.available_markets.unwrap().len();
    let album_track_count = album.tracks.total;
    let album_url = &album.external_urls["spotify"];

    let album_type = match album.album_type {
        AlbumType::Album => "Album".to_owned(),
        AlbumType::Single => {
            if album_track_count > 1 && album_track_count <= 6 {
                "Extended Play (EP)".to_string()
            } else {
                "Single".to_owned()
            }
        }
        AlbumType::Compilation => "Compilation".to_owned()
    };

    let album_copyright = if album.copyrights.is_empty() {
        album.label
    } else {
        format!("{} ({})", album.copyrights.first().unwrap().text, album.label)
    };

    let album_track_items = &album.tracks.items;
    let album_track_lengths: u64 = album_track_items.iter().map(|track| track.duration.as_millis() as u64).sum();
    let album_length = format_duration(Duration::from_millis(album_track_lengths / 1000 * 1000));

    let album_tracks = album_track_items
        .iter()
        .map(|track| {
            let name = &track.name;
            let position = &track.track_number;
            let url = &track.external_urls["spotify"];
            let length = format_duration(Duration::from_millis((track.duration.as_millis() as u64) / 1000 * 1000));
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

    Ok(())
}
