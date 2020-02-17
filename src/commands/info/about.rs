use crate::config::Config;
use crate::utilities::git_utils::{show_branch, show_head_rev};

use chrono::prelude::*;

use git2::Repository;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::utils::Colour;

use sysinfo::{get_current_pid, ProcessExt, System, SystemExt};

#[command]
#[aliases("info", "botinfo")]
/// Gets various information about the bot.
pub fn about(context: &mut Context, message: &Message) -> CommandResult {
    let cache = &context.cache.read();
    let config = Config::load_from_file("config.toml");
    let repo = Repository::open(env!("CARGO_MANIFEST_DIR"))?;

    let owner = context.http.get_current_application_info()?.owner;
    let owner_id = owner.id.as_u64().to_string();
    let owner_tag = owner.tag();

    let system = System::new_all();
    let pid = get_current_pid()?;
    let process = system.get_process(pid).unwrap();
    let cpu_usage = (process.cpu_usage() * 100.0).round() / 100.0;
    let memory = process.memory() / 1000;

    let version = env!("CARGO_PKG_VERSION").to_string();
    let codename = config.bot_codename.unwrap();
    let branch = show_branch(&repo);
    let revision = show_head_rev(&repo);
    let avatar = cache.user.avatar_url().unwrap();
    let user_id = cache.user.id.as_u64();
    let users = cache.users.len().to_string();
    let guilds = cache.guilds.len().to_string();
    let channels = cache.channels.len().to_string();
    let shards = cache.shard_count.to_string();
    let color = Colour::from_rgb(0, 191, 255);

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title("Ellie Information");
            embed.thumbnail(avatar);
            embed.color(color);
            embed.fields(vec![
                ("Version", version, true),
                ("Codename", codename, true),
                ("Branch", branch, true),
                ("Revision", format!("`{}`", revision), true),
                ("Owner", owner_tag, true),
                ("Owner ID", format!("`{}`", owner_id), true),
                ("Users", users, true),
                ("Guilds", guilds, true),
                ("Shards", shards, true),
                ("Channels", channels, true),
                ("CPU", format!("{}%", cpu_usage), true),
                ("Memory", format!("{} MB", memory), true),
            ]);
            embed.footer(|footer| footer.text(format!("Bot user ID: {}", user_id)));
            embed.timestamp(&Utc::now())
        })
    })?;

    Ok(())
}
