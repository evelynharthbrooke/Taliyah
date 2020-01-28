use reqwest::blocking::Client;

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "list")]
    definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    definition: String,
    example: String,
    word: String,
    permalink: String,
}

#[command]
#[description = "Looks up a definition from Urban Dictionary."]
#[usage = "<name of word>"]
#[aliases("urbandict", "ud", "urban", "define")]
pub fn urban(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |m| {
            m.embed(|e| {
                e.title("Error: No word provided.");
                e.description("You did not provide a word to look up. Please provide one and then try again.");
                e.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;

    let term = arguments.rest();

    let request = client.get("https://api.urbandictionary.com/v0/define").query(&[("term", term)]).send()?;
    let response: Response = request.json()?;

    if response.definitions.len() == 0 {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title(format!("Error: No definitions for `{}` found.", term));
                embed.color(0x00FF_0000);
                embed.description(format!("No definitions found for the word `{}`. Please try searching for a different word.", term))
            })
        })?;
        return Ok(());
    }

    let word = &response.definitions[0].word;
    let definition = &response.definitions[0].definition;
    let example = &response.definitions[0].example;
    let permalink = &response.definitions[0].permalink;

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(word);
                author.url(permalink)
            });
            embed.color(0xEFFF00);
            embed.description(format!("{}\n\n**Example**:\n{}", definition, example));
            embed.footer(|footer| footer.text("Powered by the Urban Dictionary."))
        })
    })?;

    return Ok(());
}

#[command]
#[description = "Gets a random definition from the Urban Dictionary."]
pub fn randefine(context: &mut Context, message: &Message) -> CommandResult {
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;
    let request = client.get("http://api.urbandictionary.com/v0/random").send()?;
    let response: Response = request.json()?;

    let word = &response.definitions[0].word;
    let definition = &response.definitions[0].definition;
    let example = &response.definitions[0].example;
    let permalink = &response.definitions[0].permalink;

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(word);
                author.url(permalink)
            });
            embed.color(0xEFFF00);
            embed.description(format!("{}\n\n**Example**:\n{}", definition, example));
            embed.footer(|footer| footer.text("Powered by the Urban Dictionary."))
        })
    })?;

    return Ok(())
}