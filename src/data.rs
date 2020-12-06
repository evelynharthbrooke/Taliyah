use lavalink_rs::LavalinkClient;
use serenity::{client::bridge::gateway::ShardManager, prelude::TypeMapKey};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ShardManagerContainer;
pub struct DatabasePool;
pub struct Lavalink;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

impl TypeMapKey for DatabasePool {
    type Value = PgPool;
}

impl TypeMapKey for Lavalink {
    type Value = Arc<Mutex<LavalinkClient>>;
}
