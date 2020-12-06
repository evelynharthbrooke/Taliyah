use crate::{
    models::tmdb::show::*,
    utils::{calculate_average_sum, locale_utils, read_config}
};

use humantime::format_duration;
use itertools::Itertools;
use reqwest::{redirect::Policy, Client};
use serde::Deserialize;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::{env, time::Duration};

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
pub async fn show(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message
            .channel_id
            .send_message(&context, |message| {
                message.embed(|embed| {
                    embed.title("Error: Invalid series name provided.");
                    embed.description("You have provided an invalid series name. Please try again.");
                    embed.color(0x00FF_0000)
                })
            })
            .await
            .unwrap();
        return Ok(());
    }

    let show: String = arguments.rest().to_string();

    let config = read_config(&env::var("ELLIE_CONFIG_FILE").unwrap());

    let api_key = config.api.entertainment.tmdb;
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), ", v", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).redirect(Policy::none()).build().unwrap();

    let search_endpoint = "https://api.themoviedb.org/3/search/tv";
    let search_query = ("query", &show.replace(" --cast", "").replace(" -c", ""));
    let search_response = client.get(search_endpoint).query(&[("api_key", &api_key), search_query]);
    let search_result: SearchResponse = search_response.send().await.unwrap().json().await.unwrap();
    let search_results = search_result.results;

    if search_results.is_empty() {
        message
            .channel_id
            .send_message(&context, |message| {
                message.content(format!("Unable to find a show on TMdb matching the term {}. Try a different name.", show))
            })
            .await
            .unwrap();
        return Ok(());
    }

    let show_id = search_results.first().unwrap().id;

    let show_endpoint = format!("https://api.themoviedb.org/3/tv/{}", show_id);
    let show_sub_requests = ("append_to_response", &"external_ids".to_string());
    let show_response = client.get(&show_endpoint).query(&[("api_key", &api_key), show_sub_requests]).send().await.unwrap();
    let show_result: Show = show_response.json().await.unwrap();
    let show_poster_path = show_result.poster_path.unwrap();
    let show_poster = format!("https://image.tmdb.org/t/p/original/{}", &show_poster_path.replace("/", ""));

    let show_title = show_result.name;
    let show_id = show_result.id;
    let show_url = format!("https://themoviedb.org/tv/{}", &show_id);
    let show_status = show_result.status;
    let show_type = show_result.show_type;
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
    let show_tagline = if !show_result.tagline.is_empty() {
        format!("*{}*", show_result.tagline)
    } else {
        "".to_string()
    };
    let show_language = locale_utils::get_language_name_from_iso(&show_result.original_language).to_string();
    let show_languages = show_result.languages.iter().map(|l| locale_utils::get_language_name_from_iso(&l).to_string()).join("\n");
    let show_origin_country = show_result.origin_country.iter().map(|c| locale_utils::get_country_name_from_iso(&c).to_string()).join("\n");
    let show_creators = if show_result.created_by.is_empty() {
        "Unknown".to_string()
    } else {
        show_result.created_by.iter().map(|creator| &creator.name).join("\n")
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

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.title(show_title);
                embed.url(show_url);
                embed.thumbnail(show_poster);
                embed.color(0x01b4e4);
                embed.description(show_tagline);
                embed.fields(vec![
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
                    ("Popularity", format!("{}%", show_popularity), true),
                    ("User Score", format!("{}/100 ({} votes)", show_user_score, show_user_score_count), true),
                    ("Seasons", show_seasons, true),
                    ("Episodes", show_episodes, true),
                    ("Networks / Services", show_networks, true),
                    ("Genres", show_genres, true),
                    ("Studios", show_studios, false),
                    ("Production Status", show_production_status.to_string(), true),
                    ("External Links", show_external_links, false),
                ]);
                embed.footer(|footer| footer.text("Powered by the The Movie Database API."))
            })
        })
        .await
        .unwrap();

    Ok(())
}
