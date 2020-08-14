use reqwest::Client;

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

#[command]
#[aliases("pf")]
/// Gets a random printer fact from the printer facts API.
pub async fn printerfacts(context: &Context, message: &Message) -> CommandResult {
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;

    let printerfacts_endpoint = "https://printerfacts.cetacean.club/fact";
    let printerfacts_response = client.get(printerfacts_endpoint).send().await?;
    let printerfacts_text = printerfacts_response.text().await?;

    message.channel_id.say(&context, printerfacts_text).await?;

    Ok(())
}
