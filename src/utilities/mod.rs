pub mod built_info;
pub mod database;
pub mod geo_utils;
pub mod git_utils;

use serenity::model::prelude::{GuildId, UserId};
use serenity::prelude::Context;
use serenity::utils::parse_username;

pub fn format_int(integer: usize) -> String {
    let mut string = String::new();
    let integer_str = integer.to_string();
    let a = integer_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            string.insert(0, ',');
        }
        string.insert(0, val);
    }
    string
}

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
        None => return None,
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
