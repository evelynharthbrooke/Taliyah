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
    extra::sloc::*,
    fun::{ascii::*, printerfacts::*, urban::*, xkcd::*},
    info::{about::*, channel::*, first_message::*, guild::*, profile::*, role::*, user::*},
    moderation::{ban::*, kick::*, slowmode::*},
    music::{lastfm::*, spotify::*},
    search::tmdb::*,
    social::twitter::*,
    utilities::{help::*, invite::*, owner::bnick::*, ping::*, source::*}
};

use listeners::{handler::Handler, hooks::*};

use reqwest::{redirect::Policy, Client};
use serenity::{
    client::ClientBuilder,
    framework::{standard::macros::group, StandardFramework},
    model::gateway::GatewayIntents,
    http::Http
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
#[commands(ascii, printerfacts, urban, randefine, xkcd)]
struct Fun;

#[group("Info")]
#[description = "Informational commands that provide useful information."]
#[commands(about, channel, first_message, guild, profile, role, user)]
struct Info;

#[group("Moderation")]
#[description = "Commands that help with the moderation of servers."]
#[commands(ban, kick, slowmode)]
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

        info!("Tracing initialized with logging level set to {}.", level);
    }

    let appid = configuration.bot.discord.appid;
    let token = configuration.bot.discord.token;
    let prefix = configuration.bot.general.prefix.as_str();

    let http = Http::new_with_token(&token);
    let id = http.get_current_user().await.unwrap().id;
    let owner = http.get_current_application_info().await.unwrap().owner.id;

    let mut owners = HashSet::new();
    owners.insert(owner);

    let framework = StandardFramework::new()
        .configure(|configuration| {
            configuration
                .on_mention(Some(id))
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
        .application_id(appid)
        .intents(GatewayIntents::all())
        .framework(framework)
        .await?;

    {
        let mut data = client.data.write().await;

        let url = configuration.bot.database.url;
        let pool = PgPoolOptions::new().max_connections(20).connect(&url).await?;
        let http_client = Client::builder().user_agent(REQWEST_USER_AGENT).redirect(Policy::none()).build()?;

        data.insert::<ConfigContainer>(read_config("config.toml"));
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
