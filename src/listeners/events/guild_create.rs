use crate::{read_config, DatabasePool};

use serenity::{client::Context, model::guild::Guild};
use tracing::info;

pub async fn guild_create(context: Context, guild: Guild, is_new: bool) {
    let config = read_config("config.toml");
    let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();

    let guild_name = guild.name;
    let guild_id = guild.id.0 as i64;
    let guild_prefix = config.bot.general.prefix;

    if is_new {
        info!("Adding guild {} to the database.", guild_name);
        sqlx::query!(
            "INSERT INTO guild_info (guild_id, guild_name, guild_prefix) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
            guild_id,
            guild_name,
            guild_prefix
        )
        .execute(&pool)
        .await
        .unwrap();
    }

    info!("Guild {} recognized and loaded.", guild_name);
}
