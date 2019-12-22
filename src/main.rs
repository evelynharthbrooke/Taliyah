//! Ellie for Discord
//!
//! Ellie is a bot for the Discord chat platform focused on giving users
//! a powerful set of features, while remaining quick to respond.

mod commands;

use dotenv::dotenv;

use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::user::OnlineStatus;

use std::collections::HashSet;
use std::env;

use commands::music::lastfm::*;
use commands::utils::help::*;
use commands::utils::ping::*;

// Define the Handler struct.
struct Handler;
impl EventHandler for Handler {
    // When the ready event happens, print a message to the
    // console letting us know which user authenticated with
    // the Discord API, and also print the amount of guilds
    // we are currently connected to.
    fn ready(&self, ctx: Context, ready: Ready) {
        // Print that we have logged into the Discord API.
        println!(
            "Successfully logged into the Discord API as {}#{}. (ID: {})",
            ready.user.name, ready.user.discriminator, ready.user.id
        );

        // Print how many guilds we are currently connected to.
        println!("Connected to {} guild(s).", ready.guilds.len());

        // Set the bot's presence.
        ctx.set_presence(Some(Activity::playing("!help")), OnlineStatus::Online);
    }

    // Handle the resume event. Doesn't do much yet.
    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed!");
    }
}

pub fn main() {
    // Initialize the reading of the .env environment file.
    dotenv().expect("Unable to read / access the .env file!");

    let token: String = env::var("DISCORD_TOKEN").expect("Unable to read the bot token.");
    let prefix: String = env::var("DISCORD_PREFIX").expect("Unable to get the bot prefix.");

    let mut client: Client = Client::new(&token, Handler).expect("Error creating client.");

    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Couldn't get app info: {:?}", why),
    };

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.with_whitespace(true).prefix(&prefix).owners(owners).on_mention(Some(bot_id)))
            .help(&HELP)
            .group(&UTILITIES_GROUP)
            .group(&MUSIC_GROUP),
    );

    if let Err(err) = client.start() {
        println!("An error occurred while running the client: {:?}", err);
    }
}

group!({
    name: "Utilities",
    options: {
        description: "Ellie's selection of utility commands."
    },
    commands: [ping]
});

group!({
    name: "Music",
    options: {
        description: "Various music-focused commands."
    },
    commands: [lastfm]
});
