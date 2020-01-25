use crate::listeners::events::message;
use crate::listeners::events::ready;

use serenity::client::{Context, EventHandler};
use serenity::model::gateway::Ready;
use serenity::model::prelude::Message;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, context: Context, ready: Ready) {
        ready::ready(context, ready);
    }
    fn message(&self, context: Context, message: Message) {
        message::message(context, message);
    }
}
