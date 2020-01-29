use crate::utilities::database;
use crate::utilities::database::get_database;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
#[only_in(guilds)]
#[owners_only(true)]
#[sub_commands(get, set, clear)]
#[description(
    "\
    Retrieves, sets, or clears the command prefix for the current guild.\n\n\
    Sub-commands:\n\
    `get`: Retrieves the currently set command prefix for the guild.\n\
    `set`: Sets the guild's command prefix.\n\
    `clear`: Clears the guild's currently set command prefix. This will reset the \
    command prefix back to the default value in the bot's configuration file.\
    "
)]
fn prefix(ctx: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&ctx, move |m| {
        m.embed(|e| {
            e.title("Error: Invalid / No Subcommand Entered!");
            e.description("Please use subcommand get or set to use this command.")
        })
    })?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[owners_only]
#[description = "Retrieves the command prefix for the current server."]
pub fn get(ctx: &mut Context, message: &Message) -> CommandResult {
    let prefix = database::get_prefix(message.guild_id.unwrap())?;

    let guild = match message.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            message.channel_id.say(&ctx, "Unable to get the command prefix, as the guild cannot be located.")?;
            return Ok(());
        }
    };

    let guild_name = &guild.read().name;

    message.channel_id.say(&ctx, format!("The currently set command prefix for {} is `{}`.", guild_name, prefix))?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(1)]
#[description = "Sets the command prefix for the current server."]
pub fn set(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {
    let connection = match get_database() {
        Ok(connection) => connection,
        Err(_) => return Ok(()),
    };

    let prefix = args.current().unwrap_or(";");

    let guild = match message.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            message.channel_id.say(&ctx, "Unable to set command prefix, as the guild cannot be located.")?;
            return Ok(());
        }
    };

    let guild_id = &guild.read().id.as_u64().to_string();
    let guild_name = &guild.read().name;

    let _ = connection.execute(
        "INSERT OR REPLACE INTO guild_settings (guild_id, guild_name, prefix) values (?1, ?2, ?3)",
        &[&guild_id, &guild_name, &prefix.to_string()],
    );

    message.channel_id.say(&ctx, format!("The command prefix for {} has been set to `{}`.", guild_name, prefix))?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[owners_only]
#[aliases(clear, delete)]
#[description = "Clears the current server's currently set command prefix."]
pub fn clear(ctx: &mut Context, message: &Message) -> CommandResult {
    // Upon running the command, run the DELETE command on the database
    // to remove the set prefix from it.
    database::clear_prefix(message.guild_id.unwrap());

    let guild = match message.guild(&ctx.cache) {
        Some(guild) => guild,
        None => {
            message.channel_id.say(&ctx, "Unable to clear the command prefix, as the guild cannot be located.")?;
            return Ok(());
        }
    };

    let guild_name = &guild.read().name;

    message.channel_id.say(&ctx, format!("The command prefix for {} has been cleared.", guild_name))?;

    Ok(())
}
