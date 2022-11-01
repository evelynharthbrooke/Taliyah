use humantime::format_duration;
use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::Deserialize;

use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use std::time::Duration;

use crate::{
    data::ReqwestContainer,
    models::tmdb::movie::*,
    utils::{format_int, locale, read_config}
};

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchMovie>
}

#[derive(Debug, Deserialize)]
pub struct SearchMovie {
    pub id: u64 // The movie's ID from the search result, which is all we need.
}

#[command]
#[aliases("film")]
#[description("Gets detailed information about a movie from The Movie Database.")]
async fn movie(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "Invalid movie name provided. Please try again.").await?;
        return Ok(());
    }

    let mut movie: String = arguments.rest().to_string();

    let config = read_config("config.toml");
    let api_key = config.api.entertainment.tmdb;
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();

    let search_endpoint = "https://api.themoviedb.org/3/search/movie";
    let search_response: RequestBuilder;

    // This is a pretty hacky way of being able to search by year, but
    // surprisingly enough it actually works from what I've tested, and
    // while it might be a tad slow, it should compute fast enough to not
    // make users wonder why its taking so long for the response to send.
    //
    // This code follows the y: notation syntax as available on the website
    // for The Movie Database, with the additional ability to use year: in
    // place of y:, depending on the user's preference.
    if movie.contains("y:") || movie.contains("year:") {
        movie = movie.replace(" y:", "").replace(" year:", "");
        let mut year_rev: Vec<char> = movie.chars().rev().take(4).collect();
        year_rev.reverse();
        let year = year_rev.iter().join("");
        movie = movie.replace(&year, "");
        search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &movie), ("year", &year)]);
    } else {
        search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &movie)]);
    }

    let search_result: SearchResponse = search_response.send().await.unwrap().json().await.unwrap();
    let search_results = search_result.results;
    if search_results.is_empty() {
        message.channel_id.say(context, format!("Nothing found for `{movie}`. Please try again.")).await?;
        return Ok(());
    }

    let id = search_results.first().unwrap().id;
    let endpoint = format!("https://api.themoviedb.org/3/movie/{id}");
    let response = client.get(&endpoint).query(&[("api_key", &api_key)]).send().await.unwrap();
    let result: Movie = response.json().await.unwrap();

    let tagline = match result.tagline {
        Some(tagline) => {
            if tagline.is_empty() {
                String::new()
            } else {
                format!("*{tagline}*")
            }
        }
        None => String::new()
    };

    let overview = match result.overview {
        Some(overview) => {
            if !tagline.is_empty() {
                format!("\n\n{overview}")
            } else {
                overview
            }
        }
        None => String::new()
    };

    let studios = if result.production_companies.is_empty() {
        "No Known Studios".to_string()
    } else {
        result.production_companies.iter().map(|c| &c.name).join("\n")
    };

    let collection = match result.belongs_to_collection {
        Some(collection) => collection.name,
        None => "N/A".to_string()
    };

    let homepage = match result.homepage {
        Some(homepage) => {
            if homepage.is_empty() {
                "No Website".to_string()
            } else {
                format!("[Website]({homepage})")
            }
        }
        None => "No Website".to_string()
    };

    let id = result.id.to_string();
    let title = result.title.as_str();
    let status = result.status;
    let language = locale::get_language_name_from_iso(&result.original_language).to_string();
    let release_date = result.release_date.unwrap().format("%B %e, %Y").to_string();
    let budget = format_int(result.budget);
    let revenue = format_int(result.revenue);
    let imdb = format!("[IMDb](https://www.imdb.com/title/{})", result.imdb_id.unwrap());
    let url = format!("https://www.themoviedb.org/movie/{id}");
    let genres = result.genres.iter().map(|g| &g.name).join("\n");
    let popularity = format!("{}%", result.popularity);
    let poster_uri = result.poster_path.unwrap();
    let poster = format!("https://image.tmdb.org/t/p/original/{}", &poster_uri.replace('/', ""));
    let user_score = format!("{}/100", result.vote_average * 10.0);
    let user_score_count = result.vote_count;
    let runtime = format_duration(Duration::from_secs(result.runtime.unwrap() * 60)).to_string();
    let external_links = format!("{homepage} | {imdb}");

    let embed = CreateEmbed::new()
        .title(title)
        .url(url)
        .color(0x01b4e4)
        .thumbnail(poster)
        .description(format!("{tagline}{overview}"))
        .fields(vec![
            ("Status", status, true),
            ("Film ID", id, true),
            ("Language", language, true),
            ("Runtime", runtime, true),
            ("Release Date", release_date, true),
            ("Collection", collection, true),
            ("Popularity", popularity, true),
            ("User Score", format!("{user_score} ({user_score_count} votes)"), true),
            ("Budget", format!("${budget}"), true),
            ("Box Office", format!("${revenue}"), true),
            ("Genres", genres, true),
            ("Studios", studios, true),
            ("External Links", external_links, false),
        ])
        .footer(CreateEmbedFooter::new("Powered by the The Movie Database API."));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await.unwrap();

    Ok(())
}
