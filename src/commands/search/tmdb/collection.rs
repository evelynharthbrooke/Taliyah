use chrono::prelude::{NaiveDate, Utc};
use serde::Deserialize;

use serenity::{
    builder::CreateActionRow,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{interactions::ButtonStyle, prelude::Message}
};

use crate::{data::ReqwestContainer, utils::read_config};

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<SearchCollection> // The collection search results.
}

#[derive(Deserialize, Debug)]
pub struct SearchCollection {
    pub id: u64 // The ID of the collection. All we need.
}

#[derive(Deserialize, Debug)]
pub struct Collection {
    pub id: u64,                     // The TMDb ID belonging to the collection.
    pub name: String,                // The name of the collection.
    pub overview: String,            // The overview of the collection.
    pub poster_path: String,         // The poster belonging to the collection.
    pub backdrop_path: String,       // The backdrop path of the collection.
    pub parts: Vec<SimplifiedMovie>  // The movies part of the collection.
}

#[derive(Deserialize, Debug)]
pub struct SimplifiedMovie {
    pub adult: bool,             // Whether or not the movie is marked as an adult film by TMDb.
    pub id: u64,                 // The TMDb ID belonging to the movie.
    pub overview: String,        // The overview of the movie.
    pub release_date: NaiveDate, // The release date of the movie.
    pub title: String            // The title of the movie.
}

#[derive(Deserialize, Debug)]
pub struct Movie {
    pub status: String
}

#[command]
#[aliases("collection")]
#[description("Gets detailed information about a collection from The Movie Database.")]
async fn collection(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "Invalid collection name provided. Please try again.").await?;
        return Ok(());
    }

    let collection: String = arguments.rest().to_string();

    let config = read_config("config.toml");
    let api_key = config.api.entertainment.tmdb;
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();

    let search_endpoint = "https://api.themoviedb.org/3/search/collection";
    let search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &collection)]);
    let search_result: SearchResponse = search_response.send().await?.json().await?;
    let search_results = search_result.results;

    if search_results.is_empty() {
        message.channel_id.say(context, format!("Nothing found for `{}`. Please try something else.", collection)).await?;
        return Ok(());
    }

    let collection_id = search_results.first().unwrap().id;
    let collection_endpoint = format!("https://api.themoviedb.org/3/collection/{}", collection_id);
    let collection_response = client.get(&collection_endpoint).query(&[("api_key", &api_key)]).send().await?;
    let collection_result: Collection = collection_response.json().await?;

    let collection_name = collection_result.name;
    let collection_poster = format!("https://image.tmdb.org/t/p/original{}", collection_result.poster_path);
    let collection_id = collection_result.id;
    let collection_url = format!("https://www.themoviedb.org/collection/{}", collection_id);
    let collection_overview = collection_result.overview;
    let collection_parts = collection_result.parts;
    let mut collection_fields = Vec::with_capacity(collection_parts.len());
    let mut collection_rows = CreateActionRow::default();

    for part in &collection_parts {
        // This probably isn't the best implementation for getting a collection
        // movie's release date, because every time a collection entry is processed,
        // its gonna make a request to The Movie Database, using additional requests
        // in the process. While the TMDb API doesn't have rate limits, this might
        // become a bit network I/O intensive if there are a lot of movies in a given
        // collection.
        let part_id = part.id;
        let part_title = &part.title;
        let part_url = format!("https://www.themoviedb.org/movie/{}", part_id);
        let part_release_date = part.release_date.format("%B %-e, %Y");
        let part_summary = &part.overview;
        let movie_endpoint = format!("https://api.themoviedb.org/3/movie/{}", part_id);
        let movie_response = client.get(&movie_endpoint).query(&[("api_key", &api_key)]).send().await?;
        let movie_result: Movie = movie_response.json().await?;
        let movie_status = match movie_result.status.as_str() {
            "Planned" | "In Production" | "Post Production" => "releasing on",
            _ => "released"
        };

        collection_fields.push((format!("{} — {} {}", part_title, movie_status, part_release_date), part_summary, false));
        collection_rows.create_button(|b| b.label(part_title).url(part_url).style(ButtonStyle::Link));
    }

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.title(collection_name);
                embed.url(collection_url);
                embed.thumbnail(collection_poster);
                embed.color(0x0001_d277);
                embed.description(collection_overview);
                embed.fields(collection_fields);
                embed.footer(|footer| footer.text("Powered by The Movie Database."));
                embed.timestamp(&Utc::now())
            });
            message.components(|comps| comps.add_action_row(collection_rows))
        })
        .await?;

    Ok(())
}
