use crate::utilities::database;
use crate::utilities::database::get_database;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandError;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::prelude::Context;

#[command]
#[only_in(guilds)]
#[owners_only]
#[sub_commands(get, set)]
#[description("Get or set the command prefix for the current server.")]
fn prefix(ctx: &mut Context, message: &Message) -> CommandResult {
    message
        .channel_id
        .send_message(&ctx, move |m| {
            m.embed(move |e| {
                e.title("Error: Invalid / No Subcommand Entered!");
                e.description("Please use subcommand get or set to use this command.");
                e
            })
        })
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}

#[command]
#[only_in(guilds)]
#[owners_only]
#[description = "Retrieves the command prefix for the current server."]
pub fn get(ctx: &mut Context, message: &Message) -> CommandResult {
    let prefix = database::get_prefix(&message.guild_id.unwrap()).unwrap().to_string();
    let guild = message.guild(&ctx.cache).unwrap();
    let guild_name = &guild.read().name;

    return message
        .channel_id
        .say(&ctx.http, format!("The currently set command prefix for {} is {}.", guild_name, prefix))
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
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
    let guild = message.guild(&ctx.cache).unwrap();
    let guild_id = guild.read().clone().id.as_u64().to_string();
    let guild_name = guild.read().clone().name;

    let _ = connection.execute(
        "INSERT OR REPLACE INTO guild_settings (guild_id, guild_name, prefix) values (?1, ?2, ?3)",
        &[&guild_id, &guild_name, prefix],
    );

    return message
        .channel_id
        .say(&ctx.http, format!("The command prefix for {} has been set to {}.", guild_name, prefix))
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
}
