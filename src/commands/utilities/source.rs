use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

#[command]
#[description = "Sends a link containing the bot's source code."]
pub async fn source(context: &Context, message: &Message) -> CommandResult {
    message.reply(context, "GitHub repository: <https://github.com/KamranMackey/Ellie>").await?;
    Ok(())
}
