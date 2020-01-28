use crate::ShardManagerContainer;

use log::{error, info};

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[owners_only]
#[description = "Gracefully shutdowns the bot."]
pub fn shutdown(context: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&context, |message| message.content("Shutting down..."))?;

    let data = context.data.write();

    let manager = match data.get::<ShardManagerContainer>() {
        Some(shard) => shard,
        None => {
            error!("Unable to get the shard manager...killing the bot ungracefully.");
            std::process::exit(0);
        }
    };

    if let Some(mut shards) = manager.try_lock() {
        info!("Shutting down all shards...");
        shards.shutdown_all();
    } else {
        error!("Unable to get shard manager...killing the bot ungracefully.");
        std::process::exit(0);
    }

    return Ok(());
}
