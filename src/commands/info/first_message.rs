use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

/// Retrieves the first message ever sent to a channel.
#[command]
#[required_permissions(READ_MESSAGE_HISTORY)]
#[aliases("first-message", "first-msg")]
#[only_in(guilds)]
pub async fn first_message(context: &Context, message: &Message) -> CommandResult {
    let guild_id = message.guild_id;

    if guild_id.is_none() {
        message.reply(&context, "You are not in a guild channel!").await?;
        return Ok(());
    }

    let messages = message.channel_id.messages(&context, |retriever| retriever.after(1).limit(1)).await.unwrap();
    let msg = messages.first().unwrap();
    let msg_link = msg.link().replace("@me", &guild_id.unwrap().to_string()).to_string();
    let msg_author_color = message.guild(context).await.unwrap().member(context, msg.author.id).await.unwrap().colour(context).await.unwrap();

    message
        .channel_id
        .send_message(context, |message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.name(msg.author.tag());
                    author.icon_url(msg.author.avatar_url().unwrap())
                });
                embed.color(msg_author_color);
                embed.thumbnail(msg.author.avatar_url().unwrap());
                embed.description(&msg.content);
                embed.timestamp(&msg.timestamp);
                embed.field("❯ Jump To Message", format!("[Click Here]({})", msg_link), true);
                embed.footer(|footer| footer.text(format!("Message ID: {}", msg.id)));
                embed
            })
        })
        .await?;

    Ok(())
}
