use crate::data::ReqwestContainer;
use reqwest::StatusCode;
use serde::Deserialize;
use serenity::{
    builder::{CreateActionRow, CreateButton, CreateComponents, CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

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
    let selected_comic = format!("https://xkcd.com/{comic_num}/info.0.json");
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let request = client.get(if comic_num == 0 { latest_comic } else { &selected_comic }).send().await?;
    if request.status() == StatusCode::NOT_FOUND {
        message.reply(context, "You did not provide a valid comic id.").await?;
        return Ok(());
    }

    let response: XkcdComic = request.json().await?;
    let num = response.num;
    let page = format!("https://xkcd.com/{num}");
    let wiki = format!("https://explainxkcd.com/wiki/index.php/{num}");

    let embed = CreateEmbed::new()
        .title(&response.title)
        .color(0xfafafa)
        .description(&response.alt)
        .image(&response.img)
        .footer(CreateEmbedFooter::new(format!("xkcd comic no. {num}")));

    let components = CreateComponents::new().add_action_row(
        CreateActionRow::new()
            .add_button(CreateButton::new_link(page).label("View xkcd image page"))
            .add_button(CreateButton::new_link(wiki).label("View explanation"))
    );

    message.channel_id.send_message(context, CreateMessage::new().embed(embed).components(components)).await?;

    Ok(())
}
