use reqwest::StatusCode;
use serde::Deserialize;
use serenity::{
    builder::{CreateActionRow, CreateButton, CreateComponents, CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::data::ReqwestContainer;

#[derive(Debug, Clone, Deserialize)]
struct XkcdComic {
    num: u16,      // the numeric ID of the xkcd comic.
    alt: String,   // the caption of the xkcd comic.
    img: String,   // the image URL of the xkcd comic.
    title: String  // the title of the xkcd comic.
}

/// Retrieves the latest or a given comic from xkcd.
#[command]
async fn xkcd(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let comic_num = arguments.single::<u32>().unwrap_or(0);
    let latest_comic = "https://xkcd.com/info.0.json";
    let xkcd_url = format!("https://xkcd.com/{}/info.0.json", comic_num);
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let request = client.get(if comic_num == 0 { latest_comic } else { &xkcd_url }).send().await?;
    if request.status() == StatusCode::NOT_FOUND {
        message.reply(context, "You did not provide a valid xkcd comic ID!").await?;
        return Ok(());
    }

    let response: XkcdComic = request.json().await?;
    let title = &response.title;
    let alt = &response.alt;
    let num = response.num;
    let page = format!("https://xkcd.com/{}", num);
    let wiki = format!("https://explainxkcd.com/wiki/index.php/{}", num);

    let embed = CreateEmbed::new()
        .title(title)
        .color(0xfafafa)
        .description(alt)
        .image(response.img)
        .footer(CreateEmbedFooter::new(format!("xkcd comic no. {}", &num)));

    let components = CreateComponents::new().add_action_row(
        CreateActionRow::new()
            .add_button(CreateButton::new_link(page).label("View xkcd image page"))
            .add_button(CreateButton::new_link(wiki).label("View explanation"))
    );

    message.channel_id.send_message(context, CreateMessage::new().embed(embed).components(components)).await?;

    Ok(())
}
