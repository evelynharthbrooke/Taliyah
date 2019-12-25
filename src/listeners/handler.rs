use crate::listeners::events::ready;

use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready);
    }
}