use crate::{
    data::ConfigContainer,
    utils::git_utils::{show_branch, show_head_rev}
};

use git2::Repository;
use serenity::{
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

    let current_user = context.cache.current_user();

    let version = env!("CARGO_PKG_VERSION").to_string();
    let codename = &config.bot.general.codename;
    let branch = show_branch(&repo);
    let revision = show_head_rev(&repo);

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
        ("Revision", format!("`{}`", revision), true),
        ("Owner", bot_owner, true),
        ("Shards", num_shards.to_string(), true),
        ("Guilds", num_guilds.to_string(), true),
        ("Channels", num_channels.to_string(), true),
        ("Users", num_users.to_string(), true),
    ];

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.title(format!("**{}**", bot_name));
                embed.url("https://github.com/KamranMackey/Ellie");
                embed.thumbnail(bot_avatar);
                embed.color(0x00BFFF);
                embed.fields(about_fields);
                embed.footer(|footer| footer.text("Written with Rust & serenity."));
                embed
            })
        })
        .await?;

    Ok(())
}
