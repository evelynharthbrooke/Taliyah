use itertools::Itertools;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandError, CommandResult};
use serenity::model::channel::ChannelType::{Category, Text, Voice};
use serenity::model::guild::ExplicitContentFilter;
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
    let guild_channels = guild.channels.iter().filter(|(_, c)| c.read().kind != Category).collect::<Vec<_>>().len();
    let guild_channels_text = guild.channels.iter().filter(|(_, c)| c.read().kind == Text).collect::<Vec<_>>().len();
    let guild_channels_voice = guild.channels.iter().filter(|(_, c)| c.read().kind == Voice).collect::<Vec<_>>().len();
    let guild_id = guild.id;
    let guild_members = guild.member_count;
    let guild_icon = guild.icon_url().unwrap();

    let guild_explicit_filter = match guild.explicit_content_filter {
        ExplicitContentFilter::None => "Disabled".to_string(),
        ExplicitContentFilter::WithoutRole => "No role".to_string(),
        ExplicitContentFilter::All => "Everyone".to_string(),
        _ => "".to_string(),
    };

    let guild_region = match guild.region.as_str() {
        "brazil" => "Brazil".to_string(),
        "europe" => "Europe".to_string(),
        "eu-central" => "Central Europe".to_string(),
        "eu-west" => "Western Europe".to_string(),
        "hongkong" => "Hong Kong".to_string(),
        "india" => "India".to_string(),
        "japan" => "Japan".to_string(),
        "russia" => "Russia".to_string(),
        "singapore" => "Singapore".to_string(),
        "southafrica" => "South Africa".to_string(),
        "sydney" => "Sydney, Australia".to_string(),
        "us-central" => "Central United States".to_string(),
        "us-east" => "Eastern United States".to_string(),
        "us-south" => "Southern United States".to_string(),
        "us-west" => "Western United States".to_string(),
        &_ => guild.region.to_string(),
    };

    let guild_roles = guild.roles.iter().filter(|&(_, r)| &r.id != guild.id.as_u64()).map(|(_, r)| &r.name).join(" / ");
    let guild_role_count = guild.roles.iter().filter(|&(_, r)| &r.id != guild.id.as_u64()).collect::<Vec<_>>().len();
    let guild_presences = guild.presences.len();
    let guild_creation_date = guild_id.created_at().format("%B %e, %Y - %I:%M %p");

    let mut highest = None;
    for (role_id, _) in &guild.roles {
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
                e.title(format!("Information on guild {}", guild_name));
                e.thumbnail(guild_icon);
                e.colour(highest_role_color);
                e.description(format!("
                    **Name**: {}\n\
                    **Owner**: {}\n\
                    **Members**: {}\n\
                    **Members Online**: {}\n\
                    **Channels**: {} ({} text, {} voice)\n\
                    **Region**: {}\n\
                    **Creation Date**: {}\n\
                    **Explicit Content Filter**: {}\n\
                    **Highest Role**: {}\n\
                    **Roles ({})**: {}\n",
                    guild_name, guild_owner, guild_members, guild_presences, guild_channels,
                    guild_channels_text, guild_channels_voice, guild_region, guild_creation_date,
                    guild_explicit_filter, highest_role_name, guild_role_count, guild_roles
                ))
            })
        })
        .map_or_else(|e| Err(CommandError(e.to_string())), |_| Ok(()))
}
