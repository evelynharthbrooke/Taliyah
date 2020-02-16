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
#[sub_commands(set, clear)]
/// Retrieves the current server prefix if available. However, there
/// are several sub commands available that allow clearing and setting
/// the server prefix. The available sub-commands are listed below.
/// 
/// *Sub-commands*:
/// `set`: Sets the current server prefix to the one provided by the user.
/// `clear`: Clears the current server prefix and resets it back to the
/// default.
fn prefix(context: &mut Context, message: &Message) -> CommandResult {
    let prefix = database::get_prefix(message.guild_id.unwrap())?;

    let guild = match message.guild(&context.cache) {
        Some(guild) => guild,
        None => {
            message.channel_id.say(&context, "Unable to get the command prefix, as the guild cannot be located.")?;
            return Ok(());
        }
    };

    let guild_name = &guild.read().name;

    message.channel_id.say(&context, format!("The current prefix for **{}** is `{}`.", guild_name, prefix))?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[owners_only]
#[num_args(1)]
#[description = "Sets the command prefix for the current server."]
pub fn set(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    let connection = match get_database() {
        Ok(connection) => connection,
        Err(_) => return Ok(()),
    };

    let prefix = arguments.current().unwrap_or(";");

    let guild = match message.guild(&context.cache) {
        Some(guild) => guild,
        None => {
            message.channel_id.say(&context, "Unable to set command prefix, as the guild cannot be located.")?;
            return Ok(());
        }
    };

    let guild_id = &guild.read().id.as_u64().to_string();
    let guild_name = &guild.read().name;

    let _ = connection.execute(
        "INSERT OR REPLACE INTO guild_settings (guild_id, guild_name, prefix) values (?1, ?2, ?3)",
        &[&guild_id, &guild_name, &prefix.to_string()],
    );

    message.channel_id.say(&context, format!("The prefix for **{}** has been set to `{}`.", guild_name, prefix))?;

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

    message.channel_id.say(&ctx, format!("The prefix for **{}** has been cleared.", guild_name))?;

    Ok(())
}
