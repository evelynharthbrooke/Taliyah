use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

use crate::data::ReqwestContainer;

#[command]
#[aliases("pf")]
/// Gets a random printer fact from the printer facts API.
async fn printerfacts(context: &Context, message: &Message) -> CommandResult {
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let endpoint = "https://printerfacts.cetacean.club/fact";
    let response = client.get(endpoint).send().await?;
    let text = response.text().await?;
    message.channel_id.say(&context, text).await?;
    Ok(())
}
