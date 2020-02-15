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

const CONFIG_FILE: &str = "config.toml";

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
    static ref CONFIG: Config = Config::load_from_file(CONFIG_FILE);
}

#[derive(Deserialize, Debug, PartialEq)]
struct Config {
    bot_codename: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    darksky: Option<String>,
    discord_token: Option<String>,
    discord_prefix: Option<String>,
    github_key: Option<String>,
    google_key: Option<String>,
    lastfm_key: Option<String>,
    tmdb_key: Option<String>,
    rust_log: Option<String>,
}

impl Config {
    fn load_from_file<P: AsRef<Path>>(path: P) -> Config {
        let error_msg = "Unable to read config.toml";
        let data = fs::read_to_string(path).expect(error_msg);
        let mut config: Config = toml::from_str(&data).expect(error_msg);

        if config.bot_codename.is_none() {
            config.bot_codename = Some("Ellie".to_owned());
        }

        config
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_CONFIG_FILE_NAME: &str = "testing-config.toml";
    const BACKUP_CONFIG_FILE_NAME: &str = "config.toml.bak";
    const CONFIG_TOML: &str = r#"
        bot_codename = "TestName"
        client_id = "123456789"
        client_secret = "TestClientSecret"
        # darksky
        discord_token = "TestDiscordToken"
        discord_prefix = "TestPrefix"
        github_key = "TestGitHubKey"
        # google_key
        # tmdb_key
        lastfm_key = "TestLastFmKey"
    "#;

    fn generate_expected_config() -> Config {
        Config {
            bot_codename: Some("TestName".to_owned()),
            client_id: Some("123456789".to_owned()),
            client_secret: Some("TestClientSecret".to_owned()),
            darksky: None,
            discord_token: Some("TestDiscordToken".to_owned()),
            discord_prefix: Some("TestPrefix".to_owned()),
            github_key: Some("TestGitHubKey".to_owned()),
            google_key: None,
            lastfm_key: Some("TestLastFmKey".to_owned()),
            tmdb_key: None,
            rust_log: None,
        }
    }

    #[test]
    fn deseralize_toml() {
        let expected_config = generate_expected_config();
        let test_config: Config = toml::from_str(CONFIG_TOML).expect("Unable to deserialize toml");
        assert_eq!(expected_config, test_config);
    }

    #[test]
    #[should_panic]
    fn invalid_toml() {
        // missing double-quote
        let bad_toml = r#"
            botcodename = "TestName
        "#;
        let _: Config = toml::from_str(bad_toml).unwrap();
    }

    #[test]
    fn load_from_file() {
        let expected_config = generate_expected_config();
        create_test_file(TEST_CONFIG_FILE_NAME);
        let test_config = Config::load_from_file(TEST_CONFIG_FILE_NAME);
        remove_test_file(TEST_CONFIG_FILE_NAME);
        assert_eq!(expected_config, test_config);
    }

    #[test]
    #[should_panic]
    fn load_from_file_error() {
        Config::load_from_file("invalid.toml");
    }

    #[test]
    fn getters() {
        temp_move_config_file();
        create_test_file(CONFIG_FILE);

        let expected_config = Config::load_from_file(CONFIG_FILE);

        check_field(expected_config.bot_codename, bot_codename());
        check_field(expected_config.client_id, client_id());
        check_field(expected_config.client_secret, client_secret());
        check_field(expected_config.darksky, darksky());
        check_field(expected_config.discord_token, discord_token());
        check_field(expected_config.discord_prefix, discord_prefix());
        check_field(expected_config.github_key, github_key());
        check_field(expected_config.google_key, google_key());
        check_field(expected_config.lastfm_key, lastfm_key());
        check_field(expected_config.tmdb_key, tmdb_key());
        check_field(expected_config.rust_log, rust_log());

        remove_test_file(CONFIG_FILE);
        move_back_config_file();
    }

    fn check_field(field: Option<String>, res: Option<&'static str>) {
        let equal = match field {
            Some(data) => {
                if let Some(res_data) = res {
                    res_data == data
                } else {
                    false
                }
            }
            None => res.is_none(),
        };
        if !equal {
            remove_test_file(CONFIG_FILE);
            move_back_config_file();
            panic!("Getter does not match fields");
        }
    }

    fn temp_move_config_file() {
        if Path::new(CONFIG_FILE).exists() {
            fs::rename(CONFIG_FILE, BACKUP_CONFIG_FILE_NAME).expect("Unable to backup config file before testing");
        }
    }

    fn move_back_config_file() {
        if Path::new(BACKUP_CONFIG_FILE_NAME).exists() {
            fs::rename(BACKUP_CONFIG_FILE_NAME, CONFIG_FILE).expect("Unable to restore backup of config file");
        }
    }

    fn create_test_file<P: AsRef<Path>>(test_file_name: P) {
        fs::write(test_file_name, CONFIG_TOML).expect("Unable to create testing config file");
    }

    fn remove_test_file<P: AsRef<Path>>(test_file_name: P) {
        fs::remove_file(test_file_name).expect("Unable to remove testing config file");
    }
}
