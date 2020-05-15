use crate::listeners::events::ready;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::gateway::Ready
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        ready::ready(context, ready).await
    }
}
