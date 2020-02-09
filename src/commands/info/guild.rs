use itertools::Itertools;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::ChannelType::{Category, Text, Voice};
use serenity::model::prelude::Message;

#[command]
#[description = "Shows various information about the current guild."]
#[usage = "<blank>"]
#[aliases("guild", "guildinfo", "ginfo", "g")]
#[only_in("guilds")]
pub fn guild(context: &mut Context, message: &Message) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.unwrap();
    let cached_guild = cache.read().guild(guild_id).unwrap();

    let guild = cached_guild.read();
    let guild_name = &guild.name;
    let guild_owner = guild.member(&context, guild.owner_id).unwrap().user.read().tag();
    let guild_channels = guild.channels.len();
    let guild_channels_category = guild.channels.iter().filter(|(_, c)| c.read().kind == Category).count();
    let guild_channels_text = guild.channels.iter().filter(|(_, c)| c.read().kind == Text).count();
    let guild_channels_voice = guild.channels.iter().filter(|(_, c)| c.read().kind == Voice).count();
    let guild_creation_date = guild_id.created_at().format("%B %e, %Y @ %l:%M %P");
    let guild_emojis = guild.emojis.len();
    let guild_emojis_animated = guild.emojis.iter().filter(|(_, e)| e.animated).count();
    let guild_emojis_normal = guild.emojis.iter().filter(|(_, e)| !e.animated).count();
    let guild_members = guild.members.len();
    let guild_members_users = guild.members.iter().filter(|(u, _)| !u.to_user(&context).unwrap().bot).count();
    let guild_members_bots = guild.members.iter().filter(|(u, _)| u.to_user(&context).unwrap().bot).count();
    let guild_presences = guild.presences.len();
    let guild_presences_users = guild.presences.iter().filter(|(u, _)| !u.to_user(&context).unwrap().bot).count();
    let guild_presences_bots = guild.presences.iter().filter(|(u, _)| u.to_user(&context).unwrap().bot).count();
    let guild_prefix = crate::utilities::database::get_prefix(guild_id).unwrap();
    let guild_icon = guild.icon_url().unwrap();

    let guild_explicit_filter = match guild.explicit_content_filter.num() {
        0 => "Disabled".to_owned(),
        1 => "No role".to_owned(),
        2 => "Everyone".to_owned(),
        _ => "Unrecognized filter setting.".to_owned(),
    };

    let guild_region = match guild.region.as_str() {
        "brazil" => "Brazil",
        "europe" => "Europe",
        "eu-central" => "Central Europe",
        "eu-west" => "Western Europe",
        "hongkong" => "Hong Kong",
        "india" => "India",
        "japan" => "Japan",
        "russia" => "Russia",
        "singapore" => "Singapore",
        "southafrica" => "South Africa",
        "sydney" => "Sydney, Australia",
        "us-central" => "Central United States",
        "us-east" => "Eastern United States",
        "us-south" => "Southern United States",
        "us-west" => "Western United States",
        &_ => guild.region.as_str(),
    };

    let guild_boosts = guild.premium_subscription_count;
    let guild_boost_tier = match guild.premium_tier.num() {
        0 => "No current tier (not boosted)",
        1 => "Level 1 (2+ boosts)",
        2 => "Level 2 (15+ boosts)",
        3 => "Level 3 (30+ boosts)",
        _ => "Unrecognized boost tier.",
    };

    let guild_roles = guild.roles.iter().filter(|&(_, r)| &r.id != guild_id.as_u64()).map(|(_, r)| &r.name).join(" / ");
    let guild_role_count = guild.roles.iter().filter(|&(_, r)| &r.id != guild_id.as_u64()).count();

    let guild_verification_level = match guild.verification_level.num() {
        0 => "None - Unrestricted.",
        1 => "Low - Must have a verified email.",
        2 => "Medium - Registered on Discord for 5+ minutes.",
        3 => "(╯°□°）╯︵ ┻━┻ - In the server for 10+ minutes.",
        4 => "┻━┻ ﾐヽ(ಠ益ಠ)ノ彡┻━┻) - Must have a verified phone number.",
        _ => "Unrecognized verification level.",
    };

    let guild_mfa_level = match guild.mfa_level.num() {
        0 => "Multi-factor authentication not required.",
        1 => "Multi-Factor authentication required.",
        _ => "Unrecognized multi-factor authentication level.",
    };

    let mut highest = None;

    for role_id in guild.roles.keys() {
        if let Some(role) = guild.roles.get(&role_id) {
            if let Some((id, pos)) = highest {
                if role.position < pos || (role.position == pos && role.id > id) {
                    continue;
                }
            }
            highest = Some((role.id, role.position));
        }
    }

    let highest_role_id = highest.map(|(id, _)| id).unwrap();
    let highest_role = guild.roles.get(&highest_role_id).unwrap();
    let highest_role_name = &highest_role.name;
    let highest_role_color = highest_role.colour;

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(&guild_name);
                author.icon_url(guild_icon)
            });
            embed.colour(highest_role_color);
            embed.description(format!(
                "**Name**: {}\n\
                **Owner**: {}\n\
                **Online Members**: {} ({} bots, {} users)\n\
                **Total Members**: {} ({} bots, {} users)\n\
                **Channels**: {} ({} categories, {} text, {} voice)\n\
                **Emojis**: {} ({} animated, {} static)\n\
                **Region**: {}\n\
                **Creation Date**: {}\n\
                **MFA Level**: {}\n\
                **Verification Level**: {}\n\
                **Explicit Content Filter**: {}\n\
                **Nitro Boosts**: {}\n\
                **Nitro Boost Level**: {}\n\
                **Highest Role**: {}\n\
                **Roles ({})**: {}\n",
                guild_name,
                guild_owner,
                guild_presences,
                guild_presences_bots,
                guild_presences_users,
                guild_members,
                guild_members_bots,
                guild_members_users,
                guild_channels,
                guild_channels_category,
                guild_channels_text,
                guild_channels_voice,
                guild_emojis,
                guild_emojis_animated,
                guild_emojis_normal,
                guild_region,
                guild_creation_date,
                guild_mfa_level,
                guild_verification_level,
                guild_explicit_filter,
                guild_boosts,
                guild_boost_tier,
                highest_role_name,
                guild_role_count,
                guild_roles
            ));
            embed.footer(|footer| {
                footer.text(format!(
                    "{name} guild ID: {id} | {name} prefix: {prefix}",
                    name = guild_name,
                    id = guild_id,
                    prefix = guild_prefix
                ))
            })
        })
    })?;

    Ok(())
}
