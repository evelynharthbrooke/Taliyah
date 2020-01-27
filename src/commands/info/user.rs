use itertools::Itertools;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::gateway::Activity;
use serenity::model::gateway::ActivityType;
use serenity::model::guild::Role;
use serenity::model::prelude::Message;
use serenity::model::user::OnlineStatus;
use serenity::utils::Colour;

use log::info;

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
            (*(cached_guild.read().members_containing(args.rest(), false, true).first().ok_or("couldn't find member")?)).clone()
        }
    } else {
        guild_id.member(&ctx, msg.mentions.first().ok_or("Failed to get user mentioned.")?)?
    };

    let user = member.user.read();
    let guild = cached_guild.read();

    let mut root_activity: String = "".to_string();
    let status: String;
    let main_role: String;

    match guild.presences.get(&user.id).is_none() {
        true => {
            info!("No status for this user could be found.");
            status = "No status available.".to_owned();
        }
        false => {
            let presence = guild.presences.get(&user.id).unwrap();

            match presence.activities.is_empty() {
                true => info!("No activities could be found."),
                false => {
                    if presence.activities.len() == 1 {
                        let activities = &presence.activities;
                        let activity = activities.first().unwrap();

                        let activity_name = &activity.name;

                        let emoji = match activity.emoji.is_none() {
                            true => "".to_owned(),
                            false => activity.emoji.as_ref().unwrap().name.to_owned(),
                        };

                        let activity_kind = match activity.kind {
                            ActivityType::Listening => "listening to ".to_owned(),
                            ActivityType::Playing => {
                                if activity_name == "Visual Studio Code" {
                                    "developing in ".to_owned()
                                } else {
                                    "playing ".to_owned()
                                }
                            }
                            ActivityType::Streaming => "streaming ".to_owned(),
                            _ => "".to_owned(),
                        };

                        if activity.kind == ActivityType::Custom {
                            let name = match activity.state.is_none() {
                                true => "".to_owned(),
                                false => activity.state.as_ref().unwrap().to_owned(),
                            };

                            let emoji = match activity.emoji.is_none() {
                                true => "".to_owned(),
                                false => emoji.clone(),
                            };

                            root_activity = format!("({}{})", emoji, name)
                        } else {
                            root_activity = format!("({}{}**{}**)", emoji, activity_kind, activity_name)
                        }
                    } else {
                        root_activity.push('(');

                        let activity = presence
                            .activities
                            .iter()
                            .filter(|a| a.kind != ActivityType::Custom)
                            .map(|activity: &Activity| {
                                let activity_name = activity.name.as_str();
                                let activity_kind = match activity.kind {
                                    ActivityType::Listening => "listening to".to_owned(),
                                    ActivityType::Playing => {
                                        if activity_name == "Visual Studio Code" {
                                            "developing in".to_owned()
                                        } else {
                                            "playing".to_owned()
                                        }
                                    }
                                    ActivityType::Streaming => "streaming on".to_owned(),
                                    _ => "".to_owned(),
                                };

                                format!("{} **{}**", activity_kind, activity_name)
                            })
                            .join(" & ");

                        root_activity.push_str(activity.as_str());
                        root_activity.push(')');
                    }
                }
            };

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

    let created = user.created_at().format("%A, %B %e, %Y @ %I:%M %P");
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
            color_hex = format!("#{}", color.hex().to_lowercase());
        }
    }

    let mut roles: String = "".to_owned();
    let mut role_count = 0;
    match member.roles(&cache).is_none() {
        true => info!("No roles available for this user."),
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
            info!("Cannot get role information.");
            main_role = "No main role available.".to_owned();
        }
        false => {
            let hoist_role_id = member.highest_role_info(&cache).ok_or("cannot get role id")?.0;
            let hoist_role = guild.roles.get(&hoist_role_id).ok_or("Cannot get role")?;
            main_role = hoist_role.name.to_owned();
        }
    }

    let nickname = member.nick.map_or("No nickname has been set.".to_owned(), |nick| nick);
    let joined = member.joined_at.map_or("Unavailable".to_owned(), |d| {
        let formatted_string = d.format("%A, %B %e, %Y @ %I:%M %P");
        format!("{}", formatted_string)
    });

    msg.channel_id
        .send_message(&ctx, move |m| {
            m.embed(move |e| {
                e.author(|a| a.name(&user.name).icon_url(&user.face())).colour(color).description(format!(
                    "\
                    **__General__**:\n\
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
                    **Roles ({})**: {}",
                    status, root_activity, account_type, tag, id, created, joined, nickname, color_hex, main_role, role_count, roles
                ))
            })
        })
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}
