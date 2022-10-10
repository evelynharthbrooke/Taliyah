use chrono::{offset::Utc, Duration};

use serenity::{
    builder::{CreateEmbed, EditMessage},
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

use crate::ShardManagerContainer;

#[command]
#[description = "Checks Discord's API / message latency."]
async fn ping(context: &Context, message: &Message) -> CommandResult {
    let start = Utc::now();
    let start_ts = start.timestamp();
    let start_ts_ss = start.timestamp_subsec_millis() as i64;
    let mut ping: Message = message.channel_id.say(context, ":ping_pong: Pinging!").await?;
    let end = Utc::now();
    let end_ts = end.timestamp();
    let end_ts_ss = end.timestamp_subsec_millis() as i64;
    let api_response = ((end_ts - start_ts) * 1000) + (end_ts_ss - start_ts_ss);
    let ctx_data = context.data.read().await;
    let shard_manager = match ctx_data.get::<ShardManagerContainer>() {
        Some(shard) => shard,
        None => {
            message.reply(context, "I encountered a problem while getting the shard manager.").await?;
            return Ok(());
        }
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = match runners.get(&ShardId(context.shard_id)) {
        Some(runner) => runner,
        None => {
            message.reply(context, "Could not find a shard").await?;
            return Ok(());
        }
    };

    let shard_response = match runner.latency {
        Some(latency) => match Duration::from_std(latency) {
            Ok(time) => format!("`{}ms`", time.num_milliseconds()),
            Err(_) => "No latency information available".to_string()
        },
        None => "No data available at the moment.".to_string()
    };

    let response = format!(
        "Pong! Succesfully retrieved the message and shard latencies. :ping_pong:\n\n\
        **API Response Time**: `{}ms`\n\
        **Shard Response Time**: {}",
        api_response, shard_response
    );

    let embed = CreateEmbed::new().color(0x008b_0000).title("Discord Latency Information").description(response);
    ping.edit(context, EditMessage::new().embed(embed)).await?;

    Ok(())
}
