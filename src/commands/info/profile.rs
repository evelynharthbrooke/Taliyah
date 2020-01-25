use crate::utilities::database;

use log::error;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::prelude::Message;

use std::env;

use rustfm::error::Error;
use rustfm::error::LastFMErrorResponse::InvalidParameter;
use rustfm::Client;

#[command]
#[description = "Shows the information about a user via their profile details."]
#[usage = "<user> or <blank>"]
#[sub_commands(set)]
#[only_in("guilds")]
pub fn profile(ctx: &mut Context, message: &Message, args: Args) -> CommandResult {
    let cache = &ctx.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.read().guild(guild_id).ok_or("Unable to retrieve guild")?;
    let member = if message.mentions.is_empty() {
        if args.is_empty() {
            message.member(&ctx).ok_or("Could not find member.")?
        } else {
            (*(cached_guild
                .read()
                .members_containing(args.rest(), false, true)
                .first()
                .ok_or("couldn't find member")?))
            .clone()
        }
    } else {
        guild_id.member(&ctx, message.mentions.first().ok_or("Failed to get user mentioned.")?)?
    };

    let user_name = member.user.read().tag().to_string();
    let user_id = member.user.read().id;
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

    return message
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                e.author(|a| {
                    a.name(format!("{}'s profile", user_name));
                    a.icon_url(&member.user.read().face())
                });
                e.color(0xff99cc);
                e.description(format!(
                    "\
                    **Display Name**: {}\n\
                    **Last.fm Username**: {}\
                    ",
                    display_name, lastfm_name
                ))
            })
        })
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
            let api_key: String = env::var("LASTFM_KEY").expect("No API key detected");
            let mut client: Client = Client::new(&api_key);

            match client.user_info(&value).send() {
                Ok(_) => (),
                Err(e) => match e {
                    Error::LastFMError(InvalidParameter(e)) => match e.message.as_str() {
                        "User not found" => {
                            return message
                                .channel_id
                                .send_message(&ctx, |m| {
                                    m.embed(|e| {
                                        e.title("Error: Invalid username provided.");
                                        e.description("You cannot use this as your profile's Last.fm username.");
                                        e.color(0x00FF_0000)
                                    })
                                })
                                .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
                        }
                        _ => (),
                    },
                    _ => (),
                },
            };

            if value.is_empty() {
                return message
                    .channel_id
                    .say(&ctx.http, "You did not provide your last.fm username. Please provide one!")
                    .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
            };
            let _ = connection.execute("UPDATE profile SET lastfm = ?1 WHERE user_id = ?2;", &[&value, &&user_id]);
            return message
                .channel_id
                .say(&ctx.http, format!("Your lastfm username has been set to {}.", &value))
                .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
        }
        "name" => {
            if value.is_empty() {
                return message
                    .channel_id
                    .say(&ctx.http, "You did not provide a name. Please provide one!")
                    .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()));
            };
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
