use crate::models::tmdb::show::*;

use chrono::prelude::*;

use itertools::Itertools;

use reqwest::blocking::{Client, RequestBuilder};
use reqwest::redirect::Policy;

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[derive(Debug, Deserialize)]
pub struct SeriesSearchResponse {
    pub results: Vec<SeriesResult>,
}

#[derive(Debug, Deserialize)]
pub struct SeriesResult {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct MovieSearchResponse {
    pub results: Vec<MovieResult>,
}

#[derive(Debug, Deserialize)]
pub struct MovieResult {
    pub title: String,
    pub poster_path: String,
    pub id: i64,
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

/// Gets detailed information about the cast / crew of a television series
/// from The Movie Database.
#[command]
#[aliases("cast", "credits")]
pub fn cast(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    message.channel_id.broadcast_typing(&context)?;

    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: No show or film name provided.");
                embed.description(
                    "You did not provide the name of a show or film. Please provide one, \
                    and then try again.",
                );
                embed.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    let media_type: String = arguments.single()?;
    let mut media: String = arguments.rest().to_string();

    let api_key = crate::config::tmdb_key().expect("Could not find API key for The Movie Database...").to_owned();
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), ", v", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).redirect(Policy::none()).build()?;

    if media_type.contains("show") || media_type.contains("series") {
        let search_endpoint = "https://api.themoviedb.org/3/search/tv";
        let search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &media)]);
        let search_result: SeriesSearchResponse = search_response.send()?.json()?;
        let search_results = search_result.results;

        if search_results.is_empty() {
            message.channel_id.send_message(&context, |message| {
                message.content(format!(
                    "Sorry, I was unable to find a TV show on TMDb matching the term `{}`. \
                    Please try a different search term.",
                    media
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

        let credits_endpoint = format!("https://api.themoviedb.org/3/tv/{}/credits", show_id);
        let credits_response = client.get(&credits_endpoint).query(&[("api_key", &api_key)]).send()?;
        let credits_result: Credits = credits_response.json()?;

        let show_name = show_result.name;
        let show_cast = credits_result.cast;
        let show_crew = credits_result.crew;
        let mut show_cast_fields = Vec::with_capacity(show_cast.len());
        let mut show_crew_fields = Vec::with_capacity(show_crew.len());

        match show_cast.len() | show_crew.len() {
            // Match the amount of fields known to break the formatting of the
            // Discord embed on desktop, making the embed look a tad bit better
            // instead of having the 2nd field break its alignment due to a bug
            // that exists in the Discord desktop and web client's embed parsing
            // code. This issue does not exist on Discord's mobile apps, due to
            // the inline property being completely ignored, due to the way the
            // embeds were designed to look and work on Discord's suite of mobile
            // applications.
            //
            // Quite frankly, I'm honestly surprised that Discord has not yet been
            // able to figure out a fix to this embed parsing issue. Hopefully at
            // some point, they'll look into fixing this issue so that way this check
            // doesn't have to be made.
            //
            // Anyways, this matcher allows us to iterate through each element of
            // the show cast and crew, removing the last inlined field and disabling
            // its inline property, and re-inserting it into the show cast and crew
            // field vectors.
            //
            // TODO: Figure out why this is breaking with certain shows like How I Met 
            // Your Mother.
            5 | 8 | 11 | 14 | 17 | 20 | 23 => {
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
            // If the length of the show crew or cast does not match any of the known
            // embed field amounts we match against, proceed to push each cast / crew
            // member normally, making all of the fields inline.
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
                embed.title(format!("{} — Cast & Crew", show_name));
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
    } else if media_type.contains("movie") || media_type.contains("film") {
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
        if media.contains("y:") || media.contains("year:") {
            media = media.replace(" y:", "").replace(" year:", "");
            let mut year_rev: Vec<char> = media.chars().rev().take(4).collect();
            year_rev.reverse();
            let year = year_rev.iter().map(|c| c).join("");
            media = media.replace(&year, "");
            search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &media), ("year", &year)]);
        } else {
            search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &media)]);
        }

        let search_result: MovieSearchResponse = search_response.send()?.json()?;
        let search_results = search_result.results;

        if search_results.is_empty() {
            message.channel_id.send_message(&context, |message| {
                message.content(format!(
                    "Sorry, I was unable to find a movie on TMDb matching the term `{}`. \
                    Please try a different search term.",
                    media
                ))
            })?;
            return Ok(());
        }

        let movie_result = search_results.first().unwrap();
        let movie_id = movie_result.id;
        let movie_name = &movie_result.title;
        let movie_poster_url = &movie_result.poster_path;
        let movie_poster = format!("https://image.tmdb.org/t/p/original/{}", movie_poster_url.replace("/", ""));

        let credits_endpoint = format!("https://api.themoviedb.org/3/movie/{}/credits", movie_id);
        let credits_response = client.get(&credits_endpoint).query(&[("api_key", &api_key)]).send()?;
        let credits_result: Credits = credits_response.json()?;
        
        let movie_cast = &credits_result.cast[..20];
        let movie_crew = &credits_result.crew[..5];
        let movie_cast_url = format!("https://www.themoviedb.org/movie/{}/cast", movie_id);
        let mut movie_cast_fields = Vec::with_capacity(20);
        let mut movie_crew_fields = Vec::with_capacity(5);

        for member in movie_cast {
            movie_cast_fields.push((&member.name, &member.character, true));
        }

        for member in movie_crew {
            movie_crew_fields.push((&member.name, &member.job, true));
        }

        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title(format!("{} — Cast & Crew", movie_name));
                embed.color(0x0001_d277);
                embed.thumbnail(movie_poster);
                embed.description(format!(
                    "Please note that not all cast and crew members could be displayed \
                    for *{}*. For a full list of the cast and crew of this movie, please \
                    visit the full The Movie Database website by [clicking here]({}).", 
                    movie_name, movie_cast_url
                ));
                embed.fields(movie_cast_fields);
                if !movie_crew_fields.is_empty() {
                    embed.fields(movie_crew_fields);
                }
                embed.footer(|footer| footer.text("Powered by the The Movie Database API."));
                embed.timestamp(&Utc::now())
            })
        })?;
        
        return Ok(())
    } else {
        message.channel_id.send_message(&context, |message| message.content("This is not a recognized media type!"))?;
        return Ok(())
    }
}
