use chrono::prelude::NaiveDate;
use serde::Deserialize;

use serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
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
        message.channel_id.say(context, format!("Nothing found for `{collection}`. Please try something else.")).await?;
        return Ok(());
    }

    let collection_id = search_results.first().unwrap().id;
    let collection_endpoint = format!("https://api.themoviedb.org/3/collection/{collection_id}");
    let collection_response = client.get(&collection_endpoint).query(&[("api_key", &api_key)]).send().await?;
    let collection_result: Collection = collection_response.json().await?;

    let collection_name = collection_result.name;
    let collection_poster = format!("https://image.tmdb.org/t/p/original{}", collection_result.poster_path);
    let collection_url = format!("https://www.themoviedb.org/collection/{collection_id}");
    let collection_overview = collection_result.overview;

    let mut parts = collection_result.parts;
    let mut fields = Vec::with_capacity(parts.len());
    parts.sort_by_cached_key(|p| p.release_date);

    #[rustfmt::skip]
    let rows = parts.chunks(5).map(|chunk| CreateActionRow::Buttons(chunk.iter().map(|part| {
        let id = &part.id;
        let title = &part.title;
        let release_date = part.release_date.format("%B %-e, %Y");
        let summary = &part.overview;
        fields.push((format!("{title} ({release_date})"), summary, false));
        CreateButton::new_link(title, format!("https://themoviedb.org/movie/{id}"))
    }).collect())).collect::<Vec<CreateActionRow>>();

    let embed = CreateEmbed::new()
        .title(collection_name)
        .url(collection_url)
        .thumbnail(collection_poster)
        .color(0x0001_d277)
        .description(collection_overview)
        .fields(fields)
        .footer(CreateEmbedFooter::new("Powered by TMDb."));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed).components(rows)).await?;

    Ok(())
}
