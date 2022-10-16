use aspotify::{ItemType, Mode};
use humantime::format_duration;
use itertools::Itertools;

use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::time::Duration;

use crate::data::SpotifyContainer;

#[command]
#[aliases("song")]
#[description("Displays information about a specified track on Spotify.")]
async fn track(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.say(context, "No track name provided. Please provide one & try again.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let spotify = data.get::<SpotifyContainer>().unwrap();

    let track_search = spotify.search().search(args.rest(), [ItemType::Track].iter().copied(), false, 1, 0, None);
    let track_result = &track_search.await.unwrap().data;
    let tracks = track_result.tracks.clone();
    let items = tracks.unwrap().items;

    if items.is_empty() {
        message.channel_id.say(context, format!("No track was found for `{}`. Try something else.", args.rest())).await?;
        return Ok(());
    }

    let track = items.first().unwrap();
    let track_id = &track.id.clone().unwrap();
    let track_album = track.album.clone();
    let track_album_id = &track_album.id.unwrap();
    let track_name = &track.name;

    let track_album = spotify.albums().get_album(track_album_id, None).await.unwrap().data;

    let track_album_name = &track_album.name;
    let track_album_url = track_album.external_urls.get("spotify").unwrap();
    let track_markets = track_album.available_markets.unwrap().len();
    let track_label = track_album.label;
    let track_length = format_duration(Duration::from_millis((track.duration.as_millis() as u64) / 1000 * 1000));
    let track_url = track.external_urls.get("spotify").unwrap();
    let track_explicit = track.explicit;
    let track_popularity = &track.popularity;
    let track_position = &track.track_number;
    let track_disc = &track.disc_number;
    let track_image = &track_album.images.first().unwrap().url;
    let track_artists = track.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");
    let track_date = track_album.release_date.to_string();

    let track_copyright = match &track_album.copyrights.is_empty() {
        true => track_label,
        false => {
            let copyright = &track_album.copyrights.first().unwrap().text;
            format!("{copyright} ({track_label})")
        }
    };

    let track_preview_url = if track.preview_url.is_none() {
        "No preview available.".to_owned()
    } else {
        format!("[Click Here]({})", track.preview_url.as_ref().unwrap())
    };

    let track_features = spotify.tracks().get_features_track(track_id).await.unwrap().data;

    let track_key = match track_features.key {
        0 => "C".to_owned(),
        1 => "C♯, D♭".to_owned(),
        2 => "D".to_owned(),
        3 => "D♯".to_owned(),
        4 => "E".to_owned(),
        5 => "F".to_owned(),
        6 => "F♯, G♭".to_owned(),
        7 => "G".to_owned(),
        8 => "G♯, A♭".to_owned(),
        9 => "A".to_owned(),
        10 => "A♯, B♭".to_owned(),
        11 => "B".to_owned(),
        _ => track_features.key.to_string()
    };

    let track_loudness = track_features.loudness;
    let track_tempo = track_features.tempo;
    let track_time_signature = track_features.time_signature;
    let track_mode = match track_features.mode {
        Mode::Major => "Minor".to_owned(),
        Mode::Minor => "Major".to_owned()
    };

    let embed = CreateEmbed::new()
        .title(track_name)
        .thumbnail(track_image)
        .url(track_url)
        .color(0x001D_B954)
        .fields(vec![
            ("Artists", track_artists, true),
            ("Album", format!("[{track_album_name}]({track_album_url})"), true),
            ("Disc", track_disc.to_string(), true),
            ("Position", track_position.to_string(), true),
            ("Release Date", track_date, true),
            ("Popularity", format!("{track_popularity}%"), true),
            ("Explicit", track_explicit.to_string(), true),
            ("Song Preview", track_preview_url, true),
            ("Markets", track_markets.to_string(), true),
            ("Duration", track_length.to_string(), true),
            ("Loudness", format!("{track_loudness} dB"), true),
            ("Keys", track_key, true),
            ("Mode", track_mode, true),
            ("Tempo", track_tempo.to_string(), true),
            ("Time Signature", track_time_signature.to_string(), true),
        ])
        .footer(CreateEmbedFooter::new(track_copyright));

    message.channel_id.send_message(&context, CreateMessage::new().add_embed(embed)).await?;

    Ok(())
}
