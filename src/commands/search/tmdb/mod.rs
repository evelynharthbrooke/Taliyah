pub mod cast;
pub mod collection;
pub mod movie;
pub mod show;

use self::cast::*;
use self::collection::*;
use self::movie::*;
use self::show::*;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

/// Gets a variety of information from the API provided by The Movie Database.
#[command]
#[sub_commands(cast, collection, movie, show)]
async fn tmdb(context: &Context, message: &Message) -> CommandResult {
    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: Invalid / No Subcommand Entered!");
                embed.description("No valid subcommand entered. Please check the help, or try again.")
            })
        })
        .await?;

    Ok(())
}
