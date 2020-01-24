use log::error;

use rusqlite::Connection;
use rusqlite::NO_PARAMS;

use serenity::model::prelude::GuildId;

use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn create_database() {
    let database = Path::new("database.sqlite3");
    if !database.exists() {
        match File::create(&database) {
            Ok(_) => (),
            Err(e) => error!("Failed to create database file: {}", e),
        }
    }
    if let Ok(connection) = Connection::open(&database) {
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS guild_settings (
                guild_id TEXT PRIMARY KEY,
                guild_name TEXT NOT NULL,
                prefix TEXT NOT NULL
            );",
            NO_PARAMS,
        ) {
            Ok(_) => (),
            Err(e) => {
                error!("{}", e);
            }
        }
    } else {
        error!("Could not open connection to database ({})", &database.to_string_lossy());
    }
}

pub fn get_database() -> Result<Connection, Box<dyn Error>> {
    let db = Path::new("database.sqlite3");
    Ok(Connection::open(db)?)
}

pub fn get_prefix(guildid: &GuildId) -> Result<String, Box<dyn Error>> {
    let conn = get_database()?;
    let mut statement = conn.prepare("SELECT prefix FROM guild_settings WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guildid.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("Guild not found.")?.get(0)?)
}