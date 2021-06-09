use serenity::{
    model::id::{ChannelId, GuildId, UserId},
    prelude::Context,
    utils::{parse_channel as parse_channel_name, parse_username}
};

pub async fn parse_user(name: &str, guild_id: GuildId, context: &Context) -> Option<UserId> {
    if let Some(x) = parse_username(&name) {
        return Some(UserId(x));
    }

    let guild = guild_id.to_guild_cached(&context).await.unwrap();
    if let Ok(id) = name.parse::<u64>() {
        if let Ok(m) = guild.member(context, id).await {
            return Some(m.user.id);
        }
    }

    if let Some(m) = guild.member_named(name) {
        return Some(m.user.id);
    } else if let Some(m) = guild.members_starting_with(name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    } else if let Some(m) = guild.members_containing(name, false, true).await.get(0) {
        let (mem, _) = m;
        return Some(mem.user.id);
    }

    None
}

pub async fn parse_channel(name: &str, guild_id: Option<&GuildId>, context: Option<&Context>) -> Option<ChannelId> {
    if let Some(x) = parse_channel_name(&name) {
        return Some(ChannelId(x));
    } else if guild_id.is_none() || context.is_none() {
        return None;
    }

    let guild_id = guild_id.unwrap();
    let context = context.unwrap();

    if let Ok(id) = name.parse::<u64>() {
        if let Some(x) = ChannelId(id).to_channel_cached(&context).await {
            return Some(x.id());
        }
    }

    let guild = match guild_id.to_guild_cached(&context).await {
        Some(guild) => guild,
        None => return None
    };

    for (key, value) in guild.channels.iter() {
        let cname = &value.name;
        if cname == name || cname.contains(name) {
            return Some(*key);
        }
    }

    None
}
