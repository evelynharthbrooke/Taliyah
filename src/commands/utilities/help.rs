use serenity::{
    client::Context,
    framework::standard::{help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions},
    model::prelude::{Message, UserId}
};

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
    context: &Context,
    message: &Message,
    arguments: Args,
    options: &'static HelpOptions,
    command_groups: &[&'static CommandGroup],
    bot_owners: HashSet<UserId>
) -> CommandResult {
    help_commands::plain(context, message, arguments, &options, command_groups, bot_owners).await
}
