use crate::ShardManagerContainer;

use serenity::client::bridge::gateway::ShardId;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use chrono::Duration;

#[command]
#[description("Checks the overall message latency.")]
#[usage("<blank>")]
fn ping(context: &mut Context, message: &Message) -> CommandResult {
    let initial_timestamp = message.timestamp.timestamp_millis();
    let mut msg = message.channel_id.send_message(&context, |message| message.content(":ping_pong: Pinging!"))?;

    let data = context.data.read();
    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(shard) => shard,
        None => {
            message.reply(&context, "There was a problem getting the shard manager.")?;
            return Ok(());
        }
    };

    let manager = shard_manager.try_lock().ok_or("Couldn't get a lock on the manager")?;
    let runners = manager.runners.try_lock().ok_or("Couldn't get a lock on the current shard runner.")?;

    let runner = match runners.get(&ShardId(context.shard_id)) {
        Some(runner) => runner,
        None => {
            message.reply(&context, "Couldn't find any shards...")?;
            return Ok(());
        }
    };

    let latency = match runner.latency {
        Some(latency) => match Duration::from_std(latency) {
            Ok(milli) => format!("`{}ms`", milli.num_milliseconds()),
            Err(_) => "Could not get latency information... :(".to_string(),
        },
        None => "No data available yet.".to_string(),
    };

    let message_latency = msg.timestamp.timestamp_millis() - initial_timestamp;

    let response = format!(
        "Pong! Succesfully retrieved the message and shard latencies. :ping_pong:\n\n\
        **Message Latency**: `{}ms`\n\
        **Shard Latency**: {}",
        message_latency, latency
    );

    msg.edit(&context, |message| {
        message.content("");
        message.embed(|embed| {
            embed.color(0x008b_0000);
            embed.title("Discord Latency Information");
            embed.description(response)
        })
    })?;

    return Ok(())
}
