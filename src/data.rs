use aspotify::Client as SpotifyClient;
use reqwest::Client as ReqwestClient;
use serenity::{client::bridge::gateway::ShardManager, prelude::TypeMapKey};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::ConfigurationData;

pub struct ShardManagerContainer;
pub struct ConfigContainer;
pub struct DatabasePool;
pub struct ReqwestContainer;
pub struct SpotifyContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for ConfigContainer {
    type Value = ConfigurationData;
}

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

impl TypeMapKey for ReqwestContainer {
    type Value = ReqwestClient;
}

impl TypeMapKey for SpotifyContainer {
    type Value = SpotifyClient;
}
