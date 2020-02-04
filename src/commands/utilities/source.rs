use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[description = "Sends a link to the bot's source code."]
pub fn source(context: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&context, |message| {
        let source = "http://github.com/KamranMackey/Ellie";
        message.content(format!("Here is the link to my source: <{}>", source))
    })?;

    Ok(())
}
