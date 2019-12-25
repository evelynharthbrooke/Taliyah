//! Ellie for Discord
//!
//! Ellie is a bot for the Discord chat platform focused on giving users
//! a powerful set of features, while remaining quick to respond.

mod commands;
mod listeners;

use commands::info::user::*;
use commands::music::lastfm::*;
use commands::utils::help::*;
use commands::utils::ping::*;
use commands::utils::version::*;

use dotenv::dotenv;

use listeners::handler::Handler;

use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use std::collections::HashSet;
use std::env;

#[group]
#[description = "Various informational commands."]
#[commands(user)]
struct Information;

#[group]
#[description = "Ellie's selection of utility commands."]
#[commands(ping, version)]
struct Utilities;

#[group]
#[description = "Music-focused commands."]
#[commands(lastfm)]
struct Music;

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
            .group(&INFORMATION_GROUP)
            .group(&UTILITIES_GROUP)
            .group(&MUSIC_GROUP),
    );

    if let Err(err) = client.start() {
        println!("An error occurred while running the client: {:?}", err);
    }
}
