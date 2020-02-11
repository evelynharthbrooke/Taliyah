use crate::spotify;

use chrono::prelude::*;

use humantime::format_duration;

use itertools::Itertools;

use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use std::time::Duration;

#[command]
#[description("Displays information about a specified track on Spotify.")]
fn track(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |embed| {
            embed.embed(|embed| {
                embed.title("Error: No track name provided.");
                embed.color(0x00FF_0000);
                embed.description("You did not provide a track name. Please enter one and then try again.")
            })
        })?;
        return Ok(());
    }

    let track_name = args.rest();
    let track_name_encoded = utf8_percent_encode(&track_name, NON_ALPHANUMERIC).to_string();
    let track_search = spotify().search_track(&track_name_encoded, 1, 0, None);
    let track_result = &track_search.unwrap().tracks.items;

    let track = track_result.first().unwrap();
    let track_id = &track.id.clone().unwrap();
    let track_album = track.album.clone();
    let track_album_id = &track_album.id.unwrap();
    let track_name = &track.name;

    let track_album = spotify().album(track_album_id).unwrap();

    let track_album_name = &track_album.name;
    let track_album_url = &track_album.external_urls["spotify"];
    let track_label = track_album.clone().label;
    let mut track_date = track_album.release_date;

    track_date = NaiveDate::parse_from_str(&track_date, "%Y-%m-%d").map_or(track_date, |d| d.format("%B %-e, %Y").to_string());

    let track_copyright = match &track_album.copyrights.is_empty() {
        true => track_album.label,
        false => {
            let copyright = &track_album.copyrights.first().unwrap()["text"];
            format!("{} ({})", copyright, track_label)
        }
    };

    let track_explicit = if track.explicit {
        "Yes.".to_owned()
    } else {
        match track_label.as_str() {
            "Walt Disney Records" => "Of course not, it's Disney.".to_owned(),
            _ => "No.".to_owned(),
        }
    };

    let track_popularity = &track.popularity;
    let track_position = &track.track_number;
    let track_disc = &track.disc_number;

    let track_preview_url = if track.preview_url.is_none() {
        "No preview available.".to_owned()
    } else {
        format!("[click here]({})", track.preview_url.as_ref().unwrap())
    };

    let mut track_markets = track.available_markets.len().to_string();

    // This will have to be updated as Spotify is launched
    // in more markets / countries.
    if track_markets == "79" {
        track_markets.push_str(" (Worldwide)");
    }

    let track_length = format_duration(Duration::from_millis(track.duration_ms as u64 / 1000 * 1000));
    let track_url = &track.external_urls["spotify"];
    let track_image = &track_album.images.first().unwrap().url;
    let track_artists = &track.artists.iter().map(|a| format!("[{}]({})", &a.name, &a.external_urls["spotify"])).join(", ");

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
        _ => track_analysis.mode.to_string(),
    };

    let track_mode_confidence = track_analysis.mode_confidence;
    let track_tempo = track_analysis.tempo;
    let track_tempo_confidence = track_analysis.tempo_confidence;
    let track_time_signature = track_analysis.time_signature;
    let track_time_signature_confidence = track_analysis.time_signature_confidence;

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(track_name);
                author.url(track_url);
                author.icon_url(track_image)
            });
            embed.color(0x001D_B954);
            embed.description(format!(
                "
                **Artist(s)**: {}\n\
                **Album**: [{}]({}) (disc {}, track {})\n\
                **Popularity**: {}\n\
                **Explicit?**: {}\n\
                **Release date**: {}\n\
                **Song preview**: {}\n\
                **Markets**: {}\n\
                **Length**: {}\n\
                **Loudness**: {} dB\n\
                **Key**: {} (confidence: {})\n\
                **Mode**: {} (confidence: {})\n\
                **Tempo**: {} (confidence: {})\n\
                **Time Signature**: {} (confidence: {})\n\n\
                [Play {} on Spotify →]({})",
                track_artists,
                track_album_name,
                track_album_url,
                track_disc,
                track_position,
                track_popularity,
                track_explicit,
                track_date,
                track_preview_url,
                track_markets,
                track_length,
                track_loudness,
                track_key,
                track_key_confidence,
                track_mode,
                track_mode_confidence,
                track_tempo,
                track_tempo_confidence,
                track_time_signature,
                track_time_signature_confidence,
                track_name,
                track_url
            ));
            embed.footer(|footer| footer.text(track_copyright))
        })
    })?;

    Ok(())
}
