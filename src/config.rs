use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigurationData {
    pub bot: BotConfig,
    pub api: ApiConfig
}

#[derive(Deserialize)]
pub struct BotConfig {
    pub general: GeneralConfig,
    pub database: DatabaseConfig,
    pub discord: DiscordConfig,
    pub denylist: DenylistConfig,
    pub logging: LoggingConfig
}

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub codename: String,
    pub prefix: String
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub enabled: bool,
    pub level: String
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String
}

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub appid: u64,
    pub token: String
}

#[derive(Deserialize)]
pub struct DenylistConfig {
    pub spotify: DenylistSpotifyConfig
}

#[derive(Deserialize)]
pub struct DenylistSpotifyConfig {
    pub ids: Vec<u64>
}

#[derive(Deserialize)]
pub struct ApiConfig {
    pub entertainment: EntertainmentConfig,
    pub minecraft: MinecraftConfig,
    pub music: MusicConfig,
    pub services: ServicesConfig,
    pub social: SocialConfig
}

#[derive(Deserialize)]
pub struct EntertainmentConfig {
    pub tmdb: String
}

#[derive(Deserialize)]
pub struct MinecraftConfig {
    pub hypixel: String
}

#[derive(Deserialize)]
pub struct MusicConfig {
    pub spotify: SpotifyConfig,
    pub lastfm: LastFmConfig,
    pub lavalink: LavalinkConfig
}

#[derive(Deserialize)]
pub struct ServicesConfig {
    pub github: String,
    pub google: String
}

#[derive(Deserialize)]
pub struct SocialConfig {
    pub twitter: TwitterConfig
}

#[derive(Deserialize)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String
}

#[derive(Deserialize)]
pub struct LastFmConfig {
    pub api_key: String
}

#[derive(Deserialize)]
pub struct LavalinkConfig {
    pub host: String,
    pub port: u16,
    pub password: String
}

#[derive(Deserialize)]
pub struct TwitterConfig {
    pub core: TwitterCore,
    pub client: TwitterClient
}

#[derive(Deserialize)]
pub struct TwitterCore {
    pub api_key: String,
    pub api_key_secret: String,
    pub bearer_token: String,
    pub access_token: String,
    pub access_token_secret: String
}

#[derive(Deserialize)]
pub struct TwitterClient {
    pub client_id: String,
    pub client_secret: String
}
