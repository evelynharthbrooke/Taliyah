use crate::listeners::events::{guild_create, message, ready};
use lavalink_rs::gateway::LavalinkEventHandler;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::channel::Message,
    model::{gateway::Ready, guild::Guild}
};

pub struct Handler;
pub struct LavaHandler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        ready::ready(context, ready).await
    }

    async fn guild_create(&self, context: Context, guild: Guild, is_new: bool) {
        guild_create::guild_create(context, guild, is_new).await
    }

    async fn message(&self, context: Context, message: Message) {
        message::message(context, message).await
    }
}

#[async_trait]
impl LavalinkEventHandler for LavaHandler {}
