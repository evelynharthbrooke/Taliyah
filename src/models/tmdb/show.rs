use chrono::prelude::NaiveDate;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Show {
    pub backdrop_path: Option<String>,
    pub created_by: Vec<CreatedBy>,
    pub episode_run_time: Vec<i64>,
    pub first_air_date: NaiveDate,
    pub genres: Vec<Genre>,
    pub homepage: String,
    pub id: i64,
    pub in_production: bool,
    pub languages: Vec<String>,
    pub last_air_date: NaiveDate,
    pub last_episode_to_air: TEpisodeToAir,
    pub name: String,
    pub next_episode_to_air: Option<TEpisodeToAir>,
    pub networks: Vec<NetworkOrStudio>,
    pub number_of_episodes: i64,
    pub number_of_seasons: i64,
    pub origin_country: Vec<String>,
    pub original_language: String,
    pub original_name: String,
    pub overview: String,
    pub popularity: f64,
    pub poster_path: Option<String>,
    #[serde(rename = "production_companies")]
    pub studios: Vec<NetworkOrStudio>,
    pub seasons: Vec<Season>,
    pub status: String,
    #[serde(rename = "type")]
    pub series_type: String,
    pub vote_average: f64,
    pub vote_count: i64,
    pub external_ids: ExternalId
}

#[derive(Debug, Deserialize)]
pub struct CreatedBy {
    pub id: i64,
    pub credit_id: String,
    pub name: String,
    pub gender: Option<i64>,
    pub profile_path: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct TEpisodeToAir {
    pub air_date: Option<NaiveDate>,
    pub episode_number: i64,
    pub id: i64,
    pub name: String,
    pub overview: String,
    pub production_code: String,
    pub season_number: i64,
    pub show_id: i64,
    pub still_path: Option<String>,
    pub vote_average: f64,
    pub vote_count: i64
}

#[derive(Debug, Deserialize)]
pub struct NetworkOrStudio {
    pub name: String,
    pub id: i64,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Season {
    pub air_date: Option<NaiveDate>,
    pub episode_count: i64,
    pub id: i64,
    pub name: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: i64
}

#[derive(Debug, Deserialize)]
pub struct ExternalId {
    pub imdb_id: Option<String>,
    pub freebase_mid: Option<String>,
    pub freebase_id: Option<String>,
    pub tvdb_id: Option<i64>,
    pub tvrage_id: Option<i64>,
    pub facebook_id: Option<String>,
    pub instagram_id: Option<String>,
    pub twitter_id: Option<String>,
    pub id: Option<i64>
}
