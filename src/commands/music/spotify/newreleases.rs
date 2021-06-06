use aspotify::CountryCode;
use itertools::Itertools;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::{data::SpotifyContainer, utils::locale_utils};

#[command]
#[description("Displays information about the new releases for a given market.")]
pub async fn newreleases(context: &Context, message: &Message, args: Args) -> CommandResult {
    let market = args.rest().to_string();

    if !market.is_empty() {
        if market.len() < 2 || market.len() > 2 {
            message.channel_id.say(context, "The market name you provided is more or less than 2 characters long.").await?;
            return Ok(());
        }
    } else if market.is_empty() {
        message.channel_id.say(context, "You did not provide a valid market name.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let spotify = data.get::<SpotifyContainer>().unwrap();
    let country_iso = CountryCode::for_alpha2_caseless(&market).unwrap();
    let country_name = locale_utils::get_country_name_from_iso(&market);
    let new_releases = spotify.browse().get_new_releases(20, 0, Some(country_iso)).await?;
    let nr_items = new_releases
        .data
        .items
        .iter()
        .map(|album| {
            let album_name = &album.name;
            let album_artists = &album.artists.iter().map(|a| &a.name).join(", ");
            let album_date = album.release_date.unwrap().format("%B %-d, %Y").to_string();
            format!("**{album_name}** — {album_artists} — {album_date}")
        })
        .join("\n");

    message
        .channel_id
        .send_message(context, |m| {
            m.embed(|e| {
                e.title(format!("New Releases on Spotify for: {country_name}"));
                e.colour(0x001D_B954);
                e.description(nr_items);
                e.footer(|f| f.text("Powered by the Spotify Web API."));
                e
            });
            m
        })
        .await?;

    return Ok(());
}
