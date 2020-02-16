use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use reqwest::blocking::Client;

#[command]
#[aliases("pf")]
/// Gets a random printer fact from the printer facts API.
pub fn printerfacts(context: &mut Context, message: &Message) -> CommandResult {

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;
    
    let printerfacts_endpoint = "https://printerfacts.cetacean.club/fact";
    let printerfacts_response = client.get(printerfacts_endpoint).send()?.text()?;

    message.channel_id.say(&context, printerfacts_response)?;

    Ok(())
}