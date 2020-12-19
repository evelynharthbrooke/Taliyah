use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: ApiConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
    pub logging: LoggingConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneralConfig {
    pub codename: String,
    pub prefix: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub token: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiConfig {
    pub music: MusicConfig,
    pub entertainment: EntertainmentConfig,
    pub social: SocialConfig,
    pub services: ServicesConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MusicConfig {
    pub spotify: SpotifyConfig,
    pub lastfm: LastFmConfig,
    pub lavalink: LavalinkConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LastFmConfig {
    pub api_key: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LavalinkConfig {
    pub host: String,
    pub port: u16,
    pub password: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EntertainmentConfig {
    pub tmdb: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SocialConfig {
    pub twitter: TwitterConfig
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TwitterConfig {
    pub api_key: String,
    pub api_key_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
    pub bearer_token: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServicesConfig {
    pub github: String,
    pub google: String
}
