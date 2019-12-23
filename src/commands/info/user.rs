use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::gateway::ActivityType;
use serenity::model::prelude::Message;
use serenity::model::user::OnlineStatus;

#[command]
#[description = "Shows various information about a user"]
#[only_in("guilds")]
pub fn user(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let cache = &ctx.cache; // get a reference to serenity's cache
    let guild_id = msg.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let member = if msg.mentions.is_empty() {
        if args.is_empty() {
            msg.member(&ctx).ok_or("Could not find member.")?
        } else {
            (*(guild_id
                .to_guild_cached(cache)
                .ok_or("couldn't get guild")?
                .read()
                .members_starting_with(args.rest(), false, true)
                .first()
                .ok_or("couldn't find member")?))
            .clone()
        }
    } else {
        guild_id.member(&ctx, msg.mentions.first().ok_or("Failed to get user mentioned.")?)?
    };

    let cached_guild = cache.read().guild(guild_id).ok_or("Unable to retrieve guild")?;
    let user = member.user.read();
    let guild = cached_guild.read();
    let presence = guild.presences.get(&user.id).ok_or("Cannot retrieve status")?;

    let mut activity_kind = "".to_string();
    let mut activity_name = "".to_string();
    let mut main_role = "".to_string();

    match presence.activity.is_none() {
        true => println!("No activity could be detected. Omitting."),
        false => {
            let activity = presence.activity.as_ref().ok_or("Cannot retrieve status")?;

            activity_kind = match activity.kind {
                ActivityType::Listening => ", listening to".to_string(),
                ActivityType::Playing => ", playing".to_string(),
                ActivityType::Streaming => ", streaming".to_string(),
                _ => "".to_string(),
            };

            if activity.name.is_empty() {
                activity_name = "".to_string();
            } else {
                activity_name = activity.name.to_string()
            }
        }
    }

    let status = match presence.status {
        OnlineStatus::Online => match user.bot {
            true => "Available".to_string(),
            false => "Online".to_string(),
        },
        OnlineStatus::Idle => "Idle".to_string(),
        OnlineStatus::DoNotDisturb => "Do Not Disturb".to_string(),
        OnlineStatus::Invisible => "Invisible".to_string(),
        _ => match user.bot {
            true => "Unavailable".to_string(),
            false => "Offline".to_string(),
        },
    };

    let account_type = match user.bot {
        true => "Bot".to_string(),
        false => "User".to_string(),
    };

    let tag = user.tag();
    let id = user.id;
    let color = member.colour(cache).ok_or("Could not retrieve member color")?;
    let color_hex = color.hex();
    let created = user.created_at().format("%B %e, %Y - %I:%M %p");

    match member.highest_role_info(&cache).is_none() {
        true => println!("Cannot get role information."),
        false => {
            let hoist_role_id = member.highest_role_info(&cache).ok_or("cannot get role id")?.0;
            let hoist_role = guild.roles.get(&hoist_role_id).ok_or("Cannot get role")?;
            main_role = hoist_role.name.to_string();
        }
    }

    let nickname = member.nick.map_or("None".to_owned(), |nick| nick.clone());
    let joined = member.joined_at.map_or("Unavailable".to_owned(), |d| {
        let formatted_string = d.format("%B %e, %Y - %I:%M %p");
        format!("{}", formatted_string)
    });

    msg.channel_id
        .send_message(&ctx, move |m| {
            m.embed(move |e| {
                e.author(|a| a.name(&user.name).icon_url(&user.face())).colour(color).description(format!(
                    "**__General__**:\n\
                        **Status**: {}{} {}\n\
                        **Type**: {}\n\
                        **Tag**: {}\n\
                        **ID**: {}\n\
                        **Creation Date**: {}\n\n\
                        **__Guild-related Information__**:\n\
                        **Join Date**: {}\n\
                        **Nickname**: {}\n\
                        **Display Color**: #{}\n\
                        **Main Role**: {}\n\
                        ",
                    status, activity_kind, activity_name, account_type, tag, id, created, joined, nickname, color_hex, main_role
                ))
            })
        })
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}
