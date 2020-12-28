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
    message.channel_id.say(context, "No valid subcommand entered. Do `help tmdb` to see the commands.").await?;
    Ok(())
}
