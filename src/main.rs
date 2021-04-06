//! Ellie for Discord
//!
//! Ellie is a bot for the Discord chat platform focused on giving users
//! a powerful set of features, while remaining quick to respond.

mod commands;
mod config;
mod constants;
mod data;
mod error;
mod listeners;
mod models;
mod utils;

use commands::{
    extra::{sloc::*, steamstatus::*},
    fun::{ascii::*, printerfacts::*, urban::*, xkcd::*},
    info::{about::*, channel::*, first_message::*, guild::*, profile::*, role::*, user::*},
    moderation::slowmode::*,
    music::{lastfm::*, spotify::*},
    search::{krate::*, tmdb::*},
    social::twitter::*,
    utilities::{help::*, invite::*, owner::bnick::*, ping::*, source::*},
};

use listeners::{
    handler::Handler,
    hooks::*
};

use reqwest::{redirect::Policy, Client};
use serenity::{
    client::{bridge::gateway::GatewayIntents, validate_token, ClientBuilder},
    framework::{standard::macros::group, StandardFramework},
    http::Http
};

use sqlx::postgres::PgPoolOptions;

use std::{collections::HashSet, error::Error, sync::Arc};

use tracing::{error, info, instrument, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::{constants::*, data::*, utils::read_config};

#[group("Extra")]
#[description = "Commands that don't really fit in the other command groups."]
#[commands(sloc, steamstatus)]
struct Extra;

#[group("Fun")]
#[description = "Commands that could be considered fun / silly."]
#[commands(ascii, printerfacts, urban, randefine, xkcd)]
struct Fun;

#[group("Info")]
#[description = "Informational commands that provide useful information."]
#[commands(about, channel, first_message, guild, profile, role, user)]
struct Info;

#[group("Moderation")]
#[description = "Commands that help with moderation of servers."]
#[commands(slowmode)]
struct Moderation;

#[group("Music")]
#[description = "Music-focused commands."]
#[commands(lastfm, spotify)]
struct Music;

#[group("Owner")]
#[description = "Commands restricted to the bot owner."]
#[commands(bnick)]
#[owners_only]
struct Owner;

#[group("Search")]
#[description = "Various commands that search various web services."]
#[commands(krate, tmdb)]
struct Search;

#[group("Social")]
#[description = "Commands that integrate with various services, e.g. Twitter."]
#[commands(twitter)]
struct Social;

#[group("Utilities")]
#[description = "Miscellaneous commands that don't really fit into a more-specific category."]
#[commands(invite, ping, source)]
struct Utilities;

#[tokio::main(worker_threads = 16)]
#[instrument]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let configuration = read_config("config.toml");
    let logging = configuration.bot.logging.enabled;

    if logging {
        LogTracer::init()?;

        let base_level = configuration.bot.logging.level.as_str();

        let level = match base_level {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::TRACE
        };

        let subscriber = FmtSubscriber::builder()
            .with_target(false)
            .with_max_level(level)
            .with_env_filter(EnvFilter::from_default_env())
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;

        info!("Tracing initialized; level {}.", level);
    }

    let token = configuration.bot.discord.token;
    let prefix = configuration.bot.general.prefix.as_str();

    match validate_token(&token) {
        Ok(_) => info!("Token successfully validated. Continuing."),
        Err(_) => {
            error!("Token was not successfully validated. Cannot continue.");
            return Ok(());
        }
    }

    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => {
            error!("Unable to retrieve application info: {:?}", why);
            return Ok(());
        }
    };

    let framework = StandardFramework::new()
        .configure(|configuration| {
            configuration
                .on_mention(Some(bot_id))
                .prefix(prefix)
                .ignore_webhooks(false)
                .ignore_bots(true)
                .no_dm_prefix(true)
                .with_whitespace(true)
                .owners(owners)
                .case_insensitivity(true)
        })
        .after(after)
        .prefix_only(prefix_only)
        .on_dispatch_error(dispatch_error)
        .group(&EXTRA_GROUP)
        .group(&FUN_GROUP)
        .group(&INFO_GROUP)
        .group(&MODERATION_GROUP)
        .group(&MUSIC_GROUP)
        .group(&OWNER_GROUP)
        .group(&SEARCH_GROUP)
        .group(&SOCIAL_GROUP)
        .group(&UTILITIES_GROUP)
        .help(&HELP);

    let mut client = ClientBuilder::new(&token)
        .event_handler(Handler)
        .intents(GatewayIntents::all())
        .framework(framework)
        .await?;

    {
        let mut data = client.data.write().await;

        let url = configuration.bot.database.url;
        let pool = PgPoolOptions::new().max_connections(20).connect(&url).await?;
        let http_client = Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?;

        data.insert::<DatabasePool>(pool);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ReqwestContainer>(http_client);

        {
            let id = configuration.api.music.spotify.client_id;
            let secret = configuration.api.music.spotify.client_secret;
            let credentials = aspotify::ClientCredentials { id, secret };
            let spotify_client = aspotify::Client::new(credentials);
            data.insert::<SpotifyContainer>(spotify_client);
        }
    }

    if let Err(why) = client.start_autosharded().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}
