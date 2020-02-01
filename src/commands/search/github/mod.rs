pub mod repository;
pub mod user;

use crate::commands::search::github::repository::*;
use crate::commands::search::github::user::*;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[description("Gets a variety of information from the GitHub API.")]
#[sub_commands(user, repository)]
fn github(ctx: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&ctx, |message| {
        message.embed(|embed| {
            embed.title("Error: Invalid / No Subcommand Entered!");
            embed.description(
                "You did not enter a valid subcommand! Please check \
                `<prefix>help github` for the command usage.",
            )
        })
    })?;

    Ok(())
}
