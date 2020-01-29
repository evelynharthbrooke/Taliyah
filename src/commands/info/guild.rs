use itertools::Itertools;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::ChannelType::{Category, Text, Voice};
use serenity::model::prelude::Message;

#[command]
#[description = "Shows various information about the current guild."]
#[usage = "<blank>"]
#[aliases("guild", "guildinfo", "ginfo", "g")]
#[only_in("guilds")]
pub fn guild(ctx: &mut Context, msg: &Message) -> CommandResult {
    let cache = &ctx.cache;
    let guild_id = msg.guild_id.unwrap();
    let cached_guild = cache.read().guild(guild_id).unwrap();
    
    let guild = cached_guild.read();
    let guild_name = &guild.name;
    let guild_owner = guild.member(&ctx, guild.owner_id).unwrap().user.read().tag();
    let guild_channels = guild.channels.iter().filter(|(_, c)| c.read().kind != Category).count();
    let guild_channels_text = guild.channels.iter().filter(|(_, c)| c.read().kind == Text).count();
    let guild_channels_voice = guild.channels.iter().filter(|(_, c)| c.read().kind == Voice).count();
    let guild_creation_date = guild_id.created_at().format("%B %e, %Y @ %l:%M %P");
    let guild_emojis = guild.emojis.len();
    let guild_emojis_animated = guild.emojis.iter().filter(|(_, e)| e.animated).count();
    let guild_emojis_normal = guild.emojis.iter().filter(|(_, e)| !e.animated).count();
    let guild_presences = guild.presences.len();
    let guild_members = guild.member_count;
    let guild_prefix = crate::utilities::database::get_prefix(guild_id).unwrap();
    let guild_icon = guild.icon_url().unwrap();

    let guild_explicit_filter = match guild.explicit_content_filter.num() {
        0 => "Disabled".to_owned(),
        1 => "No role".to_owned(),
        2 => "Everyone".to_owned(),
        _ => "Unrecognized filter setting.".to_owned(),
    };

    let guild_region = match guild.region.as_str() {
        "brazil" => "Brazil".to_owned(),
        "europe" => "Europe".to_owned(),
        "eu-central" => "Central Europe".to_owned(),
        "eu-west" => "Western Europe".to_owned(),
        "hongkong" => "Hong Kong".to_owned(),
        "india" => "India".to_owned(),
        "japan" => "Japan".to_owned(),
        "russia" => "Russia".to_owned(),
        "singapore" => "Singapore".to_owned(),
        "southafrica" => "South Africa".to_owned(),
        "sydney" => "Sydney, Australia".to_owned(),
        "us-central" => "Central United States".to_owned(),
        "us-east" => "Eastern United States".to_owned(),
        "us-south" => "Southern United States".to_owned(),
        "us-west" => "Western United States".to_owned(),
        &_ => guild.region.to_owned(),
    };

    let guild_boosts = guild.premium_subscription_count;
    let guild_boost_tier = match guild.premium_tier.num() {
        0 => "No current tier (not boosted)".to_owned(),
        1 => "Level 1 (2+ boosts)".to_owned(),
        2 => "Level 2 (15+ boosts)".to_owned(),
        3 => "Level 3 (30+ boosts)".to_owned(),
        _ => "Unrecognized boost tier.".to_owned()
    };

    let guild_roles = guild.roles.iter().filter(|&(_, r)| &r.id != guild_id.as_u64()).map(|(_, r)| &r.name).join(" / ");
    let guild_role_count = guild.roles.iter().filter(|&(_, r)| &r.id != guild_id.as_u64()).count();

    let guild_verification_level = match guild.verification_level.num() {
        0 => "None - Unrestricted.".to_owned(),
        1 => "Low - Must have a verified email.".to_owned(),
        2 => "Medium - Registered on Discord for 5+ minutes.".to_owned(),
        3 => "(╯°□°）╯︵ ┻━┻ - In the server for 10+ minutes.".to_owned(),
        4 => "┻━┻ ﾐヽ(ಠ益ಠ)ノ彡┻━┻) - Must have a verified phone number.".to_owned(),
        _ => "Unrecognized verification level.".to_owned()
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

    msg.channel_id
        .send_message(&ctx, move |m| {
            m.embed(move |e| {
                e.author(|a| a.name(&guild_name).icon_url(guild_icon));
                e.colour(highest_role_color);
                e.description(format!("
                    **Name**: {}\n\
                    **Owner**: {}\n\
                    **Prefix**: `{}`\n\
                    **Members**: {}\n\
                    **Members Online**: {}\n\
                    **Channels**: {} ({} text, {} voice)\n\
                    **Emojis**: {} ({} animated, {} static)\n\
                    **Region**: {}\n\
                    **Creation Date**: {}\n\
                    **Verification Level**: {}\n\
                    **Explicit Content Filter**: {}\n\
                    **Nitro Boosts**: {}\n\
                    **Nitro Boost Level**: {}\n\
                    **Highest Role**: {}\n\
                    **Roles ({})**: {}\n",
                    guild_name, guild_owner, guild_prefix, guild_members, guild_presences, guild_channels,
                    guild_channels_text, guild_channels_voice, guild_emojis, guild_emojis_animated, 
                    guild_emojis_normal, guild_region, guild_creation_date, guild_verification_level, 
                    guild_explicit_filter, guild_boosts, guild_boost_tier, highest_role_name, 
                    guild_role_count, guild_roles
                ));
                e.footer(|f| f.text(format!("The ID belonging to {} is {}.", guild_name, guild_id)))
            })
        })
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}
