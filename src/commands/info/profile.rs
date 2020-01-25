use crate::utilities::database;

use log::error;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::prelude::Message;

#[command]
#[description = "Shows the information about a user via their profile details."]
#[usage = "<user> or <blank>"]
#[sub_commands(set)]
#[only_in("guilds")]
pub fn profile(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {
    let user_name = message.author.tag().to_string();
    let user_id = message.author.id;
    let display_name = match database::get_user_display_name(&user_id) {
        Ok(dn) => dn.to_string(),
        Err(e) => {
            error!("Could not get display name from database: {}", e);
            "No display name set.".to_string()
        }
    };

    let lastfm_name = match database::get_user_lastfm_username(&user_id) {
        Ok(ln) => ln.to_string(),
        Err(e) => {
            error!("Couldn't get lastfm username from database: {}", e);
            "No last.fm username set.".to_string()
        }
    };

    return message.channel_id.send_message(&ctx, |m| m.embed(|e| {
                e.author(|a| {
                    a.name(format!("{}'s profile", user_name));
                    a.icon_url(&message.author.face()) 
                });
                e.color(0xff99cc);
                e.description(format!(
                    "\
                    **Display Name**: {}\n\
                    **Last.fm Username**: {}\
                    ",
                    display_name, lastfm_name
                ))
            }))
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
}

#[command]
#[only_in("guilds")]
pub fn set(ctx: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    let connection = match database::get_database() {
        Ok(connection) => connection,
        Err(_) => return Ok(()),
    };

    let property = &arguments.clone().single::<String>().unwrap();
    let value = arguments.clone().advance().rest().to_string();

    let user_id = message.author.id.to_string();

    match property.as_str() {
        "lastfm" => {
            let _ = connection.execute("UPDATE profile SET lastfm = ?1 WHERE user_id = ?2;", &[&value, &&user_id]);
            return message
                .channel_id
                .say(&ctx.http, format!("Your lastfm username has been set to {}.", &value))
                .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
        }
        "name" => {
            let _ = connection.execute("UPDATE profile SET display_name = ?1 WHERE user_id = ?2;", &[&value, &&user_id]);
            return message
                .channel_id
                .say(&ctx.http, format!("Your name has been set to {}.", &value))
                .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
        }
        _ => {
            return message
                .channel_id
                .say(&ctx.http, "That is not a valid profile property.")
                .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
        }
    }
}
