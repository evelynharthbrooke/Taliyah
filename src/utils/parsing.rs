use serenity::{
    model::id::{GuildId, UserId},
    prelude::Context,
    utils::parse_username
};

use std::num::NonZeroU64;

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
