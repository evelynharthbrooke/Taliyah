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
    message.channel_id.say(context, "No valid subcommand entered. Do `help twitter` to see the commands.").await?;
    Ok(())
}
