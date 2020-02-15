use lazy_static::*;
use serde::Deserialize;

lazy_static! {
    static ref CONFIG: Config = Config::load_from_file("config.toml");
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
    rust_log: Option<String>,
}

impl Config {
    fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Config {
        let error_msg = "Unable to read config.toml";
        let data = std::fs::read_to_string(path).expect(error_msg);
        toml::from_str(&data).expect(error_msg)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_CONFIG_FILE_NAME: &str = "testing-config.toml";
    const CONFIG_TOML: &str = r#"
        bot_codename = "TestName"
        client_id = "123456789"
        client_secret = "TestClientSecret"
        # darksky
        discord_token = "TestDiscordToken"
        discord_prefix = "TestPrefix"
        github_key = "TestGitHubKey"
        # google_key
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
        create_test_file();
        let test_config = Config::load_from_file(TEST_CONFIG_FILE_NAME);
        remove_test_file();
        assert_eq!(expected_config, test_config);
    }

    #[test]
    #[should_panic]
    fn load_from_file_error() {
        Config::load_from_file("invalid.toml");
    }

    fn create_test_file() {
        std::fs::write(TEST_CONFIG_FILE_NAME, CONFIG_TOML).expect("Unable to create testing config file");
    }

    fn remove_test_file() {
        std::fs::remove_file(TEST_CONFIG_FILE_NAME).expect("Unable to remove testing config file");
    }
}
