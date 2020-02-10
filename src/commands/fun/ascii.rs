use reqwest::blocking::Client;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[usage = "<font> <string>"]
#[example = "speed this is ascii"]
/// Converts a string to various ASCII forms. Various ASCII font faces / font types
/// are supported.
///
/// For the list of various fonts you can use, please visit the following website:
/// https://artii.herokuapp.com/fonts_list
///
/// **Note:** The ASCII text this command produces is best viewed on a desktop or
/// laptop computer, tablet, or a mobile device in landscape mode. Portrait mode
/// does not work well due to various issues with Discord's font handling on devices
/// such as iOS and Android phones.
pub fn ascii(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: No string provided.");
                embed.description(
                    "You didn't provide a string to convert to ASCII. Please provide one.\n\
                    For more details, please view the help documentation.",
                );
                embed.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    } else if arguments.rest().contains("\u{200B}") {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: Zero width space detected.");
                embed.description(
                    "A zero width space was detected in your message's content. This \
                    is not allowed. Please send a string without a zero width space included.",
                );
                embed.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;

    let text = arguments.rest();

    let font_url = "https://artii.herokuapp.com/fonts_list";
    let font_in_text = text.split_whitespace().next().unwrap_or("");
    let fonts_request = client.get(font_url).send()?.text()?;

    let request = if fonts_request.split_whitespace().any(|x| x == font_in_text) {
        // i don't know why this required a type annotation...but anyway
        // this took me way longer than was necessary to figure out.
        let font_name: &str = &text[0..font_in_text.chars().count()];
        let string = &text[font_in_text.chars().count()..];
        client.get("https://artii.herokuapp.com/make").query(&[("text", string), ("font", font_name)]).send()?
    } else {
        client.get("https://artii.herokuapp.com/make").query(&[("text", text)]).send()?
    };

    let response = request.text()?;

    message.channel_id.send_message(&context, |message| message.content(format!("```Markup\n{}```", response)))?;

    Ok(())
}
