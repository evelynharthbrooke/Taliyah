//! Ellie for Discord
//!
//! Ellie is a bot for the Discord chat platform focused on giving users
//! a powerful set of features, while remaining quick to respond.

mod utilities;
mod commands;
mod listeners;

use commands::info::guild::*;
use commands::info::user::*;
use commands::music::lastfm::*;
use commands::music::spotify::commands::spotify::*;
use commands::utils::help::*;
use commands::utils::prefix::*;
use commands::utils::ping::*;
use commands::utils::version::*;

use dotenv::dotenv;

use log::error;

use listeners::handler::Handler;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;

use serenity::client::Client;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::framework::standard::DispatchError;

use std::collections::HashSet;
use std::env;

use utilities::database::create_database;
use utilities::database::get_prefix;

#[group]
#[description = "Various informational commands."]
#[commands(user, guild)]
struct Information;

#[group]
#[description = "Ellie's selection of utility commands."]
#[commands(ping, prefix, version)]
struct Utilities;

#[group]
#[description = "Music-focused commands."]
#[commands(lastfm, spotify)]
struct Music;

pub fn main() {
    dotenv().expect("Unable to read / access the .env file!");

    create_database();

    let token: String = env::var("DISCORD_TOKEN").expect("Unable to read the bot token.");

    let mut client: Client = Client::new(&token, Handler).expect("Error creating client.");

    pretty_env_logger::init();

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
            .configure(|c| {
                c.with_whitespace(true)
                    .ignore_webhooks(false)
                    .case_insensitivity(true)
                    .owners(owners)
                    .dynamic_prefix(|_, message| {
                        let def_prefix: String = env::var("DISCORD_PREFIX").expect("Unable to get the bot prefix.");
                        if message.is_private() {
                            return Some(def_prefix.clone().to_string());
                        }
                        if let Some(guild_id) = message.guild_id {
                            let prefix = get_prefix(&guild_id).map_or_else(|_| def_prefix.clone().to_string(), |prefix| prefix);
                            return Some(prefix);
                        } else {
                            return Some(def_prefix.to_string())
                        }
                    })
                    .on_mention(Some(bot_id))
            })
            .on_dispatch_error(|ctx, msg, err| match err {
                DispatchError::Ratelimited(secs) => {
                    let _ = msg.channel_id.say(&ctx.http, &format!("Try this again in {} seconds", secs));
                },
                DispatchError::OnlyForOwners => {
                    let _ = msg.channel_id.say(&ctx.http, "This is only available for owners.");
                },
                DispatchError::IgnoredBot => {},
                _ => error!("Dispatch Error: {} failed: {:?}", msg.content, err),
            })
            .after(|ctx, msg, cmd_name, err| if let Err(why) = err {
                let _ = msg.channel_id.say(&ctx.http, "An error occured while running this command, please try again later.");
                error!("Command Error: {} triggered by {} has errored: {:#?}", cmd_name, msg.author.tag(), why);
            })
            .help(&HELP)
            .group(&INFORMATION_GROUP)
            .group(&UTILITIES_GROUP)
            .group(&MUSIC_GROUP),
    );

    if let Err(err) = client.start() {
        error!("An error occurred while running the client: {:?}", err);
    }
}

pub fn spotify() -> Spotify {
    let client_credential = SpotifyClientCredentials::default().build();
    return Spotify::default().client_credentials_manager(client_credential).build();
}
