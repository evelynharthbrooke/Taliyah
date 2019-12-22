use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[description("Checks the overall message latency.")]
#[usage("<blank>")]
fn ping(ctx: &mut Context, message: &Message) -> CommandResult {
    let mut msg = message.channel_id.send_message(&ctx.http, |m| {
        m.content(":ping_pong: Pinging!");
        m
    })?;

    let message_latency = msg.timestamp.timestamp_millis() - message.timestamp.timestamp_millis();
    let pong_message = "Pong! Succesfully checked the message latency. :ping_pong:\n\n";
    let msg_latency_msg = format!("**Message Latency**: `{}ms`\n", message_latency);
    let combined_message = format!("{}{}", pong_message, msg_latency_msg);

    msg.edit(&ctx, |msg| {
        msg.content("");
        msg.embed(|embed| {
            embed.color(0x008b_0000);
            embed.title("Discord Latency Information");
            embed.description(combined_message);
            embed
        });
        msg
    })?;

    Ok(())
}
