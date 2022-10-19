use crate::data::ShardManagerContainer;
use chrono::{Duration, Utc};
use serenity::{
    builder::{CreateEmbed, CreateMessage, EditMessage},
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{
        help_commands,
        macros::{command, help},
        Args, CommandGroup, CommandResult, HelpOptions
    },
    model::{
        prelude::{Message, UserId},
        Permissions
    }
};

use std::collections::HashSet;
use tracing::log::error;

#[help]
#[max_levenshtein_distance(3)]
#[no_help_available_text("No help information available.")]
async fn help(ctx: &Context, msg: &Message, args: Args, opts: &'static HelpOptions, groups: &[&'static CommandGroup], owners: HashSet<UserId>) -> CommandResult {
    drop(help_commands::plain(ctx, msg, args, opts, groups, owners).await);
    Ok(())
}

#[command]
#[description = "Generates an invite link for the bot."]
async fn invite(context: &Context, message: &Message) -> CommandResult {
    let current_user = &context.cache.current_user().clone();
    let url = match current_user.invite_url(&context.http, Permissions::empty()).await {
        Ok(invite) => invite,
        Err(why) => {
            error!("Encountered an error while trying to generate an invite: {}", why);
            message.reply(context, "Couldn't generate invite.").await?;
            return Ok(());
        }
    };

    let user = current_user;
    let name = &user.name;
    let avatar = user.avatar_url().unwrap();
    let embed = CreateEmbed::new()
        .title(format!("{name} Invite URL"))
        .thumbnail(avatar)
        .description(format!("Click [here]({url}) to add {name} to your Discord server."));

    message.channel_id.send_message(&context.http, CreateMessage::new().embed(embed)).await?;

    Ok(())
}

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
        Some(latency) => {
            if let Ok(time) = Duration::from_std(latency) {
                let time_ms = time.num_milliseconds();
                format!("`{time_ms}ms`")
            } else {
                "No latency information available".to_string()
            }
        }
        None => "No data available at the moment.".to_string()
    };

    let response = format!(
        "Pong! Succesfully retrieved the message and shard latencies. :ping_pong:\n\n\
        **API Response Time**: `{api_response}ms`\n\
        **Shard Response Time**: {shard_response}"
    );

    let embed = CreateEmbed::new().color(0x008b_0000).title("Discord Latency Information").description(response);
    ping.edit(context, EditMessage::new().embed(embed)).await?;

    Ok(())
}

#[command]
#[description = "Sends a link containing the bot's source code."]
async fn source(context: &Context, message: &Message) -> CommandResult {
    message.reply(context, "GitHub repository: <https://github.com/evelynmarie/Ellie>").await?;
    Ok(())
}
