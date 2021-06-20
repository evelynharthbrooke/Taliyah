pub mod album;
pub mod newreleases;
pub mod status;
pub mod track;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

use self::album::*;
use self::newreleases::*;
use self::status::*;
use self::track::*;

/// Retrieves information from the Spotify API about a variety of media types, such as albums,
/// tracks, and other kinds of media.
///
/// You can also view a user's current Spotify status, if they have one.
#[command]
#[aliases("sp", "spot")]
#[sub_commands(album, newreleases, status, track)]
async fn spotify(context: &Context, message: &Message) -> CommandResult {
    message.channel_id.say(context, "No valid subcommand provided. Do `help spotify` to see the commands.").await?;
    Ok(())
}
