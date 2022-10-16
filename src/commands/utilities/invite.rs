use serenity::{
    builder::{CreateEmbed, CreateMessage},
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
    let current_user = cache.current_user().clone();
    let url = match current_user.invite_url(&context.http, permissions).await {
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

    let embed = CreateEmbed::new()
        .title(format!("{name} Invite URL"))
        .thumbnail(avatar)
        .description(format!("Click [here]({url}) to add {name} to your Discord server."));

    message.channel_id.send_message(&context.http, CreateMessage::new().embed(embed)).await?;

    Ok(())
}
