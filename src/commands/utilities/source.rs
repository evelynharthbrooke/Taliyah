use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

#[command]
#[description = "Sends a link containing the bot's source code."]
async fn source(context: &Context, message: &Message) -> CommandResult {
    message.reply(context, "GitHub repository: <https://github.com/evelynmarie/Ellie>").await?;
    Ok(())
}
