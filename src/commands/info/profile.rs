use crate::utilities::database;

use log::error;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
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
            (*(cached_guild.read().members_containing(args.rest(), false, true).first().ok_or("couldn't find member")?)).clone()
        }
    } else {
        guild_id.member(&ctx, message.mentions.first().ok_or("Failed to get user mentioned.")?)?
    };

    let user_name = member.user.read().tag().to_string();
    let user_id = member.user.read().id;
    let display_name = match database::get_user_display_name(&user_id) {
        Ok(dn) => dn.to_string(),
        Err(e) => {
            error!("Error while retrieving the Display Name from the database: {}", e);
            "No display name set.".to_string()
        }
    };

    let lastfm_name = match database::get_user_lastfm(&user_id) {
        Ok(ln) => ln.to_string(),
        Err(e) => {
            error!("Error while retrieving Last.fm username from the database: {}", e);
            "No last.fm username set.".to_string()
        }
    };

    let twitter_name = match database::get_user_twitter(&user_id) {
        Ok(tn) => {
            let twitter_url = "https://twitter.com".to_string();
            format!("[{twitter_username}]({twitter_url}/{twitter_username})", twitter_url = twitter_url, twitter_username = tn.to_string())
        }
        Err(e) => {
            error!("Error while retrieving the Twitter username from the database: {}", e);
            "No Twitter username set.".to_string()
        }
    };

    let steam_id = match database::get_user_steam(&user_id) {
        Ok(si) => {
            let steam_url = "https://steamcommunity.com/id".to_string();
            format!("[{steam_id}]({steam_url}/{steam_id})", steam_id = si.to_string(), steam_url = steam_url,)
        }
        Err(e) => {
            error!("Error while retrieving the Steam ID from the database: {}", e);
            "No Steam ID set.".to_string()
        }
    };

    message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.author(|a| {
                a.name(format!("{}'s profile", user_name));
                a.icon_url(&member.user.read().face())
            });
            e.color(0xff99cc);
            e.description(format!(
                "\
                    **Display name**: {}\n\
                    **Last.fm username**: {}\n\
                    **Twitter handle**: {}\n\
                    **Steam ID**: {}\n\
                    ",
                display_name, lastfm_name, twitter_name, steam_id
            ))
        })
    })?;

    return Ok(());
}

#[command]
#[only_in("guilds")]
pub fn set(ctx: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    let connection = match database::get_database() {
        Ok(connection) => connection,
        Err(_) => return Ok(()),
    };

    let property = arguments.single::<String>()?;
    let value = arguments.rest();

    let user_id = message.author.id.to_string();

    match property.as_str() {
        "twitter" => {
            if value.is_empty() {
                message.channel_id.say(&ctx.http, "You did not provide a Twitter username. Please provide one!")?;
                return Ok(());
            };
            let _ = connection.execute("UPDATE profile SET twitter = ?1 WHERE user_id = ?2;", &[&value, &user_id[..]]);
            message.channel_id.say(&ctx, format!("Your Twitter username has been set to `{}`.", &value))?;
        }
        "lastfm" => {
            let api_key: String = env::var("LASTFM_KEY").expect("No API key detected");
            let mut client: Client = Client::new(&api_key);

            match client.user_info(&value).send() {
                Ok(_) => (),
                Err(e) => match e {
                    Error::LastFMError(InvalidParameter(e)) => match e.message.as_str() {
                        "User not found" => {
                            message.channel_id.send_message(&ctx, |m| {
                                m.embed(|e| {
                                    e.title("Error: Invalid username provided.");
                                    e.description("You cannot use this as your profile's Last.fm username.");
                                    e.color(0x00FF_0000)
                                })
                            })?;
                            return Ok(());
                        }
                        _ => (),
                    },
                    _ => (),
                },
            };

            if value.is_empty() {
                message.channel_id.say(&ctx, "You did not provide your last.fm username. Please provide one!")?;
                return Ok(());
            };
            let _ = connection.execute("UPDATE profile SET lastfm = ?1 WHERE user_id = ?2;", &[&value, &user_id[..]]);
            message.channel_id.say(&ctx, format!("Your lastfm username has been set to {}.", &value))?;
        }
        "steam" => {
            if value.is_empty() {
                message.channel_id.say(&ctx, "You did not provide a Steam ID. Please provide one!")?;
                return Ok(());
            };
            let _ = connection.execute("UPDATE profile SET steam = ?1 WHERE user_id = ?2;", &[&value, &user_id[..]]);
            message.channel_id.say(&ctx, format!("Your Steam ID has been set to `{}`.", &value))?;
        }
        "name" => {
            if value.is_empty() {
                message.channel_id.say(&ctx, "You did not provide a name. Please provide one!")?;
                return Ok(());
            };
            let _ = connection.execute("UPDATE profile SET display_name = ?1 WHERE user_id = ?2;", &[&value, &user_id[..]]);
            let _ = connection.close().unwrap();
            message.channel_id.say(&ctx, format!("Your name has been set to {}.", &value))?;
        }
        _ => {
            message.channel_id.say(&ctx, "That is not a valid profile property.")?;
        }
    }

    return Ok(());
}
