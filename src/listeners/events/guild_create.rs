use crate::config::Config;
use crate::utilities::database::get_database;

use log::info;

use serenity::client::Context;
use serenity::model::guild::Guild;

pub fn guild_create(_context: Context, guild: Guild) {
    let config = Config::load_from_file("config.toml");

    let connection = match get_database() {
        Ok(connection) => connection,
        Err(_) => return
    };

    let prefix = config.discord_prefix.unwrap_or("e.".to_string());

    let guild_name = guild.name;
    let guild_id = guild.id.as_u64().to_string();

    info!("Guild {} recognized and loaded.", guild_name);
    let _ = connection.execute(
        "INSERT OR IGNORE INTO guild_settings (guild_id, guild_name, prefix) values (?1, ?2, ?3)",
        &[&guild_id, &guild_name, &prefix]
    );
}
