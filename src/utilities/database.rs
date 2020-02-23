use log::error;

use rusqlite::Connection;
use rusqlite::NO_PARAMS;

use serenity::model::prelude::GuildId;
use serenity::model::prelude::UserId;

use std::error::Error;
use std::fs::File;
use std::path::Path;

pub fn create_database() {
    let database = Path::new("database.sqlite3");

    if !database.exists() {
        match File::create(&database) {
            Ok(_) => (),
            Err(e) => error!("Failed to create database file: {}", e)
        }
    }

    if let Ok(connection) = Connection::open(&database) {
        // Set user_version to 1.
        match connection.execute("PRAGMA user_version = 1;", NO_PARAMS) {
            Ok(_) => (),
            Err(e) => error!("{}", e)
        }

        // Create guild_settings table.
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS guild_settings (
                guild_id TEXT PRIMARY KEY,
                guild_name TEXT NOT NULL,
                prefix TEXT NOT NULL
            );",
            NO_PARAMS
        ) {
            Ok(_) => (),
            Err(e) => error!("{}", e)
        };

        // Create profile table.
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS profile (
                user_id TEXT PRIMARY KEY NOT NULL,
                user_tag TEXT NOT NULL,
                display_name TEXT,
                location TEXT,
                twitch TEXT,
                twitter TEXT,
                lastfm TEXT,
                steam TEXT,
                playstation TEXT,
                xbox TEXT
            )",
            NO_PARAMS
        ) {
            Ok(_) => (),
            Err(e) => error!("{}", e)
        }
    } else {
        error!("Could not open connection to database ({})", &database.to_string_lossy())
    };
}

pub fn get_database() -> Result<Connection, Box<dyn Error>> {
    let database_file = Path::new("database.sqlite3");
    Ok(Connection::open(database_file)?)
}

pub fn get_sqlite_version() -> String {
    let sqlite_version = rusqlite::version();
    sqlite_version.to_string()
}

pub fn get_user_display_name(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT display_name FROM profile WHERE user_id == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_user_twitch(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT twitch FROM profile where USER_ID == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_user_twitter(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT twitter FROM profile where USER_ID == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_user_location(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT location FROM profile WHERE user_id == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_user_lastfm(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT lastfm FROM profile WHERE user_id == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_user_steam(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT steam FROM profile where USER_ID == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_user_xbox_id(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT xbox FROM profile where USER_ID == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_user_playstation_id(user_id: UserId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT playstation FROM profile where USER_ID == ?1;")?;
    let mut rows = statement.query(&[&user_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("User not found in database")?.get(0)?)
}

pub fn get_prefix(guild_id: GuildId) -> Result<String, Box<dyn Error>> {
    let connection = get_database()?;
    let mut statement = connection.prepare("SELECT prefix FROM guild_settings WHERE guild_id == ?1;")?;
    let mut rows = statement.query(&[&guild_id.as_u64().to_string()])?;
    Ok(rows.next()?.ok_or("Guild not found.")?.get(0)?)
}

pub fn clear_prefix(guild_id: GuildId) {
    let connection = match get_database() {
        Ok(d) => d,
        Err(e) => return error!("An error occured while getting the database: {}", e)
    };
    let _ = connection.execute("DELETE FROM guild_settings where guild_id == ?1;", &[guild_id.to_string()]);
}
