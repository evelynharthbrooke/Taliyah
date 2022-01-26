use itertools::Itertools;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{channel::ChannelType, prelude::Message}
};

use std::fmt::Write;

#[command]
#[description = "Shows various information about the current guild."]
#[aliases("guild", "guildinfo", "ginfo", "server", "serverinfo", "serverstats", "sinfo")]
#[only_in(guilds)]
async fn guild(context: &Context, message: &Message) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.unwrap();
    let guild_id_u64 = guild_id.as_u64();
    let cached_guild = cache.guild(guild_id).unwrap();

    let guild_icon = cached_guild.icon_url().unwrap();
    let guild_name = &cached_guild.name;
    let guild_owner = cached_guild.member(&context, cached_guild.owner_id).await.unwrap().user.tag();
    let guild_system_channel = cached_guild.system_channel_id.unwrap();
    let guild_system_channel_id = guild_system_channel.as_u64();
    let guild_creation_date = guild_id.created_at().format("%B %e, %Y @ %l:%M %P");
    let guild_members = cached_guild.members.len();
    let guild_presences = cached_guild.presences.len();
    let guild_channels: Vec<_> = cached_guild.channels.iter().filter_map(|(_, c)| c.clone().guild()).collect();
    let guild_channels_all = guild_channels.len();
    let guild_channels_text = guild_channels.iter().filter(|c| c.kind == ChannelType::Text).count();
    let guild_channels_voice = guild_channels.iter().filter(|c| c.kind == ChannelType::Voice).count();
    let guild_emojis = cached_guild.emojis.len();
    let guild_emojis_animated = cached_guild.emojis.iter().filter(|(_, e)| e.animated).count();
    let guild_emojis_normal = cached_guild.emojis.iter().filter(|(_, e)| !e.animated).count();
    let guild_features = cached_guild.features.iter().join(", ");

    let guild_verification_level = match cached_guild.verification_level.num() {
        0 => "None - Unrestricted.",
        1 => "Low - Must have a verified email.",
        2 => "Medium - Registered on Discord for 5+ minutes.",
        3 => "(╯°□°)╯︵ ┻━┻ - In the server for 10+ minutes.",
        4 => "┻━┻ ﾐヽ(ಠ益ಠ)/彡┻━┻) - Must have a verified phone number.",
        _ => "Unrecognized verification level."
    };

    let guild_mfa_level = match cached_guild.mfa_level.num() {
        0 => "Multi-factor authentication not required.",
        1 => "Multi-factor authentication required.",
        _ => "Unrecognized multi-factor authentication level."
    };

    let guild_explicit_filter = match cached_guild.explicit_content_filter.num() {
        0 => "Disabled".to_owned(),
        1 => "Media scanned from members w/o a role.".to_owned(),
        2 => "Everyone".to_owned(),
        _ => "Unrecognized filter setting.".to_owned()
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

    let mut highest = None;
    for role_id in cached_guild.roles.keys() {
        if let Some(role) = cached_guild.roles.get(role_id) {
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

    let mut guild_summary = String::new();
    writeln!(guild_summary, "**Owner**: {}", guild_owner)?;
    writeln!(guild_summary, "**System Channel**: <#{}>", guild_system_channel_id)?;
    writeln!(guild_summary, "**Creation Date**: {}", guild_creation_date)?;
    writeln!(guild_summary, "**Online Members**: {}", guild_presences)?;
    writeln!(guild_summary, "**Total Members**: {}", guild_members)?;
    writeln!(
        guild_summary,
        "**Channels**: {} ({} text, {} voice)",
        guild_channels_all, guild_channels_text, guild_channels_voice
    )?;
    writeln!(guild_summary, "**Emojis**: {} ({} static, {} animated)", guild_emojis, guild_emojis_normal, guild_emojis_animated)?;
    writeln!(guild_summary, "**Features**: {}", if !guild_features.is_empty() { &guild_features } else { "None" })?;
    writeln!(guild_summary, "**MFA Level**: {}", guild_mfa_level)?;
    writeln!(guild_summary, "**Verification Level**: {}", guild_verification_level)?;
    writeln!(guild_summary, "**Explicit Content Filter**: {}", guild_explicit_filter)?;
    writeln!(guild_summary, "**Nitro Boosts**: {}", guild_boosts)?;
    writeln!(guild_summary, "**Nitro Boost Level**: {}", guild_boost_tier)?;
    writeln!(guild_summary, "**Highest Role**: {}", highest_role_name)?;
    writeln!(guild_summary, "**Roles ({})**: {}", guild_role_count, guild_roles_map)?;

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.author(|a| a.name(&guild_name).icon_url(guild_icon));
                embed.colour(highest_role_color);
                embed.description(&guild_summary);
                embed.footer(|f| f.text(format!("{} server ID: {}", guild_name, guild_id)))
            })
        })
        .await?;

    Ok(())
}
