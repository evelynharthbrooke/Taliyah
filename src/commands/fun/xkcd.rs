use reqwest::StatusCode;
use serde::Deserialize;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{interactions::ButtonStyle::Link, prelude::Message}
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

    message
        .channel_id
        .send_message(context, |message| {
            message.embed(|embed| {
                embed.title(title);
                embed.color(0xfafafa);
                embed.description(alt);
                embed.image(response.img.as_str());
                embed.footer(|f| f.text(format!("xkcd comic no. {}", &num)));
                embed
            });
            message.components(|comps| {
                comps.create_action_row(|row| row.create_button(|b| b.label("View xkcd image page").style(Link).url(page)));
                comps.create_action_row(|row| row.create_button(|b| b.label("View explanation").style(Link).url(wiki)));
                comps
            });
            message
        })
        .await?;

    Ok(())
}
