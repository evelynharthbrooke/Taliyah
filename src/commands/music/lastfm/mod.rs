pub mod profile;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

use self::profile::*;

/// Shows a bunch of information from Last.fm.
#[command]
#[aliases("lfm", "fm", "last")]
#[sub_commands(profile)]
async fn lastfm(context: &Context, message: &Message) -> CommandResult {
    message.channel_id.say(context, "No valid subcommand provided. Do `help lastfm` to see the commands.").await?;
    Ok(())
}
