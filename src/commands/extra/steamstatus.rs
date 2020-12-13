use reqwest::StatusCode;
use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

use crate::data::ReqwestContainer;

/// Fetches the current status of Steam.
#[command("steamstatus")]
pub async fn steamstatus(context: &Context, message: &Message) -> CommandResult {
    let steamstatus_url = "https://crowbar.steamstat.us/gravity.json";

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let request = client.get(steamstatus_url).send().await?;

    if request.status() == StatusCode::INTERNAL_SERVER_ERROR {
        message.reply(context, "steamstat.us is currently offline; cannot retrieve Steam status.").await?;
        return Ok(());
    }

    let response: serde_json::Value = request.json().await?;

    let steam_community = &response["services"][3][2];

    println!("{:#?}", steam_community);

    Ok(())
}
