//! Ellie for Discord
//!
//! Ellie is a bot for the Discord chat platform focused on giving users
//! a powerful set of features, while remaining quick to respond.

mod commands;
mod listeners;
mod utilities;

use commands::fun::urban::*;
use commands::extra::weather::*;
use commands::info::guild::*;
use commands::info::profile::*;
use commands::info::user::*;
use commands::music::lastfm::*;
use commands::music::spotify::commands::spotify::*;
use commands::search::krate::*;
use commands::utilities::help::*;
use commands::utilities::invite::*;
use commands::utilities::ping::*;
use commands::utilities::prefix::*;
use commands::utilities::shutdown::*;
use commands::utilities::version::*;

use dotenv::dotenv;

use log::error;

use listeners::handler::Handler;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;

use serenity::client::Client;
use serenity::client::bridge::gateway::ShardManager;

use serenity::framework::standard::macros::group;
use serenity::framework::standard::DispatchError;
use serenity::framework::StandardFramework;

use serenity::prelude::Mutex as ShardMutex;

use std::collections::HashSet;
use std::sync::Arc;
use std::env;

use typemap::Key;

use utilities::database::create_database;
use utilities::database::get_prefix;

pub struct ShardManagerContainer;

impl Key for ShardManagerContainer {
    type Value = Arc<ShardMutex<ShardManager>>;
}

#[group]
#[description = "Fun commands."]
#[commands(urban, randefine)]
struct Fun;

#[group]
#[description = "Extra utility commands."]
#[commands(weather)]
struct Extra;

#[group]
#[description = "Various informational commands."]
#[commands(user, guild, profile)]
struct Information;

#[group]
#[description = "Ellie's selection of utility commands."]
#[commands(invite, ping, prefix, shutdown, version)]
struct Utilities;

#[group]
#[description = "Music-focused commands."]
#[commands(lastfm, spotify)]
struct Music;

#[group]
#[description = "Various commands related to searching for things."]
#[commands(krate)]
struct Search;

pub fn main() {
    dotenv().expect("Unable to read / access the .env file!");

    let token = env::var("DISCORD_TOKEN").expect("Unable to read the bot token.").to_string();

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }
    
    pretty_env_logger::init();
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Couldn't get app info: {:?}", why),
    };

    create_database();

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
                            return Some(def_prefix.to_string());
                        }
                        if let Some(guild_id) = message.guild_id {
                            let prefix = get_prefix(&guild_id).map_or_else(|_| def_prefix.to_string(), |prefix| prefix);
                            return Some(prefix);
                        } else {
                            return Some(def_prefix.to_string());
                        }
                    })
                    .on_mention(Some(bot_id))
            })
            .on_dispatch_error(|context, message, err| match err {
                DispatchError::Ratelimited(secs) => {
                    let _ = message.channel_id.say(&context, &format!("Try this again in {} seconds", secs));
                }
                DispatchError::OnlyForOwners => {
                    let _ = message.channel_id.say(&context, "This is only available for owners.");
                }
                DispatchError::IgnoredBot => {}
                _ => error!("Dispatch Error: {} failed: {:?}", message.content, err),
            })
            .after(|context, message, command_name, err| {
                if let Err(why) = err {
                    let _ = message.channel_id.say(&context, "An error occured while running this command, please try again later.");
                    error!("Command Error: {} triggered by {} has errored: {:#?}", command_name, message.author.tag(), why);
                }
            })
            .help(&HELP)
            .group(&FUN_GROUP)
            .group(&EXTRA_GROUP)
            .group(&INFORMATION_GROUP)
            .group(&UTILITIES_GROUP)
            .group(&MUSIC_GROUP)
            .group(&SEARCH_GROUP),
    );

    if let Err(err) = client.start_autosharded() {
        error!("An error occurred while running the client: {:?}", err);
    }
}

pub fn spotify() -> Spotify {
    let client_credential = SpotifyClientCredentials::default().build();
    return Spotify::default().client_credentials_manager(client_credential).build();
}
