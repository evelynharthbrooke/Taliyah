use itertools::Itertools;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::gateway::ActivityType;
use serenity::model::guild::Role;
use serenity::model::prelude::Message;
use serenity::model::user::OnlineStatus;
use serenity::utils::Colour;

#[command]
#[description = "Shows various information about a user."]
#[usage = "<user> or <blank>"]
#[aliases("user", "userinfo", "uinfo", "u")]
#[only_in("guilds")]
pub fn user(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let cache = &ctx.cache;
    let guild_id = msg.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.read().guild(guild_id).ok_or("Unable to retrieve guild")?;
    let member = if msg.mentions.is_empty() {
        if args.is_empty() {
            msg.member(&ctx).ok_or("Could not find member.")?
        } else {
            (*(cached_guild
                .read()
                .members_containing(args.rest(), false, true)
                .first()
                .ok_or("couldn't find member")?))
            .clone()
        }
    } else {
        guild_id.member(&ctx, msg.mentions.first().ok_or("Failed to get user mentioned.")?)?
    };

    let user = member.user.read();
    let guild = cached_guild.read();

    let status: String;
    let main_role: String;
    let mut activity_kind = "".to_owned();
    let mut activity_name = "no available activity".to_owned();
    let mut activity_emoji = "".to_owned();

    match guild.presences.get(&user.id).is_none() {
        true => {
            println!("No status for this user could be found.");
            status = "No status available.".to_owned();
        }
        false => {
            let presence = guild.presences.get(&user.id).unwrap();
            match presence.activity.is_none() {
                true => println!("No activity could be detected. Omitting."),
                false => {
                    let activity = presence.activity.as_ref().ok_or("Cannot retrieve status")?;
                    let emoji = match activity.emoji.is_none() {
                        true => "".to_owned(),
                        false => activity.emoji.as_ref().unwrap().name.to_owned(),
                    };

                    activity_kind = match activity.kind {
                        ActivityType::Listening => "listening to ".to_owned(),
                        ActivityType::Playing => "playing ".to_owned(),
                        ActivityType::Streaming => "streaming ".to_owned(),
                        _ => "".to_owned(),
                    };

                    if activity.kind == ActivityType::Custom {
                        activity_name = match activity.state.is_none() {
                            true => "".to_owned(),
                            false => activity.state.as_ref().unwrap().to_owned(),
                        };
                        activity_emoji = match activity.emoji.is_none() {
                            true => "".to_owned(),
                            false => emoji,
                        };
                        activity_emoji.push_str(" ");
                    } else {
                        activity_name = activity.name.to_owned()
                    }
                }
            }

            status = match presence.status {
                OnlineStatus::Online => match user.bot {
                    true => "Available".to_owned(),
                    false => "Online".to_owned(),
                },
                OnlineStatus::Idle => "Idle".to_owned(),
                OnlineStatus::DoNotDisturb => "Do Not Disturb".to_owned(),
                OnlineStatus::Invisible => "Invisible".to_owned(),
                _ => match user.bot {
                    true => "Unavailable".to_owned(),
                    false => "Offline".to_owned(),
                },
            };
        }
    };

    let account_type = match user.bot {
        true => "Bot".to_owned(),
        false => "User".to_owned(),
    };

    let activity = format!("({}{}{})", activity_kind, activity_emoji, activity_name);
    let created = user.created_at().format("%B %e, %Y - %I:%M %p");
    let tag = user.tag();
    let id = user.id;
    let color: Colour;
    let color_hex: String;

    match member.colour(cache).is_none() {
        true => {
            color = Colour::new(0xFFFFFF);
            color_hex = "No display color available.".to_owned()
        }
        false => {
            color = member.colour(cache).unwrap();
            color_hex = format!("#{}", color.hex());
        }
    }

    let mut roles: String = "".to_owned();
    let mut role_count = 0;
    match member.roles(&cache).is_none() {
        true => println!("No roles available for this user."),
        false => {
            roles = member.roles(&cache).unwrap().iter().map(|r: &Role| &r.name).join(" / ");
            role_count = member.roles(&cache).unwrap().len();
            if roles.is_empty() {
                roles = "No roles available.".to_owned();
            }
        }
    }

    match member.highest_role_info(&cache).is_none() {
        true => {
            println!("Cannot get role information.");
            main_role = "No main role available.".to_owned();
        }
        false => {
            let hoist_role_id = member.highest_role_info(&cache).ok_or("cannot get role id")?.0;
            let hoist_role = guild.roles.get(&hoist_role_id).ok_or("Cannot get role")?;
            main_role = hoist_role.name.to_owned();
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
                        **Status**: {} {}\n\
                        **Type**: {}\n\
                        **Tag**: {}\n\
                        **ID**: {}\n\
                        **Creation Date**: {}\n\n\
                        **__Guild-related Information__**:\n\
                        **Join Date**: {}\n\
                        **Nickname**: {}\n\
                        **Display Color**: {}\n\
                        **Main Role**: {}\n\
                        **Roles ({})**: {}
                        ",
                    status, activity, account_type, tag, id, created, joined, nickname, color_hex, main_role, role_count, roles
                ))
            })
        })
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}
