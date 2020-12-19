pub mod user;

use self::user::*;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

/// Gets a variety of information from the API provided by The Movie Database.
#[command]
#[sub_commands(user)]
#[aliases("tw")]
async fn twitter(context: &Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title("Error: Invalid / No Subcommand Entered!");
            embed.description("No valid subcommand entered. Please check the help, or try again.")
        })
    }).await?;

    Ok(())
}
