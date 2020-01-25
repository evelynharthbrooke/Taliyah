use crate::listeners::events::messagecreate;
use crate::listeners::events::ready;

use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, ready: Ready) {
        ready::ready(ctx, ready);
    }
    fn message(&self, ctx: Context, new_message: Message) {
        messagecreate::message(ctx, new_message);
    }
}
