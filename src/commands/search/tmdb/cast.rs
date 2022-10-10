use itertools::Itertools;
use reqwest::RequestBuilder;
use serde::Deserialize;

use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::{data::ReqwestContainer, models::tmdb::show::*, utils::read_config};

#[derive(Debug, Deserialize)]
pub struct SeriesSearchResponse {
    pub results: Vec<SeriesResult>
}

#[derive(Debug, Deserialize)]
pub struct SeriesResult {
    pub id: i64
}

#[derive(Debug, Deserialize)]
pub struct MovieSearchResponse {
    pub results: Vec<MovieResult>
}

#[derive(Debug, Deserialize)]
pub struct MovieResult {
    pub title: String,
    pub poster_path: String,
    pub id: i64
}

#[derive(Debug, Deserialize)]
pub struct Credits {
    pub cast: Vec<CastMember>,
    pub crew: Vec<CrewMember>,
    pub id: i64
}

#[derive(Debug, Deserialize)]
pub struct CastMember {
    pub character: String,
    pub credit_id: String,
    pub id: i64,
    pub name: String,
    pub gender: i64,
    pub profile_path: Option<String>,
    pub order: i64
}

#[derive(Debug, Deserialize)]
pub struct CrewMember {
    pub credit_id: String,
    pub department: String,
    pub id: i64,
    pub name: String,
    pub gender: Option<i64>,
    pub job: String,
    pub profile_path: Option<String>
}

/// Gets detailed information about the cast / crew of a television series
/// from The Movie Database.
#[command]
#[aliases("cast", "credits")]
#[min_args(2)]
#[max_args(2)]
async fn cast(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "No show or movie name provided. Provide one & try again.").await?;
        return Ok(());
    }

    let media_type: String = arguments.single()?;
    let mut input: String = arguments.rest().to_string();

    let config = read_config("config.toml");
    let api_key = config.api.entertainment.tmdb;
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();

    if media_type.contains("show") || media_type.contains("series") {
        let search_endpoint = "https://api.themoviedb.org/3/search/tv";
        let search_response: RequestBuilder;

        // This is a pretty hacky way of being able to search by year, but
        // surprisingly enough it actually works from what I've tested, and
        // while it might be a tad slow, it should compute fast enough to not
        // make users wonder why its taking so long for the response to send.
        //
        // This code follows the y: notation syntax as available on the website
        // for The Movie Database, with the additional ability to use year: in
        // place of y:, depending on the user's preference.
        if input.contains("y:") || input.contains("year:") {
            input = input.replace(" y:", "").replace(" year:", "");
            let mut year_rev: Vec<char> = input.chars().rev().take(4).collect();
            year_rev.reverse();
            let year = year_rev.iter().join("");
            input = input.replace(&year, "");
            let query_string = &[("api_key", &api_key), ("query", &input), ("first_air_date_year", &year)];
            search_response = client.get(search_endpoint).query(query_string);
        } else {
            search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &input)]);
        }

        let search_result: SeriesSearchResponse = search_response.send().await?.json().await?;
        let search_results = search_result.results;

        if search_results.is_empty() {
            message.channel_id.say(&context, format!("Nothing found for `{}`. Please try a different term.", input)).await?;
            return Ok(());
        }

        let show_id = search_results.first().unwrap().id;

        let show_endpoint = format!("https://api.themoviedb.org/3/tv/{}", show_id);
        let show_sub_requests = ("append_to_response", &"external_ids".to_string());
        let show_response = client.get(&show_endpoint).query(&[("api_key", &api_key), show_sub_requests]).send().await?;
        let show_result: Show = show_response.json().await?;
        let show_poster_path = show_result.poster_path.unwrap();
        let show_poster = format!("https://image.tmdb.org/t/p/original/{}", &show_poster_path.replace('/', ""));

        let credits_endpoint = format!("https://api.themoviedb.org/3/tv/{}/credits", show_id);
        let credits_response = client.get(&credits_endpoint).query(&[("api_key", &api_key)]).send().await?;
        let credits_result: Credits = credits_response.json().await?;

        let show_name = show_result.name;
        let show_cast = credits_result.cast;
        let show_crew = credits_result.crew;
        let mut show_cast_fields = Vec::with_capacity(show_cast.len());
        let mut show_crew_fields = Vec::with_capacity(show_crew.len());

        // TODO: Look into maybe simplifying this check
        match show_cast.len() | show_crew.len() {
            // Match the amount of fields known to break the formatting of the
            // Discord embed on desktop, making the embed look a tad bit better
            // instead of having the 2nd field break its alignment due to a bug
            // that exists in the Discord desktop and web client's embed parsing
            // code. This issue does not exist on Discord's mobile apps, due to
            // the inline property being completely ignored, in turn due to the
            // way the embeds were designed to look and work on Discord's suite
            // of mobile applications.
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
            5 | 8 | 11 | 14 | 17 | 20 | 23 => {
                for member in &show_cast[0..show_cast.len() - 1] {
                    show_cast_fields.push((&member.name, &member.character, true));
                }

                let last_cast_member = show_cast.last().unwrap();
                show_cast_fields.push((&last_cast_member.name, &last_cast_member.character, false));

                // Check the length of the show's crew array and make sure that it
                // is larger than zero and greater than two, so that way we don't
                // end up running into any array overflow errors when subtracting
                // the last crew member from the array.
                //
                // When the array is greater than or equal to one, push the show
                // crew members normally, without going through the process of
                // removing the last crew member and re-adding it without the
                // inline property enabled.
                if !show_crew.is_empty() && show_crew.len() > 2 {
                    let last_crew_member = show_crew.last().unwrap();
                    for member in &show_crew[0..show_cast.len() - 1] {
                        show_crew_fields.push((&member.name, &member.job, true));
                    }
                    show_crew_fields.push((&last_crew_member.name, &last_crew_member.job, false));
                } else if !show_crew.is_empty() {
                    for member in &show_crew[0..show_cast.len()] {
                        show_crew_fields.push((&member.name, &member.job, true));
                    }
                }
            }
            // If the length of the show crew or cast does not match any of the known
            // embed field amounts we match against, proceed to push each cast / crew
            // member normally, making all of the fields inline.
            _ => {
                for member in &show_cast {
                    show_cast_fields.push((&member.name, &member.character, true));
                }

                // This specifically fixes an issue with looking up the show How I
                // Met Your Mother, where for some odd reason, the last crew member
                // would refuse to go onto a separate non-inlined embed field and
                // instead continue to be inlined, causing the last embed row to look
                // weird.
                //
                // Personal Note: The amount of crew members listed in the HIMYM array
                // could change at any time, so this is a really hacky way of fixing
                // the issue and a better solution should be looked into.
                if show_crew.len() == 11 {
                    for member in &show_crew[..11 - 2] {
                        show_crew_fields.push((&member.name, &member.job, true));
                    }

                    for member in &show_crew[9..11] {
                        show_crew_fields.push((&member.name, &member.job, false))
                    }
                } else {
                    for member in &show_crew {
                        show_crew_fields.push((&member.name, &member.job, true));
                    }
                }
            }
        }

        let embed = CreateEmbed::new()
            .title(format!("{} — Cast & Crew", show_name))
            .color(0x0001_d277)
            .thumbnail(show_poster)
            .fields(if !show_crew_fields.is_empty() { show_crew_fields } else { show_cast_fields })
            .footer(CreateEmbedFooter::new("Powered by The Movie Database."));

        message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

        Ok(())
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
        if input.contains("y:") || input.contains("year:") {
            input = input.replace(" y:", "").replace(" year:", "");
            let mut year_rev: Vec<char> = input.chars().rev().take(4).collect();
            year_rev.reverse();
            let year = year_rev.iter().join("");
            input = input.replace(&year, "");
            search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &input), ("year", &year)]);
        } else {
            search_response = client.get(search_endpoint).query(&[("api_key", &api_key), ("query", &input)]);
        }

        let search_result: MovieSearchResponse = search_response.send().await?.json().await?;
        let search_results = search_result.results;

        if search_results.is_empty() {
            message.channel_id.say(&context, format!("Nothing found for `{}`. Please try a different term.", input)).await?;
            return Ok(());
        }

        let movie_result = search_results.first().unwrap();
        let movie_id = movie_result.id;
        let movie_name = &movie_result.title;
        let movie_poster_url = &movie_result.poster_path;
        let movie_poster = format!("https://image.tmdb.org/t/p/original/{}", movie_poster_url.replace('/', ""));

        let credits_endpoint = format!("https://api.themoviedb.org/3/movie/{}/credits", movie_id);
        let credits_response = client.get(&credits_endpoint).query(&[("api_key", &api_key)]).send().await?;
        let credits_result: Credits = credits_response.json().await?;

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

        let embed = CreateEmbed::new()
            .title(format!("{} — Cast & Crew", movie_name))
            .color(0x0001_d277)
            .thumbnail(movie_poster)
            .description(format!(
                "\
            Not all cast and crew members could be displayed for *{}*. For a full \
            list of the cast / crew members in this movie, please visit the The Movie \
            Database website by [clicking here]({}).\
            ",
                movie_name, movie_cast_url
            ))
            .fields(if !movie_crew_fields.is_empty() { movie_crew_fields } else { movie_cast_fields })
            .footer(CreateEmbedFooter::new("Powered by The Movie Database."));

        message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

        Ok(())
    } else {
        message.channel_id.say(&context, "This is not a recognized media type!").await?;
        Ok(())
    }
}
