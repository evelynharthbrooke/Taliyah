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

use aspotify::{Client as SpotifyClient, ClientCredentials};
use commands::{
    extra::sloc::*,
    fun::{urban::*, xkcd::*},
    info::{about::*, guild::*, profile::*, user::*},
    moderation::{ban::*, kick::*, slowmode::*},
    music::{lastfm::*, spotify::*},
    search::tmdb::*,
    social::twitter::*,
    utilities::*
};

use listeners::{handler::Handler, hooks::*};

use reqwest::{redirect::Policy, Client};
use serenity::{
    client::ClientBuilder,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::gateway::GatewayIntents
};

use sqlx::postgres::PgPoolOptions;

use std::{collections::HashSet, error::Error, sync::Arc};

use tracing::{info, instrument, Level};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use crate::{constants::*, data::*, utils::read_config};

#[group("Extra")]
#[description = "Commands that don't really fit in the other command groups."]
#[commands(sloc)]
struct Extra;

#[group("Fun")]
#[description = "Commands that could be considered fun / silly."]
#[commands(urban, randefine, xkcd)]
struct Fun;

#[group("Info")]
#[description = "Informational commands that provide useful information."]
#[commands(about, guild, profile, user)]
struct Info;

#[group("Moderation")]
#[description = "Commands that help with the moderation of servers."]
#[commands(ban, kick, slowmode)]
struct Moderation;

#[group("Music")]
#[description = "Music-focused commands."]
#[commands(lastfm, spotify)]
struct Music;

#[group("Search")]
#[description = "Various commands that search various web services."]
#[commands(tmdb)]
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
    if configuration.bot.logging.enabled {
        LogTracer::init()?;

        let level = match configuration.bot.logging.level.as_str() {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            _ => Level::TRACE
        };

        let subscriber = FmtSubscriber::builder()
            .with_target(false)
            .with_max_level(level)
            .with_env_filter(EnvFilter::from_default_env())
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;

        info!("Tracing initialized with logging level set to {}.", level);
    }

    let token = configuration.bot.discord.token;
    let prefix = configuration.bot.general.prefix.as_str();

    let http = Http::new(&token);
    let id = http.get_current_user().await?.id;
    let owner = http.get_current_application_info().await?.owner.id;

    let mut owners = HashSet::new();
    owners.insert(owner);

    let framework = StandardFramework::new()
        .prefix_only(prefix_only)
        .after(after)
        .on_dispatch_error(dispatch_error)
        .group(&EXTRA_GROUP)
        .group(&FUN_GROUP)
        .group(&INFO_GROUP)
        .group(&MODERATION_GROUP)
        .group(&MUSIC_GROUP)
        .group(&SEARCH_GROUP)
        .group(&SOCIAL_GROUP)
        .group(&UTILITIES_GROUP)
        .help(&HELP);

    framework.configure(|c| c.on_mention(Some(id)).prefix(prefix).ignore_webhooks(false).no_dm_prefix(true).owners(owners));

    let mut client = ClientBuilder::new(&token, GatewayIntents::all()).event_handler(Handler).framework(framework).await?;

    {
        let mut data = client.data.write().await;
        let url = configuration.bot.database.url;
        let pool = PgPoolOptions::new().max_connections(20).connect(&url).await?;
        let http = Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?;

        data.insert::<ConfigContainer>(read_config("config.toml"));
        data.insert::<DatabasePool>(pool);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        data.insert::<ReqwestContainer>(http);

        {
            let id = configuration.api.music.spotify.client_id;
            let secret = configuration.api.music.spotify.client_secret;
            let client = SpotifyClient::new(ClientCredentials { id, secret });
            data.insert::<SpotifyContainer>(client);
        }
    }

    if let Err(why) = client.start_autosharded().await {
        eprintln!("An error occurred while running the client: {why:?}");
    }

    Ok(())
}
