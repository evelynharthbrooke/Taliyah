use aspotify::CountryCode;
use itertools::Itertools;

use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::{data::SpotifyContainer, utils::locale};

#[command]
#[description("Displays information about the new releases for a given market.")]
async fn newreleases(context: &Context, message: &Message, args: Args) -> CommandResult {
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
    let country_name = locale::get_country_name_from_iso(&market);
    let new_releases = spotify.browse().get_new_releases(20, 0, Some(country_iso)).await?;
    #[rustfmt::skip]
    let nr_items = new_releases.data.items.iter().map(|album| {
        let album_name = &album.name;
        let album_artists = &album.artists.iter().map(|a| &a.name).join(", ");
        let album_date = album.release_date.unwrap().format("%B %-d, %Y").to_string();
        format!("**{album_name}** — {album_artists} — {album_date}")
    }).join("\n");

    let embed = CreateEmbed::new()
        .title(format!("New Releases on Spotify for {country_name}"))
        .colour(0x001D_B954)
        .description(nr_items)
        .footer(CreateEmbedFooter::new("Powered by the Spotify Web API."));

    message.channel_id.send_message(context, CreateMessage::new().embed(embed)).await?;

    return Ok(());
}
