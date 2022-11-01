use humantime::format_duration;
use itertools::Itertools;
use serde::Deserialize;

use serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::time::Duration;

use crate::{
    data::ReqwestContainer,
    models::tmdb::show::*,
    utils::{calculate_average_sum, locale, read_config}
};

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<Result>
}

#[derive(Debug, Deserialize)]
pub struct Result {
    pub id: i64
}

#[command]
#[aliases("show", "series")]
#[description("Gets detailed information about a TV series from The Movie Database.")]
async fn show(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "Invalid show name provided. Please try again.").await?;
        return Ok(());
    }

    let show: String = arguments.rest().to_string();

    let config = read_config("config.toml");
    let api_key = config.api.entertainment.tmdb;
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();

    let search_endpoint = "https://api.themoviedb.org/3/search/tv";
    let search_query = ("query", &show.replace(" --cast", "").replace(" -c", ""));
    let search_response = client.get(search_endpoint).query(&[("api_key", &api_key), search_query]);
    let search_result: SearchResponse = search_response.send().await.unwrap().json().await.unwrap();
    let search_results = search_result.results;
    if search_results.is_empty() {
        message.channel_id.say(context, format!("Nothing found for `{show}`. Please try a different name.")).await?;
        return Ok(());
    }

    let id = search_results.first().unwrap().id;
    let endpoint = format!("https://api.themoviedb.org/3/tv/{id}");
    let sub_requests = ("append_to_response", &"external_ids".to_string());
    let response = client.get(&endpoint).query(&[("api_key", &api_key), sub_requests]).send().await.unwrap();
    let result: Show = response.json().await.unwrap();
    let poster_path = result.poster_path.unwrap();
    let poster = format!("https://image.tmdb.org/t/p/original/{}", &poster_path.replace('/', ""));

    let title = result.name;
    let url = format!("https://themoviedb.org/tv/{}", &id);
    let status = result.status;
    let format = result.format;
    let average_runtime = calculate_average_sum(&result.episode_run_time);
    let runtime = format_duration(Duration::from_secs(average_runtime as u64 * 60)).to_string();
    let overview = result.overview;
    let popularity = result.popularity.to_string();
    let production_status = if result.in_production { "In Production" } else { "Finished Production" };
    let networks = result.networks.iter().map(|n| &n.name).join("\n");
    let seasons = result.number_of_seasons.to_string();
    let episodes = result.number_of_episodes.to_string();
    let imdb_id = result.external_ids.imdb_id.unwrap();
    let imdb_url = format!("https://www.imdb.com/title/{imdb_id}");
    let genres = result.genres.iter().map(|genre| &genre.name).join("\n");
    let tagline = if !result.tagline.is_empty() {
        format!("*{}*", result.tagline)
    } else {
        String::new()
    };

    let language = locale::get_language_name_from_iso(&result.original_language).to_string();
    let languages = result.languages.iter().map(|l| locale::get_language_name_from_iso(l)).join("\n");
    let origin_country = result.origin_country.iter().map(|c| locale::get_country_name_from_iso(c)).join("\n");

    let creators = if result.created_by.is_empty() {
        "Unknown".to_string()
    } else {
        result.created_by.iter().map(|c| &c.name).join("\n")
    };

    let user_score = result.vote_average * 10.0;
    let user_score_count = result.vote_count;
    let first_air_date = result.first_air_date.format("%B %-e, %Y").to_string();
    let last_air_date = result.last_air_date.format("%B %-e, %Y").to_string();

    let studios = if result.studios.is_empty() {
        "Unknown".to_string()
    } else {
        result.studios.iter().map(|s| &s.name).join("\n")
    };

    let fields = vec![
        ("Overview", overview, false),
        ("Status", status, true),
        ("Format", format, true),
        ("Created By", creators, true),
        ("Runtime", runtime, true),
        ("First Air Date", first_air_date, true),
        ("Last Air Date", last_air_date, true),
        ("Main Language", language, true),
        ("Origin Countries", origin_country, true),
        ("Languages", languages, true),
        ("Popularity", format!("{popularity}%"), true),
        ("User Score", format!("{user_score}/100 ({user_score_count} votes)"), true),
        ("Seasons", seasons, true),
        ("Episodes", episodes, true),
        ("Networks / Services", networks, true),
        ("Genres", genres, true),
        ("Studios", studios, false),
        ("Production Status", production_status.to_string(), true),
    ];

    let embed = CreateEmbed::new()
        .title(title)
        .url(url)
        .thumbnail(poster)
        .color(0x01b4e4)
        .description(tagline)
        .fields(fields)
        .footer(CreateEmbedFooter::new("Powered by TMDb."));

    let links = CreateActionRow::Buttons(vec![(CreateButton::new_link("View IMDb Page", imdb_url))]);
    message.channel_id.send_message(&context, CreateMessage::new().embed(embed).components(vec![links])).await?;

    Ok(())
}
