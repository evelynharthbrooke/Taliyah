use serde::Deserialize;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::data::ReqwestContainer;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "list")]
    definitions: Vec<Definition>
}

#[derive(Debug, Deserialize)]
pub struct Definition {
    definition: String,
    example: String,
    word: String,
    thumbs_up: usize,
    thumbs_down: usize,
    permalink: String
}

#[command]
#[description = "Looks up a definition from the Urban Dictionary."]
#[usage = "<name of word>"]
#[aliases("urbandict", "ud", "urban", "define")]
pub async fn urban(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "You did not provide a word to look up. Please provide one.").await?;
        return Ok(());
    }

    let term = arguments.rest();

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let request = client.get("https://api.urbandictionary.com/v0/define").query(&[("term", term)]).send().await?;
    let response: Response = request.json().await?;

    if response.definitions.is_empty() {
        message.channel_id.say(context, format!("No definitions found for `{}`. Try a different word.", term)).await?;
        return Ok(());
    }

    let word = &response.definitions[0].word;
    let definition = &response.definitions[0].definition;
    let example = &response.definitions[0].example;
    let permalink = &response.definitions[0].permalink;
    let thumbs_up = &response.definitions[0].thumbs_up;
    let thumbs_down = &response.definitions[0].thumbs_down;

    let rating = format!("{} 👍 | {} 👎", thumbs_up, thumbs_down);

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.author(|author| author.name(word).url(permalink));
                embed.color(0x00EF_FF00);
                embed.description(format!("*{}*\n\n{}\n\n**Ratings**: {}", definition, example, rating));
                embed.footer(|footer| footer.text("Powered by the Urban Dictionary."))
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description = "Gets a random definition from the Urban Dictionary."]
pub async fn randefine(context: &Context, message: &Message) -> CommandResult {
    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let request = client.get("http://api.urbandictionary.com/v0/random").send().await?;
    let response: Response = request.json().await?;

    let word = &response.definitions[0].word;
    let definition = &response.definitions[0].definition;
    let example = &response.definitions[0].example;
    let permalink = &response.definitions[0].permalink;
    let thumbs_up = &response.definitions[0].thumbs_up;
    let thumbs_down = &response.definitions[0].thumbs_down;

    let rating = format!("{} 👍 | {} 👎", thumbs_up, thumbs_down);

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.author(|author| author.name(word).url(permalink));
                embed.color(0x00EF_FF00);
                embed.description(format!("*{}*\n\n{}\n\n**Ratings**: {}", definition, example, rating));
                embed.footer(|footer| footer.text("Powered by the Urban Dictionary."))
            })
        })
        .await?;

    Ok(())
}
