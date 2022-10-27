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

    let show_id = search_results.first().unwrap().id;
    let show_endpoint = format!("https://api.themoviedb.org/3/tv/{show_id}");
    let show_sub_requests = ("append_to_response", &"external_ids".to_string());
    let show_response = client.get(&show_endpoint).query(&[("api_key", &api_key), show_sub_requests]).send().await.unwrap();
    let show_result: Show = show_response.json().await.unwrap();
    let show_poster_path = show_result.poster_path.unwrap();
    let show_poster = format!("https://image.tmdb.org/t/p/original/{}", &show_poster_path.replace('/', ""));

    let show_title = show_result.name;
    let show_url = format!("https://themoviedb.org/tv/{}", &show_id);
    let show_status = show_result.status;
    let show_type = show_result.show_type;
    let show_average_runtime = calculate_average_sum(&show_result.episode_run_time);
    let show_runtime = format_duration(Duration::from_secs(show_average_runtime as u64 * 60)).to_string();
    let show_overview = show_result.overview;
    let show_popularity = show_result.popularity.to_string();
    let show_production_status = if show_result.in_production { "In Production" } else { "Finished Production" };
    let show_networks = show_result.networks.iter().map(|n| &n.name).join("\n");
    let show_seasons = show_result.number_of_seasons.to_string();
    let show_episodes = show_result.number_of_episodes.to_string();
    let show_imdb_id = show_result.external_ids.imdb_id.unwrap();
    let show_imdb_url = format!("https://www.imdb.com/title/{show_imdb_id}");
    let show_genres = show_result.genres.iter().map(|genre| &genre.name).join("\n");
    let show_tagline = if !show_result.tagline.is_empty() {
        format!("*{}*", show_result.tagline)
    } else {
        String::new()
    };

    let show_language = locale::get_language_name_from_iso(&show_result.original_language).to_string();
    let show_languages = show_result.languages.iter().map(|l| locale::get_language_name_from_iso(l)).join("\n");
    let show_origin_country = show_result.origin_country.iter().map(|c| locale::get_country_name_from_iso(c)).join("\n");

    let show_creators = if show_result.created_by.is_empty() {
        "Unknown".to_string()
    } else {
        show_result.created_by.iter().map(|c| &c.name).join("\n")
    };

    let show_user_score = show_result.vote_average * 10.0;
    let show_user_score_count = show_result.vote_count;
    let show_first_air_date = show_result.first_air_date.format("%B %-e, %Y").to_string();
    let show_last_air_date = show_result.last_air_date.format("%B %-e, %Y").to_string();

    let show_studios = if show_result.studios.is_empty() {
        "Unknown".to_string()
    } else {
        show_result.studios.iter().map(|s| &s.name).join("\n")
    };

    let show_fields = vec![
        ("Overview", show_overview, false),
        ("Status", show_status, true),
        ("Type", show_type, true),
        ("Created By", show_creators, true),
        ("Runtime", show_runtime, true),
        ("First Air Date", show_first_air_date, true),
        ("Last Air Date", show_last_air_date, true),
        ("Main Language", show_language, true),
        ("Origin Countries", show_origin_country, true),
        ("Languages", show_languages, true),
        ("Popularity", format!("{show_popularity}%"), true),
        ("User Score", format!("{show_user_score}/100 ({show_user_score_count} votes)"), true),
        ("Seasons", show_seasons, true),
        ("Episodes", show_episodes, true),
        ("Networks / Services", show_networks, true),
        ("Genres", show_genres, true),
        ("Studios", show_studios, false),
        ("Production Status", show_production_status.to_string(), true),
    ];

    let embed = CreateEmbed::new()
        .title(show_title)
        .url(show_url)
        .thumbnail(show_poster)
        .color(0x01b4e4)
        .description(show_tagline)
        .fields(show_fields)
        .footer(CreateEmbedFooter::new("Powered by TMDB."));

    let links = CreateActionRow::Buttons(vec![(CreateButton::new_link("View IMDb Page", show_imdb_url))]);
    message.channel_id.send_message(&context, CreateMessage::new().embed(embed).components(vec![links])).await?;

    Ok(())
}
