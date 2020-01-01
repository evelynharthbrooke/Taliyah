use chrono::prelude::*;

use humantime::format_duration;

use itertools::Itertools;

use std::time::Duration;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::prelude::Message;

use crate::spotify;

#[command]
#[description("Displays information about a specified track on Spotify.")]
fn track(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        return msg
            .channel_id
            .send_message(&ctx, move |m| {
                m.embed(move |e| {
                    e.title("Error: No track name provided.");
                    e.description(
                        "You did not provide a track name. Please enter one and \
                        then try again.",
                    );
                    e
                })
            })
            .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
    }

    let track_name = args.rest();
    let track_search = spotify().search_track(&track_name, 1, 0, None);
    let track_result = &track_search.unwrap().tracks.items;
    
    let track = track_result.first().unwrap();
    let track_id = &track.id.clone().unwrap();
    let track_album = track.album.clone();
    let track_name = &track.name;
    let track_album_name = &track_album.name;
    let track_album_url = &track_album.external_urls["spotify"];
    let track_date = track_album.release_date.unwrap();
    let track_explicit = match track.explicit {
        true => "Yes".to_owned(),
        false => "No".to_owned()
    };
    let track_popularity = &track.popularity;
    let track_position = &track.track_number;
    let track_preview_url = track.preview_url.as_ref().unwrap();
    let track_markets = &track.available_markets.len();
    let track_length = format_duration(Duration::from_millis(track.duration_ms as u64 / 1000 * 1000));
    let track_url = &track.external_urls["spotify"];
    let track_image = &track_album.images.first().unwrap().url;
    let track_artists = &track.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");
    let track_date = NaiveDate::parse_from_str(&track_date, "%Y-%m-%d").map_or(track_date, move |d| {
        let formatted_string = d.format("%B %-e, %Y").to_string();
        format!("{}", formatted_string.trim())
    });

    let track_analysis = spotify().audio_analysis(track_id).unwrap().track;

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
        _ => track_analysis.key.to_string(),
    };

    let track_key_confidence = track_analysis.key_confidence;
    let track_loudness = track_analysis.loudness;
    let track_mode = match track_analysis.mode as u32 {
        0 => "Minor".to_owned(),
        1 => "Major".to_owned(),
        _ => track_analysis.mode.to_string() 
    };
    let track_mode_confidence = track_analysis.mode_confidence;
    let track_tempo = track_analysis.tempo;
    let track_tempo_confidence = track_analysis.tempo_confidence;
    let track_time_signature = track_analysis.time_signature;
    let track_time_signature_confidence = track_analysis.time_signature_confidence;

    msg.channel_id.send_message(&ctx, move |m| {
        m.embed(move |e| {
            e.title(track_name);
            e.url(track_url);
            e.color(0x1DB954);
            e.thumbnail(track_image);
            e.description(format!(
                "\
                **Artist(s)**: {}\n\
                **Album**: [{}]({}) (track #{})\n\
                **Popularity**: {}\n\
                **Explicit?**: {}\n\
                **Release date**: {}\n\
                **Play preview**: [click here]({})\n\
                **Markets**: {}\n\
                **Length**: {}\n\
                **Loudness**: {} dB\n\
                **Key**: {} (confidence: {})\n\
                **Mode**: {} (confidence: {})\n\
                **Tempo**: {} (confidence: {})\n\
                **Time Signature**: {} (confidence: {})\n\n\
                [Play {} on Spotify →]({})
                ", 
                track_artists, track_album_name, track_album_url, track_position, 
                track_popularity, track_explicit, track_date, track_preview_url, track_markets, 
                track_length, track_loudness, track_key, track_key_confidence, track_mode, 
                track_mode_confidence,track_tempo, track_tempo_confidence, track_time_signature, 
                track_time_signature_confidence, track_name, track_url
            ));
            e.footer(|f| f.text("Powered by the Spotify Web API."))
        })
    }).map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}
