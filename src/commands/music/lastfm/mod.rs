pub mod nowplaying;
pub mod profile;
pub mod scrobbles;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

use self::nowplaying::*;
use self::profile::*;
use self::scrobbles::*;

/// Shows a bunch of information from Last.fm.
#[command]
#[aliases("lfm", "fm", "last")]
#[sub_commands(profile, nowplaying, scrobbles)]
async fn lastfm(context: &Context, message: &Message) -> CommandResult {
    message.channel_id.say(context, "No valid subcommand provided. Do `help lastfm` to see the commands.").await?;
    Ok(())
}
