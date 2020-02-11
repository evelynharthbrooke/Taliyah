pub mod subreddit;
pub mod user;

use crate::commands::search::reddit::subreddit::*;
use crate::commands::search::reddit::user::*;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[description("Gets a variety of information from the Reddit API.")]
#[aliases("r")]
#[sub_commands(subreddit, user)]
fn reddit(context: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title("Error: Invalid / No Subcommand Entered!");
            embed.description(
                "You did not enter a valid subcommand! Please check \
                `<prefix>help reddit` for the command usage.",
            )
        })
    })?;

    Ok(())
}
