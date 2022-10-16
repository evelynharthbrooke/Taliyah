use crate::{
    data::ConfigContainer,
    utils::git_utils::{show_branch, show_head_rev}
};

use git2::Repository;
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

#[command]
#[aliases("info", "botinfo")]
async fn about(context: &Context, message: &Message) -> CommandResult {
    let data = context.data.read().await;
    let config = data.get::<ConfigContainer>().unwrap();
    let repo = Repository::open(env!("CARGO_MANIFEST_DIR"))?;

    let version = env!("CARGO_PKG_VERSION").to_string();
    let codename = &config.bot.general.codename;
    let branch = show_branch(&repo);
    let revision = show_head_rev(&repo);

    let current_user = context.cache.current_user().clone();

    let bot_owner = context.http.get_current_application_info().await?.owner.tag();
    let bot_name = &current_user.name;
    let bot_avatar = &current_user.avatar_url().unwrap();

    let num_guilds = context.cache.guilds().len();
    let num_shards = context.cache.shard_count();
    let num_channels = context.cache.guild_channel_count();
    let num_users = context.cache.user_count();

    let about_fields = vec![
        ("Version", version, true),
        ("Codename", codename.to_string(), true),
        ("Branch", branch, true),
        ("Revision", format!("`{revision}`"), true),
        ("Owner", bot_owner, true),
        ("Shards", num_shards.to_string(), true),
        ("Guilds", num_guilds.to_string(), true),
        ("Channels", num_channels.to_string(), true),
        ("Users", num_users.to_string(), true),
    ];

    let embed = CreateEmbed::new()
        .title(format!("**{bot_name}**"))
        .url("https://github.com/evelynmarie/Ellie")
        .thumbnail(bot_avatar)
        .color(0x00BFFF)
        .fields(about_fields)
        .footer(CreateEmbedFooter::new("Written with Rust & serenity."));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

    Ok(())
}
