use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message, Permissions}
};

use tracing::error;

#[command]
#[description = "Generates an invite link for the bot."]
async fn invite(context: &Context, message: &Message) -> CommandResult {
    let cache = &context.cache;
    let permissions = Permissions::empty();
    let current_user = &cache.current_user();
    let url = match current_user.invite_url(context, permissions).await {
        Ok(invite) => invite,
        Err(why) => {
            error!("Encountered an error while trying to generate an invite: {}", why);
            message.reply(context, "Couldn't generate invite.").await?;
            return Ok(());
        }
    };

    let user = &current_user;
    let name = &user.name;
    let avatar = user.avatar_url().unwrap();

    message
        .channel_id
        .send_message(context, |message| {
            message.embed(|embed| {
                embed.title(format!("{} Invite URL", name));
                embed.thumbnail(avatar);
                embed.description(format!("Click [here]({}) to add {} to your Discord server.", url, name))
            })
        })
        .await?;

    Ok(())
}
