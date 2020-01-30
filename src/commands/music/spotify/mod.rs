pub mod album;
pub mod track;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::prelude::Message;

use crate::commands::music::spotify::album::*;
use crate::commands::music::spotify::track::*;

use std::env;

#[command]
#[description(
    "Gets a variety of information from the Spotify API, such as \
    artist information, album information, song information, and more."
)]
#[sub_commands(album, track)]
fn spotify(ctx: &mut Context, message: &Message) -> CommandResult {
    let prefix = env::var("DISCORD_PREFIX").unwrap();

    message.channel_id.send_message(&ctx, |message| {
        message.embed(|embed| {
            embed.title("Error: Invalid / No Subcommand Entered!");
            embed.description(format!(
                "You did not enter a valid subcommand! Please check \
                `{}help spotify` for the command usage.",
                prefix
            ))
        })
    })?;

    Ok(())
}
