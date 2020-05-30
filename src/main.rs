//! Ellie for Discord
//!
//! Ellie is a bot for the Discord chat platform focused on giving users
//! a powerful set of features, while remaining quick to respond.

mod commands;
mod listeners;
mod utils;

use commands::utilities::help::*;
use commands::utilities::invite::*;
use commands::utilities::ping::*;
use commands::utilities::source::*;

use listeners::{handler::Handler, hooks::prefix_only::*};

use serenity::{
    client::{
        bridge::gateway::{GatewayIntents, ShardManager},
        Client
    },
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    prelude::{Mutex, TypeMapKey}
};

use std::{collections::HashSet, error::Error, fs::File, io::prelude::*, sync::Arc};

use toml::Value;

use tracing::{info, instrument, Level};
use tracing_log::LogTracer;
use tracing_subscriber::FmtSubscriber;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[group("Utilities")]
#[description = "Miscellaneous commands that don't really fit into a more-specific category."]
#[commands(invite, ping, source)]
struct Utilities;

#[tokio::main]
#[instrument]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("config.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let configuration = contents.parse::<Value>().unwrap();
    let logging = configuration["logging"]["enabled"].as_bool().unwrap();

    if logging {
        LogTracer::init()?;

        let base_level = configuration["logging"]["level"].as_str().unwrap();

        let level = match base_level {
            "error" => Level::ERROR,
            "warn" => Level::WARN,
            "info" => Level::INFO,
            "debug" => Level::DEBUG,
            "trace" => Level::TRACE,
            _ => Level::TRACE
        };

        let subscriber = FmtSubscriber::builder().with_max_level(level).finish();
        tracing::subscriber::set_global_default(subscriber)?;

        info!("Logger initialized.");
    }

    let token = configuration["discord"]["token"].as_str().unwrap();
    let prefix = configuration["discord"]["prefix"].as_str().unwrap();
    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);
            (owners, info.id)
        }
        Err(why) => panic!("Unable to retrieve application info: {:?}", why)
    };

    let framework = StandardFramework::new()
        .configure(|config| {
            config
                .on_mention(Some(bot_id))
                .prefix(prefix)
                .ignore_webhooks(false)
                .ignore_bots(true)
                .with_whitespace(true)
                .owners(owners)
                .case_insensitivity(true)
        })
        .prefix_only(prefix_only)
        .group(&UTILITIES_GROUP)
        .help(&HELP);

    let mut client = Client::new(&token)
        .event_handler(Handler)
        .framework(framework)
        .add_intent({
            let mut intents = GatewayIntents::all();
            intents.remove(GatewayIntents::DIRECT_MESSAGE_TYPING);
            intents
        })
        .await?;

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start_autosharded().await {
        eprintln!("An error ocurred while running the client: {:?}", why);
    }

    Ok(())
}
