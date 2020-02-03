pub mod album;
pub mod artist;
pub mod credits;
pub mod track;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use crate::commands::music::spotify::album::*;
use crate::commands::music::spotify::credits::*;
use crate::commands::music::spotify::track::*;
use crate::commands::music::spotify::artist::*;

#[command]
#[description(
    "\
    Gets a variety of information from the Spotify API.\n\n\
    Available sub-commands:\n\
    `album <name>`: Retrieves information on a Spotify album.\n\
    `artist <name>`: Retrieves information about a specified Spotify artist.\n\
    `credits <track>`: Retrieves the credits for a specified Spotify track.\n\
    `track <name>`: Retrieves information on a specified Spotify track.\n"
)]
#[sub_commands(album, artist, credits, track)]
fn spotify(ctx: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&ctx, |message| {
        message.embed(|embed| {
            embed.title("Error: Invalid / No subcommand provided.");
            embed.description(format!("No valid subcommand provided. Please check the help command for help details.",))
        })
    })?;

    Ok(())
}
