use log::error;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::model::Permissions;

#[command]
#[owners_only]
#[description = "Generates an invite link for the bot."]
pub fn invite(context: &mut Context, message: &Message) -> CommandResult {
    let cache = &context.cache.read();
    
    let invite_url = match cache.user.invite_url(&context, Permissions::empty()) {
        Ok(invite) => invite,
        Err(e) => {
            error!("Encountered an error while attempting to generate an invite URL: {}", e);
            message.channel_id.say(&context, "Could not retrieve invite URL. :(")?;
            return Ok(());
        }
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            let user = &cache.user.name;
            embed.title(format!("{} Invite URL", user));
            embed.thumbnail(cache.user.avatar_url().unwrap());
            embed.description(format!("Click [here]({}) to add {} to your Discord server.", invite_url, user))
        })
    })?;

    return Ok(());
}
