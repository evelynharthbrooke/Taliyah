use serenity::client::Context;
use serenity::framework::standard::help_commands;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::Args;
use serenity::framework::standard::HelpOptions;
use serenity::framework::standard::{CommandGroup, CommandResult};
use serenity::model::prelude::{Message, UserId};

use std::collections::HashSet;

#[help]
#[max_levenshtein_distance(3)]
#[no_help_available_text(
    "**Error**: I was unable to find any information on this command, \
    usually indicating that this command does not exist or does not have \
    any help available for said command. Please try again later, or try \
    searching for a different command instead."
)]
async fn help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    help_commands::with_embeds(context, msg, args, help_options, groups, owners).await
}
