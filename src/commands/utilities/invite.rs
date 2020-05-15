use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message, Permissions}
};

use tracing::error;

#[command]
#[description = "Generates an invite link for the bot."]
pub async fn invite(context: &Context, message: &Message) -> CommandResult {
    let cache = context.cache.read().await;
    let invite_url = match cache.user.invite_url(context, Permissions::empty()).await {
        Ok(inv) => inv,
        Err(why) => {
            error!("Encountered an error while trying to generate an invite: {}", why);
            message.channel_id.send_message(context, |m| m.content("Couldn't generate invite.")).await?;
            return Ok(());
        }
    };

    message
        .channel_id
        .send_message(context, |message| {
            message.embed(|embed| {
                let user = &cache.user.name;
                embed.title(format!("{} Invite URL", user));
                embed.thumbnail(cache.user.avatar_url().unwrap());
                embed.description(format!("Click [here]({}) to add {} to your Discord server.", invite_url, user))
            })
        })
        .await?;

    Ok(())
}
