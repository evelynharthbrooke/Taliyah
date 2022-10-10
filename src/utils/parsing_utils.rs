use std::num::NonZeroU64;

use serenity::{
    model::id::{ChannelId, GuildId, UserId},
    prelude::Context,
    utils::{parse_channel as parse_channel_name, parse_username}
};

pub async fn parse_user(name: &str, guild_id: GuildId, context: &Context) -> Option<UserId> {
    let guild = guild_id.to_guild_cached(&context).unwrap().clone();
    if let Some(x) = parse_username(name) {
        return Some(UserId(NonZeroU64::new(x.get()).unwrap()));
    } else if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(context, id).await {
            return Some(m.user.id);
        }
    }

    if let Some(m) = guild.member_named(name) {
        return Some(m.user.id);
    } else if let Some(m) = guild.members_starting_with(name, false, true).get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    } else if let Some(m) = guild.members_containing(name, false, true).get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    None
}

pub async fn parse_channel(name: &str, guild_id: GuildId, context: &Context) -> Option<ChannelId> {
    let guild = guild_id.to_guild_cached(&context).unwrap();
    if let Some(x) = parse_channel_name(name) {
        return Some(ChannelId(x.into()));
    } else if let Ok(id) = name.parse::<u64>() {
        if let Some(x) = ChannelId(NonZeroU64::new(id).unwrap()).to_channel_cached(context) {
            return Some(x.id());
        }
    }

    for (key, value) in guild.channels.iter() {
        let channel = value.clone().guild(&context.cache).unwrap();
        let channel_name = &channel.name;
        if channel_name == name || channel_name.contains(name) {
            return Some(*key);
        }
    }

    None
}
