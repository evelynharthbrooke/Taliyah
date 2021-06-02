use reqwest::StatusCode;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{interactions::ButtonStyle, prelude::Message}
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
pub async fn xkcd(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let comic_num = arguments.single::<u32>().unwrap_or(0);

    let latest_comic = "https://xkcd.com/info.0.json";
    let xkcd_url = format!("https://xkcd.com/{}/info.0.json", comic_num);

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let xkcd_request = client.get(if comic_num == 0 { latest_comic } else { &xkcd_url }).send().await?;

    if xkcd_request.status() == StatusCode::NOT_FOUND {
        message.reply(context, "You did not provide a valid xkcd comic ID!").await?;
        return Ok(());
    }

    let xkcd_response: XkcdComic = xkcd_request.json().await?;

    let title = &xkcd_response.title;
    let alt = &xkcd_response.alt;
    let num = xkcd_response.num;

    let xkcd_page = format!("https://xkcd.com/{}", num);
    let explain_xkcd = format!("https://explainxkcd.com/wiki/index.php/{}", num);

    message
        .channel_id
        .send_message(context, |message| {
            message.embed(|embed| {
                embed.title(title);
                embed.description(alt);
                embed.image(xkcd_response.img.as_str());
                embed.footer(|footer| footer.text(format!("xkcd comic no. {}", num)));
                embed
            });
            message.components(|c| {
                c.create_action_row(|row| {
                    row.create_button(|b| b.label("View Comic").style(ButtonStyle::Link).url(xkcd_page));
                    row.create_button(|b| b.label("Explain XKCD").style(ButtonStyle::Link).url(explain_xkcd));
                    row
                });
                c
            });
            message
        })
        .await?;

    Ok(())
}
