//! Ellie for Discord
//!
//! Ellie is a bot for the Discord chat platform focused on giving users
//! a powerful set of features, while remaining quick to respond.

mod commands;
mod listeners;
mod utilities;

use commands::extra::sloc::*;
use commands::extra::weather::*;
use commands::fun::ascii::*;
use commands::fun::urban::*;
use commands::info::changelog::*;
use commands::info::channel::*;
use commands::info::guild::*;
use commands::info::profile::*;
use commands::info::role::*;
use commands::info::user::*;
use commands::music::lastfm::*;
use commands::music::spotify::*;
use commands::music::voice::join::*;
use commands::music::voice::leave::*;
use commands::music::voice::play::*;
use commands::search::github::*;
use commands::search::krate::*;
use commands::search::reddit::*;
use commands::utilities::help::*;
use commands::utilities::invite::*;
use commands::utilities::ping::*;
use commands::utilities::prefix::*;
use commands::utilities::shutdown::*;
use commands::utilities::source::*;
use commands::utilities::version::*;

use dotenv::dotenv;

use listeners::handler::Handler;

use log::error;

use rspotify::spotify::client::Spotify;
use rspotify::spotify::oauth2::SpotifyClientCredentials;

use serenity::client::bridge::gateway::ShardManager;
use serenity::client::bridge::voice::ClientVoiceManager;
use serenity::client::Client;

use serenity::framework::standard::macros::group;
use serenity::framework::standard::DispatchError;
use serenity::framework::StandardFramework;

use serenity::prelude::Mutex;
use serenity::prelude::TypeMapKey;

use std::collections::HashSet;
use std::env;
use std::sync::Arc;

use utilities::database::create_database;
use utilities::database::get_prefix;

pub struct VoiceManager;

impl TypeMapKey for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[description = "Extra utility commands."]
#[commands(sloc, weather)]
struct Extra;

#[group]
#[description = "Fun commands."]
#[commands(ascii, urban, randefine)]
struct Fun;

#[group]
#[description = "Various informational commands."]
#[commands(changelog, channel, guild, profile, role, user)]
struct Information;

#[group]
#[description = "Music-focused commands."]
#[commands(lastfm, spotify)]
struct Music;

#[group]
#[description = "Ellie's voice command suite."]
#[commands(join, leave, play)]
struct Voice;

#[group]
#[description = "Ellie's selection of utility commands."]
#[commands(invite, ping, prefix, shutdown, source, version)]
struct Utilities;

#[group]
#[description = "Various commands related to searching for things."]
#[commands(github, krate, reddit)]
struct Search;

pub fn main() {
    dotenv().expect("Unable to read / access the .env file!");

    let token = env::var("DISCORD_TOKEN").expect("Unable to read the bot token.");

    let mut client = Client::new(&token, Handler).expect("Error creating client.");

    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
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
                            def_prefix.to_string();
                        }
                        if let Some(guild_id) = message.guild_id {
                            Some(get_prefix(guild_id).map_or_else(|_| def_prefix.to_string(), |prefix| prefix))
                        } else {
                            Some(def_prefix)
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
                DispatchError::TooManyArguments { max, given } => {
                    let _ = message.channel_id.send_message(&context, |error_message| {
                        error_message.embed(|embed| {
                            embed.title("Too many arguments provided.");
                            embed.description(format!(
                                "You provided too many arguments for this command. Minimum \
                                arguments were {} argument(s), you provided {} argument(s) \
                                instead. Please provide the right amount of arguments. For \
                                more information, please view the respective command's help \
                                documentation, if available.",
                                max, given
                            ))
                        })
                    });
                }
                DispatchError::NotEnoughArguments { min, given } => {
                    let _ = message.channel_id.send_message(&context, |error_message| {
                        error_message.embed(|embed| {
                            embed.title("Error: Not enough arguments provided.");
                            embed.description(format!(
                                "You didn't provide enough arguments for this command. Minimum \
                                arguments were {} argument(s), you provided {} argument(s) instead. \
                                Please provide the right amount of arguments. For more information, \
                                please view the respective command's help documentation, if \
                                available.",
                                min, given
                            ))
                        })
                    });
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
            .group(&EXTRA_GROUP)
            .group(&FUN_GROUP)
            .group(&INFORMATION_GROUP)
            .group(&MUSIC_GROUP)
            .group(&SEARCH_GROUP)
            .group(&VOICE_GROUP)
            .group(&UTILITIES_GROUP),
    );

    if let Err(err) = client.start_autosharded() {
        error!("An error occurred while running the client: {:?}", err);
    }
}

pub fn spotify() -> Spotify {
    let client_credential = SpotifyClientCredentials::default().build();
    Spotify::default().client_credentials_manager(client_credential).build()
}
