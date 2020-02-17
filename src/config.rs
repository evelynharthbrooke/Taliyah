//! Provides functions for accessing configuration values.
//! These are loaded from a toml file on first request during runtime.
//!
//! To add more configuration options:
//!
//!  - Add field to `Config`
//!
//!  - Add an accessor function with the same name as the field
//!    Copy an existing one like `bot_codename()` to make life easier
//!
//!  - Add field to `test::generate_expected_config()` and set to `None`
//!
//!  - Add another call to `test::check_field()` in `test::getters()`
//!    referencing the field
//!
//!  - Add a line containing the field name in "config.toml.sample"
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

pub const CONFIG_FILE: &str = "config.toml";

macro_rules! config_str_convert {
    ($field:ident) => {
        Some(&CONFIG.$field.as_ref()?)
    };
}

/// Try to get `bot_codename` from loaded config
#[allow(dead_code)]
pub fn bot_codename() -> Option<&'static str> {
    config_str_convert!(bot_codename)
}

/// Try to get `client_id` from loaded config
#[allow(dead_code)]
pub fn client_id() -> Option<&'static str> {
    config_str_convert!(client_id)
}

/// Try to get `client_secret` from loaded config
#[allow(dead_code)]
pub fn client_secret() -> Option<&'static str> {
    config_str_convert!(client_secret)
}

/// Try to get `darksky` from loaded config
#[allow(dead_code)]
pub fn darksky() -> Option<&'static str> {
    config_str_convert!(darksky)
}

/// Try to get `discord_token` from loaded config
#[allow(dead_code)]
pub fn discord_token() -> Option<&'static str> {
    config_str_convert!(discord_token)
}

/// Try to get `discord_prefix` from loaded config
#[allow(dead_code)]
pub fn discord_prefix() -> Option<&'static str> {
    config_str_convert!(discord_prefix)
}

/// Try to get `github_key` from loaded config
#[allow(dead_code)]
pub fn github_key() -> Option<&'static str> {
    config_str_convert!(github_key)
}

/// Try to get `google_key` from loaded config
#[allow(dead_code)]
pub fn google_key() -> Option<&'static str> {
    config_str_convert!(google_key)
}

/// Try to get `lastfm_key` from loaded config
#[allow(dead_code)]
pub fn lastfm_key() -> Option<&'static str> {
    config_str_convert!(lastfm_key)
}

/// Try to get `rust_log` from loaded config
#[allow(dead_code)]
pub fn tmdb_key() -> Option<&'static str> {
    config_str_convert!(tmdb_key)
}

/// Try to get `rust_log` from loaded config
#[allow(dead_code)]
pub fn rust_log() -> Option<&'static str> {
    config_str_convert!(rust_log)
}

/// Preload cofing and load neccessary config variables into the
/// application environment.
pub fn init() {
    // When specified in config file, set log level
    if let Some(level) = rust_log() {
        env::set_var("RUST_LOG", level);
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::load_from_file(CONFIG_FILE);
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub bot_codename: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub darksky: Option<String>,
    pub discord_token: Option<String>,
    pub discord_prefix: Option<String>,
    pub github_key: Option<String>,
    pub google_key: Option<String>,
    pub lastfm_key: Option<String>,
    pub tmdb_key: Option<String>,
    pub rust_log: Option<String>,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Config {
        let error_msg = "Unable to read config.toml";
        let data = fs::read_to_string(path).expect(error_msg);
        let mut config: Config = toml::from_str(&data).expect(error_msg);

        if config.bot_codename.is_none() {
            config.bot_codename = Some("Ellie".to_owned());
        }

        config
    }
}
