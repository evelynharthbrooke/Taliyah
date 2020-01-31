use crate::utilities::parse_user;

use itertools::Itertools;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
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
pub fn user(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.read().guild(guild_id).ok_or("Unable to retrieve guild")?;
    let member = if message.mentions.is_empty() {
        if args.is_empty() {
            message.member(&context).ok_or("Could not find member.")?
        } else {
            match parse_user(&args.rest(), Some(&guild_id), Some(&context)) {
                Some(i) => guild_id.member(&context, i)?,
                None => return Ok(()),
            }
        }
    } else {
        guild_id.member(&context, message.mentions.first().ok_or("Failed to get user mentioned.")?)?
    };

    let user = member.user.read();
    let guild = cached_guild.read();

    let mut activities: String = String::new();
    let mut active_status: String = String::new();

    if !guild.presences.get(&user.id).is_none() {
        let p = guild.presences.get(&user.id).unwrap();

        if !p.activities.is_empty() {
            activities = p
                .activities
                .iter()
                .filter(|a| a.kind != ActivityType::Custom)
                .map(|activity: &Activity| {
                    let mut activity_name = activity.name.as_str();
                    let activity_kind = match activity.kind {
                        ActivityType::Listening => {
                            if activity_name == "Spotify" {
                                let song = activity.details.as_ref().unwrap();
                                let artists = activity.state.as_ref().unwrap().replace(";", " & ");
                                let album = activity.assets.as_ref().unwrap().large_text.as_ref().unwrap();
                                format!("listening to **{}** by **{}** on the album **{}** via", song, artists, album)
                            } else {
                                "listening to".to_owned()
                            }
                        }
                        ActivityType::Playing => {
                            if activity_name == "Visual Studio Code" {
                                let file = activity.details.as_ref().unwrap().replace("Editing ", "");
                                let project = activity.state.as_ref().unwrap().replace("Workspace: ", "");
                                let app = activity.assets.as_ref().unwrap().small_text.as_ref().unwrap();
                                activity_name = app;
                                format!("working on the file **{}** in the project **{}** with", file, project)
                            } else {
                                "playing".to_owned()
                            }
                        }
                        ActivityType::Watching => "watching".to_owned(),
                        ActivityType::Streaming => "streaming on".to_owned(),
                        _ => "".to_owned(),
                    };
                    format!("{} **{}**", activity_kind, activity_name)
                })
                .join(" and ");
        };

        let currently_status: String = format!("{} is currently ", user.name);

        active_status.push_str(currently_status.as_str());

        let status = match p.status {
            OnlineStatus::Online => "Online",
            OnlineStatus::Idle => "Idle",
            OnlineStatus::DoNotDisturb => "Do Not Disturb",
            OnlineStatus::Invisible => "Invisible",
            _ => "Offline",
        };

        if status != "Do Not Disturb" {
            active_status.push_str("**");
            active_status.push_str(status);
            active_status.push_str("**");
        } else {
            active_status.push_str("in **Do Not Disturb** mode");
        }

        if activities.is_empty() {
            active_status.push_str(".\n\n")
        }
    };

    if !activities.is_empty() {
        activities = format!("; {}.\n\n", activities);
    }

    let account_type = if user.bot { "Bot" } else { "User" };

    let created = user.created_at().format("%A, %B %e, %Y @ %l:%M %P");
    let tag = user.tag();
    let id = user.id;
    let color: Colour;
    let hex: String;

    if member.colour(cache).is_none() {
        color = Colour::new(0x00FF_FFFF);
        hex = "No display color available.".to_owned()
    } else {
        color = member.colour(cache).unwrap();
        hex = format!("#{}", color.hex().to_lowercase());
    }

    let mut roles = String::new();
    let mut role_count = 0;

    if !member.roles(&cache).is_none() {
        roles = member.roles(&cache).unwrap().iter().map(|r: &Role| &r.name).join(" / ");
        role_count = member.roles(&cache).unwrap().len();
        if roles.is_empty() {
            roles = "No roles available.".to_owned();
        }
    }

    let main_role = if member.highest_role_info(&cache).is_none() {
        info!("Cannot get role information.");
        "No main role available.".to_owned()
    } else {
        let hoist_role_id = member.highest_role_info(&cache).ok_or("cannot get role id")?.0;
        let hoist_role = guild.roles.get(&hoist_role_id).ok_or("Cannot get role")?;
        hoist_role.name.to_owned()
    };

    let nickname = member.nick.map_or("No nickname has been set.".to_owned(), |nick| nick);
    let joined = member.joined_at.map_or("Unavailable".to_owned(), |d| {
        let formatted_string = d.format("%A, %B %e, %Y @ %l:%M %P");
        format!("{}", formatted_string)
    });

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(&user.name);
                author.icon_url(&user.face())
            });
            embed.colour(color);
            embed.description(format!(
                "{}{}\
                **__User Information__**:\n\
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
                active_status, activities, account_type, tag, id, created, joined, nickname, hex, main_role, role_count, roles
            ))
        })
    })?;

    Ok(())
}
