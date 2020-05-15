use serenity::{
    client::Context,
    framework::standard::{
        macros::command,
        CommandResult,
    },
    model::prelude::Message,
};

#[command]
#[description = "Sends a link containing the bot's source code."]
pub async fn source(context: &Context, message: &Message) -> CommandResult {

    message.channel_id.send_message(context, |message| {
        let source = "https://github.com/KamranMackey/Ellie";
        message.content(format!("Source code: <{}>", source))
    }).await?;

    Ok(())
}
