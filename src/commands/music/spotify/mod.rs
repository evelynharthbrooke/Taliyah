pub mod album;
pub mod status;
pub mod track;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

use self::album::*;
use self::status::*;
use self::track::*;

/// Retrieves information from the Spotify API about a varity of media types; e.g. albums, tracks, etc.
#[command]
#[aliases("sp")]
#[sub_commands(album, status, track)]
async fn spotify(context: &Context, message: &Message) -> CommandResult {
    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: Invalid / No subcommand provided.");
                embed.color(0x00FF_0000);
                embed.description("No valid subcommand provided. Please check the help command for help details.")
            })
        })
        .await?;

    Ok(())
}
