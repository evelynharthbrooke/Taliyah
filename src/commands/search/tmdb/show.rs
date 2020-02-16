use crate::utilities::calculate_average_sum;

use chrono::prelude::*;

use humantime::format_duration;

use isolang::Language;

use itertools::Itertools;

use reqwest::blocking::Client;
use reqwest::redirect::Policy;

use serde::Deserialize;

use serenity::client::Context;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;

use serenity::model::prelude::Message;

use std::time::Duration;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<Result>,
}

#[derive(Debug, Deserialize)]
pub struct Result {
    pub id: i64,
}

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
    pub external_ids: ExternalId,
}

#[derive(Debug, Deserialize)]
pub struct CreatedBy {
    pub id: i64,
    pub credit_id: String,
    pub name: String,
    pub gender: Option<i64>,
    pub profile_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
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
    pub vote_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct NetworkOrStudio {
    pub name: String,
    pub id: i64,
    pub logo_path: Option<String>,
    pub origin_country: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Season {
    pub air_date: Option<NaiveDate>,
    pub episode_count: i64,
    pub id: i64,
    pub name: String,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub season_number: i64,
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
    pub id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct Credits {
    pub cast: Vec<CastMember>,
    pub crew: Vec<CrewMember>,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CastMember {
    pub character: String,
    pub credit_id: String,
    pub id: i64,
    pub name: String,
    pub gender: i64,
    pub profile_path: Option<String>,
    pub order: i64,
}

#[derive(Debug, Deserialize)]
pub struct CrewMember {
    pub credit_id: String,
    pub department: String,
    pub id: i64,
    pub name: String,
    pub gender: Option<i64>,
    pub job: String,
    pub profile_path: Option<String>,
}

#[command]
#[aliases("show", "series")]
#[description("Gets detailed information about a TV series from The Movie Database.")]
pub fn show(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: Invalid series name provided.");
                embed.description("You have provided an invalid series name. Please try again.");
                embed.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    message.channel_id.broadcast_typing(&context)?;

    let show: String = arguments.rest().to_string();

    let api_key = crate::config::tmdb_key().expect("Could not find API key for The Movie Database...").to_owned();
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), ", v", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).redirect(Policy::none()).build()?;

    let search_endpoint = "https://api.themoviedb.org/3/search/tv";
    let search_query = ("query", &show.replace(" --cast", "").replace(" -c", ""));
    let search_response = client.get(search_endpoint).query(&[("api_key", &api_key), search_query]);
    let search_result: SearchResponse = search_response.send()?.json()?;
    let search_results = search_result.results;

    if search_results.is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.content(format!(
                "Sorry, I was unable to find a TV show on TMDb matching the term `{}`. \
                Please try a different search term.",
                show
            ))
        })?;
        return Ok(());
    }

    let show_id = search_results.first().unwrap().id;

    let show_endpoint = format!("https://api.themoviedb.org/3/tv/{}", show_id);
    let show_sub_requests = ("append_to_response", &"external_ids".to_string());
    let show_response = client.get(&show_endpoint).query(&[("api_key", &api_key), show_sub_requests]).send()?;
    let show_result: Show = show_response.json()?;
    let show_poster_path = show_result.poster_path.unwrap();
    let show_poster = format!("https://image.tmdb.org/t/p/original/{}", &show_poster_path.replace("/", ""));

    if show.contains(" --cast") || show.contains(" -c") {
        let credits_endpoint = format!("https://api.themoviedb.org/3/tv/{}/credits", show_id);
        let credits_response = client.get(&credits_endpoint).query(&[("api_key", &api_key)]).send()?;
        let credits_result: Credits = credits_response.json()?;

        let show_name = show_result.name;
        let show_cast = credits_result.cast;
        let show_crew = credits_result.crew;
        let mut show_cast_fields = Vec::with_capacity(show_cast.len());
        let mut show_crew_fields = Vec::with_capacity(show_crew.len());

        match show_cast.len() | show_crew.len() {
            5 | 8 | 11 | 14 | 17 | 20 | 23 => {
                // Iterate through each element of the show cast and crew, removing the
                // last element. This is done because Discord's embeds are really finicky
                // when there are less than 3 embed fields and more than 1 on each row.
                //
                // Honestly, I'm surprised Discord hasn't fixed that bug by now, but
                // for now, let's do a really hacky thing where we remove the last
                // element of the crew and cast vectors and later push it to the
                // show cast and crew vector fields separately. This helps the
                // embed look better on desktop. This however does not affect
                // Discord mobile, because all fields are non-inline by default
                // with no way to change it.
                for member in &show_cast[0..show_cast.len() - 1] {
                    show_cast_fields.push((&member.name, &member.character, true));
                }

                for member in &show_crew[0..show_cast.len() - 1] {
                    show_crew_fields.push((&member.name, &member.job, true));
                }

                let last_cast_member = show_cast.last().unwrap();
                show_cast_fields.push((&last_cast_member.name, &last_cast_member.character, false));
                let last_crew_member = show_crew.last().unwrap();
                show_crew_fields.push((&last_crew_member.name, &last_crew_member.job, false));
            }
            _ => {
                for member in &show_cast {
                    show_cast_fields.push((&member.name, &member.character, true));
                }

                for member in &show_crew {
                    show_crew_fields.push((&member.name, &member.job, true));
                }
            }
        }

        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title(format!("Cast & Crew — {}", show_name));
                embed.color(0x0001_d277);
                embed.thumbnail(show_poster);
                embed.fields(show_cast_fields);

                if !show_crew_fields.is_empty() {
                    embed.fields(show_crew_fields);
                }

                embed.footer(|footer| footer.text("Powered by the The Movie Database API."));
                embed.timestamp(&Utc::now())
            })
        })?;

        return Ok(());
    }

    let show_title = show_result.name;
    let show_id = show_result.id;
    let show_url = format!("https://themoviedb.org/tv/{}", &show_id);
    let show_status = show_result.status;
    let show_type = show_result.series_type;
    let show_average_runtime = calculate_average_sum(&show_result.episode_run_time);
    let show_runtime = format_duration(Duration::from_secs(show_average_runtime as u64 * 60)).to_string();
    let show_overview = show_result.overview;
    let show_popularity = show_result.popularity.to_string();
    let show_production_status = if show_result.in_production { "In Production" } else { "Finished Production" };
    let show_networks = show_result.networks.iter().map(|network| &network.name).join("\n");
    let show_seasons = show_result.number_of_seasons.to_string();
    let show_episodes = show_result.number_of_episodes.to_string();
    let show_imdb_id = show_result.external_ids.imdb_id.unwrap();
    let show_imdb_url = format!("https://www.imdb.com/title/{}", show_imdb_id);
    let show_external_links = format!("[IMDb]({})", show_imdb_url);
    let show_genres = show_result.genres.iter().map(|genre| &genre.name).join("\n");
    let show_language = Language::from_639_1(&show_result.original_language).unwrap().to_name().to_string();
    let show_languages = show_result.languages.iter().map(|l| Language::from_639_1(&l).unwrap().to_name().to_string()).join("\n");
    let show_creators = show_result.created_by.iter().map(|creator| &creator.name).join("\n");
    let show_user_score = show_result.vote_average * 10.0;
    let show_user_score_count = show_result.vote_count;
    let show_initial_air_date = show_result.first_air_date.format("%B %-e, %Y").to_string();
    let show_final_air_date = show_result.last_air_date.format("%B %-e, %Y").to_string();

    let show_studios = if show_result.studios.is_empty() {
        "N/A".to_string()
    } else {
        show_result.studios.iter().map(|s| &s.name).join("\n")
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title(show_title);
            embed.url(show_url);
            embed.thumbnail(show_poster);
            embed.color(0x0001_d277);
            embed.description(show_overview);
            embed.fields(vec![
                ("Status", show_status, true),
                ("Type", show_type, true),
                ("Created By", show_creators, true),
                ("Runtime", show_runtime, true),
                ("First Air Date", show_initial_air_date, true),
                ("Last Air Date", show_final_air_date, true),
                ("Main Language", show_language, true),
                ("Languages", show_languages, true),
                ("Popularity", format!("{}%", show_popularity), true),
                ("User Score", format!("{}/100 ({} votes)", show_user_score, show_user_score_count), true),
                ("Episodes", show_episodes, true),
                ("Seasons", show_seasons, true),
                ("Networks / Services", show_networks, true),
                ("Studios", show_studios, true),
                ("Genres", show_genres, true),
                ("Production Status", show_production_status.to_string(), true),
                ("External Links", show_external_links, false),
            ]);
            embed.footer(|footer| footer.text("Powered by the The Movie Database API."));
            embed.timestamp(&Utc::now())
        })
    })?;

    Ok(())
}
