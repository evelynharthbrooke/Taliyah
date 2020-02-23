use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;
use serenity::model::id::UserId;
use serenity::prelude::Context;
use serenity::utils::parse_channel as parse_channel_name;
use serenity::utils::parse_username;

pub fn parse_user(name: &str, guild_id: Option<&GuildId>, context: Option<&Context>) -> Option<UserId> {
    if let Some(x) = parse_username(&name) {
        return Some(UserId(x));
    } else if guild_id.is_none() || context.is_none() {
        return None;
    }

    let guild_id = guild_id.unwrap();
    let context = context.unwrap();

    let cached_guild = match guild_id.to_guild_cached(&context) {
        Some(guild) => guild,
        None => return None
    };

    let guild = cached_guild.read();

    if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(context, id) {
            return Some(m.user.read().id);
        }
    }

    if let Some(m) = guild.member_named(name) {
        return Some(m.user.read().id);
    } else if let Some(m) = guild.members_starting_with(name, false, true).get(0) {
        return Some(m.user.read().id);
    } else if let Some(m) = guild.members_containing(name, false, true).get(0) {
        return Some(m.user.read().id);
    }

    None
}

pub fn parse_channel(name: &str, guild_id: Option<&GuildId>, context: Option<&Context>) -> Option<ChannelId> {
    if let Some(x) = parse_channel_name(&name) {
        return Some(ChannelId(x));
    } else if guild_id.is_none() || context.is_none() {
        return None;
    }

    let guild_id = guild_id.unwrap();
    let context = context.unwrap();

    if let Ok(id) = name.parse::<u64>() {
        if let Some(x) = ChannelId(id).to_channel_cached(&context) {
            return Some(x.id());
        }
    }

    let guild = match guild_id.to_guild_cached(&context) {
        Some(guild) => guild,
        None => return None
    };

    let guild = guild.read();

    for (key, value) in guild.channels.iter() {
        let cname = &value.read().name;
        if cname == name || cname.contains(name) {
            return Some(*key);
        }
    }

    None
}
