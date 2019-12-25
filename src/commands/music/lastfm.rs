//! Last.fm command
//!
//! Retrieves a chosen user's last.fm state, along with various
//! user information such as their most recent tracks.

use itertools::Itertools;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use rustfm::Client;
use rustfm::user::recent_tracks::Track;

use std::env;

#[command]
#[description("Retrieves various Last.fm user stats.")]
#[usage("<user> <limit>")]
pub fn lastfm(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    if args.rest().is_empty() {
        let _ = message.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Error: No Last.fm username was provided.");
                e.description("You did not provide a Last.fm username. Please enter one and then try again.");
                e.color(0x00FF_0000);
                e
            });
            m
        })?;
        println!("No Last.fm username was provided.");
    }

    let user: String = args.single::<String>()?;
    let mut limit: usize = 5;

    match args.single() {
        Ok(value) => limit = value,
        Err(_e) => {}
    };

    let api_key: String = env::var("LASTFM_KEY").expect("No API key detected");
    let mut client: Client = Client::new(&api_key);

    let recent_tracks = client.recent_tracks(&user).with_limit(limit).send().unwrap().tracks;
    let loved_tracks = client.loved_tracks(&user).with_limit(1).send().unwrap().attrs.total;
    let track = recent_tracks.first().unwrap();

    let tracks: String;

    match recent_tracks.is_empty() {
        true => {
            println!("No tracks available :(");
            tracks = "No tracks available".to_string();
        }
        false => {
            tracks = recent_tracks.iter().map(|t: &Track| {
                let mut now_playing: String = "".to_string();

                match t.attrs.as_ref().is_none() {
                    true => println!("No track attributes associated with this track."),
                    false => now_playing = "\x5c▶️".to_string()
                }

                format!("{} **{}** — {}", now_playing, t.name, t.artist.name) 
            }).join("\n");
        }
    };

    let track_play_state: String;
    match track.attrs.as_ref().is_none() {
        true => track_play_state = "last listened to".to_string(),
        false => track_play_state = "is currently listening to".to_string()
    }

    let currently_playing: String = format!(
        "{} {} {} by {} on {}.",
        user, track_play_state, track.name, track.artist.name, track.album.name
    );

    let _ = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title(format!("{}'s Last.fm Details", user));
            e.color(0x00d5_1007);
            e.description(format!(
                "{}\n\n\
                **__User Information:__**\n\
                **Loved Tracks**: {}\n\n\
                **__Recently Played:__**\n\
                {}",
                currently_playing, loved_tracks, tracks,
            ));
            e.footer(|f| f.text("Powered by the Last.fm API."));
            e
        });
        m
    })?;

    Ok(())
}
