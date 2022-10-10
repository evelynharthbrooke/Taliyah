use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage, GetMessages},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::utils::parsing_utils::parse_channel;

/// Retrieves the first message ever sent to a channel.
#[command]
#[required_permissions(READ_MESSAGE_HISTORY)]
#[aliases("first-message", "first-msg")]
#[only_in(guilds)]
async fn first_message(context: &Context, message: &Message, args: Args) -> CommandResult {
    let guild_id = message.guild_id.unwrap();

    let channel_name = if args.is_empty() {
        message.channel_id.name(&context).await.unwrap()
    } else {
        args.rest().to_string()
    };

    let channel_id = match parse_channel(&channel_name, guild_id, context).await {
        Some(channel_id) => channel_id,
        None => {
            message.channel_id.say(context, "This channel does not exist.").await?;
            return Ok(());
        }
    };

    let messages = channel_id.messages(&context, GetMessages::new().after(1).limit(1)).await.unwrap();
    let msg = messages.first().unwrap();
    let msg_link = msg.link().replace("@me", &guild_id.to_string()).to_string();
    let msg_guild = message.guild(&context.cache).unwrap().clone();
    let msg_member = msg_guild.member(context, msg.author.id).await.unwrap();
    let msg_author_color = msg_member.colour(context).unwrap();

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(msg.author.tag()).icon_url(msg.author.avatar_url().unwrap()))
        .color(msg_author_color)
        .thumbnail(msg.author.avatar_url().unwrap())
        .description(&msg.content)
        .timestamp(&msg.timestamp)
        .field("❯ Jump To Message", format!("[Click Here]({})", msg_link), true)
        .footer(CreateEmbedFooter::new(format!("Message ID: {}", msg.id)));

    message.channel_id.send_message(context, CreateMessage::new().embed(embed)).await?;

    Ok(())
}
