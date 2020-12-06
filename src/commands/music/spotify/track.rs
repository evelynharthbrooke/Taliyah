use crate::utils::spotify;
use chrono::prelude::NaiveDate;
use humantime::format_duration;
use itertools::Itertools;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use rspotify::{
    model::{page::Page, search::SearchResult, track::FullTrack},
    senum::SearchType
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::time::Duration;

#[command]
#[description("Displays information about a specified track on Spotify.")]
pub async fn track(context: &Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message
            .channel_id
            .send_message(context, |embed| {
                embed.embed(|embed| {
                    embed.title("Error: No track name provided.");
                    embed.color(0x00FF_0000);
                    embed.description("No track name provided. Please provide one.");
                    embed
                })
            })
            .await?;
        return Ok(());
    }

    let track_query = utf8_percent_encode(args.rest(), NON_ALPHANUMERIC).to_string();
    let track_search = spotify().await.search(&track_query, SearchType::Track, 1, 0, None, None).await;
    let track_result = &track_search.unwrap();

    match track_result {
        SearchResult::Tracks(tracks) => {
            let tracks: &Page<FullTrack> = tracks;
            let items: &Vec<FullTrack> = &tracks.items;

            if items.is_empty() {
                message
                    .channel_id
                    .send_message(context, |message| {
                        message.embed(|embed| {
                            embed.title("Error: No track found.");
                            embed.color(0x00FF_0000);
                            embed.description(format!("No track found matching {}. Try a different search term.", track_query));
                            embed
                        })
                    })
                    .await?;

                return Ok(());
            }

            let track = items.first().unwrap();
            let track_id = &track.id.clone().unwrap();
            let track_album = track.album.clone();
            let track_album_id = &track_album.id.unwrap();
            let track_name = &track.name;

            let track_album = spotify().await.album(track_album_id).await.unwrap();

            let track_album_name = &track_album.name;
            let track_album_url = track_album.external_urls.get("spotify").unwrap();
            let track_label = track_album.label;
            let track_markets = track.available_markets.len().to_string();
            let track_length = format_duration(Duration::from_millis(u64::from(track.duration_ms) / 1000 * 1000));
            let track_url = track.external_urls.get("spotify").unwrap();
            let track_explicit = track.explicit;
            let track_popularity = &track.popularity;
            let track_position = &track.track_number;
            let track_disc = &track.disc_number;
            let track_image = &track_album.images.first().unwrap().url;
            let track_artists = track.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");

            let track_date = match NaiveDate::parse_from_str(&track_album.release_date, "%Y-%m-%d") {
                Ok(date) => date.format("%B %-e, %Y").to_string(),
                Err(_) => track_album.release_date
            };

            let track_copyright = match &track_album.copyrights.is_empty() {
                true => track_label,
                false => {
                    let copyright = &track_album.copyrights.first().unwrap()["text"];
                    format!("{} ({})", copyright, track_label)
                }
            };

            let track_preview_url = if track.preview_url.is_none() {
                "No preview available.".to_owned()
            } else {
                format!("[Click Here]({})", track.preview_url.as_ref().unwrap())
            };

            let track_analysis = spotify().await.audio_analysis(track_id).await.unwrap().track;

            let track_key = match track_analysis.key {
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
                _ => track_analysis.key.to_string()
            };

            let track_loudness = track_analysis.loudness;
            let track_tempo = track_analysis.tempo;
            let track_time_signature = track_analysis.time_signature;
            let track_mode = match track_analysis.mode as u32 {
                0 => "Minor".to_owned(),
                1 => "Major".to_owned(),
                _ => track_analysis.mode.to_string()
            };

            message
                .channel_id
                .send_message(&context, |message| {
                    message.embed(|embed| {
                        embed.title(track_name);
                        embed.thumbnail(track_image);
                        embed.url(track_url);
                        embed.color(0x001D_B954);
                        embed.fields(vec![
                            ("Artists", track_artists, true),
                            ("Album", format!("[{}]({})", track_album_name, track_album_url), true),
                            ("Disc", track_disc.to_string(), true),
                            ("Position", track_position.to_string(), true),
                            ("Release Date", track_date, true),
                            ("Popularity", format!("{}%", track_popularity), true),
                            ("Explicit", track_explicit.to_string(), true),
                            ("Song Preview", track_preview_url, true),
                            ("Markets", track_markets, true),
                            ("Duration", track_length.to_string(), true),
                            ("Loudness", format!("{} dB", track_loudness), true),
                            ("Keys", track_key, true),
                            ("Mode", track_mode, true),
                            ("Tempo", track_tempo.to_string(), true),
                            ("Time Signature", track_time_signature.to_string(), true),
                        ]);
                        embed.footer(|footer| footer.text(track_copyright))
                    })
                })
                .await?;
        }
        err => println!("Error while retrieving track: {:?}", err)
    }

    Ok(())
}
