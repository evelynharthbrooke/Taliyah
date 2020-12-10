use itertools::Itertools;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{
        channel::ChannelType::{Category, Text, Voice},
        prelude::Message
    }
};

#[command]
#[description = "Shows various information about the current guild."]
#[usage = "<blank>"]
#[aliases("guild", "guildinfo", "ginfo", "g", "server", "serverinfo", "serverstats", "sinfo")]
#[only_in("guilds")]
pub async fn guild(context: &Context, message: &Message) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.unwrap();
    let guild_id_u64 = guild_id.as_u64();
    let cached_guild = cache.guild(guild_id).await.unwrap();

    let guild_name = &cached_guild.name;
    let guild_owner = cached_guild.member(&context, cached_guild.owner_id).await.unwrap().user.tag();
    let guild_system_channel = cached_guild.system_channel_id.unwrap();
    let guild_system_channel_id = guild_system_channel.as_u64();
    let guild_channels = cached_guild.channels.len();
    let guild_channels_category = cached_guild.channels.iter().filter(|(_, c)| c.kind == Category).count();
    let guild_channels_text = cached_guild.channels.iter().filter(|(_, c)| c.kind == Text).count();
    let guild_channels_voice = cached_guild.channels.iter().filter(|(_, c)| c.kind == Voice).count();
    let guild_creation_date = guild_id.created_at().format("%B %e, %Y @ %l:%M %P");
    let guild_emojis = cached_guild.emojis.len();
    let guild_emojis_animated = cached_guild.emojis.iter().filter(|(_, e)| e.animated).count();
    let guild_emojis_normal = cached_guild.emojis.iter().filter(|(_, e)| !e.animated).count();
    let guild_members = cached_guild.members.len();
    let guild_presences = cached_guild.presences.len();
    let guild_icon = cached_guild.icon_url().unwrap();

    let guild_explicit_filter = match cached_guild.explicit_content_filter.num() {
        0 => "Disabled".to_owned(),
        1 => "Media scanned from members w/o a role.".to_owned(),
        2 => "Everyone".to_owned(),
        _ => "Unrecognized filter setting.".to_owned()
    };

    let guild_region = match cached_guild.region.as_str() {
        "us-west" => "Western United States",
        "us-east" => "Eastern United States",
        "us-central" => "Central United States",
        "us-south" => "Southern United States",
        "singapore" => "Singapore",
        "southafrica" => "South Africa",
        "sydney" => "Sydney",
        "europe" => "Europe",
        "brazil" => "Brazil",
        "hongkong" => "Hong Kong",
        "russia" => "Russia",
        "japan" => "Japan",
        "india" => "India",
        "dubai" => "Dubai",
        "amsterdam" => "Amsterdam",
        "london" => "London",
        "frankfurt" => "Frankfurt",
        "eu-central" => "Central Europe",
        "eu-west" => "Western Eurpe",
        &_ => cached_guild.region.as_str()
    };

    let guild_boosts = cached_guild.premium_subscription_count;
    let guild_boost_tier = match cached_guild.premium_tier.num() {
        0 => "No current tier (not boosted)",
        1 => "Level 1 (2+ boosts)",
        2 => "Level 2 (15+ boosts)",
        3 => "Level 3 (30+ boosts)",
        _ => "Unrecognized boost tier."
    };

    let guild_roles_sorted = cached_guild.roles.iter().sorted_by_key(|&(_, r)| -r.position);
    let guild_roles_map = guild_roles_sorted.filter(|&(_, r)| &r.id != guild_id_u64).map(|(_, r)| &r.name).join(" / ");
    let guild_role_count = cached_guild.roles.iter().filter(|&(_, r)| &r.id != guild_id_u64).count();

    let guild_verification_level = match cached_guild.verification_level.num() {
        0 => "None - Unrestricted.",
        1 => "Low - Must have a verified email.",
        2 => "Medium - Registered on Discord for 5+ minutes.",
        3 => "(╯°□°）╯︵ ┻━┻ - In the server for 10+ minutes.",
        4 => "┻━┻ ﾐヽ(ಠ益ಠ)ノ彡┻━┻) - Must have a verified phone number.",
        _ => "Unrecognized verification level."
    };

    let guild_mfa_level = match cached_guild.mfa_level.num() {
        0 => "Multi-factor authentication not required.",
        1 => "Multi-factor authentication required.",
        _ => "Unrecognized multi-factor authentication level."
    };

    let mut highest = None;

    for role_id in cached_guild.roles.keys() {
        if let Some(role) = cached_guild.roles.get(&role_id) {
            if let Some((id, pos)) = highest {
                if role.position < pos || (role.position == pos && role.id > id) {
                    continue;
                }
            }
            highest = Some((role.id, role.position));
        }
    }

    let highest_role_id = highest.map(|(id, _)| id).unwrap();
    let highest_role = cached_guild.roles.get(&highest_role_id).unwrap();
    let highest_role_name = &highest_role.name;
    let highest_role_color = highest_role.colour;

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.name(&guild_name);
                    author.icon_url(guild_icon)
                });
                embed.colour(highest_role_color);
                embed.description(format!(
                    "**Owner**: {}\n\
                    **System Channel**: <#{}>\n\
                    **Online Members**: {}\n\
                    **Total Members**: {}\n\
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
                    guild_owner,
                    guild_system_channel_id,
                    guild_presences,
                    guild_members,
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
                    guild_roles_map
                ));
                embed.footer(|footer| footer.text(format!("{name} server ID: {id}", name = guild_name, id = guild_id)))
            })
        })
        .await?;

    Ok(())
}
