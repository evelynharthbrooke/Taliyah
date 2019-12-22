//! Last.fm command
//!
//! Retrieves a chosen user's last.fm state, along with various
//! user information such as their most recent tracks.

extern crate reqwest;
extern crate rustfm;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use rustfm::Client;

use std::env;

#[command]
#[description("Retrieves various Last.fm user stats.")]
#[usage("<user> <limit>")]
pub fn lastfm(ctx: &mut Context, message: &Message, mut args: Args) -> CommandResult {

    if args.rest().is_empty() {
        let _ = message.channel_id.send_message(&ctx, |m| {
            m.embed(|e| {
                e.title("Error: No last.fm username was provided.");
                e.description("You did not provide a Last.fm username. Please enter one and then try again.");
                e.color(0x00FF_0000);
                e
            });
            m
        })?;
        println!("No Last.fm username was provided.");
    }
    
    let username: String = args.single::<String>()?;
    let mut limit: usize = 4;

    match args.single() {
        Ok(value) => limit = value,
        Err(_e) => {}
    };

    let api_key: String = env::var("LASTFM_KEY").expect("No API key detected");
    let mut client: Client = Client::new(&api_key);

    let recent_tracks = client.recent_tracks(&username).with_limit(limit).send().unwrap().tracks;
    let loved_tracks = client.loved_tracks(&username).with_limit(1).send().unwrap().attrs.total;

    let track = recent_tracks.first().unwrap();

    let mut track_strings: Vec<String> = Vec::with_capacity(limit);
    for t in &recent_tracks {
        let now_playing: &str;

        if t.attrs.as_ref().is_none() {
            now_playing = ""
        } else {
            now_playing = "\x5c▶️"
        }

        track_strings.push(format!("{} **{}** — {}", now_playing, &t.name, &t.artist.name));
    }

    let track_play_state: &str;
    if track.attrs.as_ref().is_none() {
        track_play_state = "last listened to"
    } else {
        track_play_state = "is currently listening to"
    }

    let _ = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title(format!("{}'s Last.fm Details", username));
            e.color(0x00d5_1007);
            e.description(format!(
                "{} {} {} by {} on {}.\n
                **__User Information:__**
                **Loved Tracks**: {}\n
                **__Recently Played:__**\n{}",
                username,
                track_play_state,
                &track.name,
                &track.artist.name,
                &track.album.name,
                loved_tracks,
                track_strings.join("\n")
            ));
            e.footer(|f| f.text("Powered by the Last.fm API."));
            e
        });
        m
    })?;

    Ok(())
}
