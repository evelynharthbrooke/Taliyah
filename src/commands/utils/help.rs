use serenity::client::Context;
use serenity::framework::standard::help_commands::with_embeds;
use serenity::framework::standard::macros::help;
use serenity::framework::standard::Args;
use serenity::framework::standard::HelpOptions;
use serenity::framework::standard::{CommandGroup, CommandResult};
use serenity::model::prelude::{Message, UserId};

use std::collections::HashSet;

#[help]
fn help(
    context: &mut Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    with_embeds(context, msg, args, &help_options, groups, owners)
}
