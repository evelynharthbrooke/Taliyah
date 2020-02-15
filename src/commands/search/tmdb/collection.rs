use chrono::prelude::*;
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use serde::Deserialize;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub results: Vec<SearchCollection>, // The collection search results.
}

#[derive(Deserialize, Debug)]
pub struct SearchCollection {
    pub id: u64, // The ID of the collection. All we need.
}

#[derive(Deserialize, Debug)]
pub struct Collection {
    pub id: u64,                     // The TMDb ID belonging to the collection.
    pub name: String,                // The name of the collection.
    pub overview: String,            // The overview of the collection.
    pub poster_path: String,         // The poster belonging to the collection.
    pub backdrop_path: String,       // The backdrop path of the collection.
    pub parts: Vec<SimplifiedMovie>, // The movies part of the collection.
}

#[derive(Deserialize, Debug)]
pub struct SimplifiedMovie {
    pub adult: bool,             // Whether or not the movie is marked as an adult film by TMDb.
    pub id: u64,                 // The TMDb ID belonging to the movie.
    pub overview: String,        // The overview of the movie.
    pub release_date: NaiveDate, // The release date of the movie.
    pub title: String,           // The title of the movie.
}

#[command]
#[aliases("collection")]
#[description("Gets detailed information about a collection from The Movie Database.")]
pub fn collection(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: Invalid collection name provided.");
                embed.description("You have provided an invalid collection name. Please try again.");
                embed.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    message.channel_id.broadcast_typing(&context)?;

    let collection: String = arguments.rest().to_string();

    let api_key = std::env::var("TMDB_KEY").expect("Could not find API key for The Movie Database...");
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), ", v", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).redirect(Policy::none()).build()?;

    let search_endpoint = "https://api.themoviedb.org/3/search/collection";
    let search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &collection)]);
    let search_result: SearchResponse = search_response.send()?.json()?;
    let search_results = search_result.results;

    if search_results.is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.content(format!(
                "Sorry, I was unable to find a collection on TMDb matching the term `{}`. \
                Please try a different search term.",
                collection
            ))
        })?;
        return Ok(());
    }

    let collection_id = search_results.first().unwrap().id;
    let collection_endpoint = format!("https://api.themoviedb.org/3/collection/{}", collection_id);
    let collection_response = client.get(&collection_endpoint).query(&[("api_key", &api_key)]).send()?;
    let collection_result: Collection = collection_response.json()?;

    let collection_name = collection_result.name;
    let collection_poster = format!("https://image.tmdb.org/t/p/original{}", collection_result.poster_path);
    let collection_id = collection_result.id;
    let collection_url = format!("https://www.themoviedb.org/collection/{}", collection_id);
    let collection_overview = collection_result.overview;
    let collection_parts = collection_result.parts;
    let mut collection_fields = Vec::with_capacity(collection_parts.len());

    for part in &collection_parts {
        collection_fields.push((format!("{} — released {}", part.title, &part.release_date.format("%B %-e, %Y")), &part.overview, false))
    }

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title(collection_name);
            embed.url(collection_url);
            embed.thumbnail(collection_poster);
            embed.color(0x0001_d277);
            embed.description(collection_overview);
            embed.fields(collection_fields);
            embed.footer(|footer| footer.text("Powered by the The Movie Database API."));
            embed.timestamp(&Utc::now())
        })
    })?;

    Ok(())
}
