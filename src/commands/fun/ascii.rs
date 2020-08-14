use reqwest::Client;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
};

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
/// does not work well due to various is
pub async fn ascii(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message
            .channel_id
            .send_message(&context, |message| {
                message.embed(|embed| {
                    embed.title("Error: No string provided.");
                    embed.description(
                        "You didn't provide a string to convert to ASCII. Please provide one.\n\
                        For more details, please view the help documentation.",
                    );
                    embed.color(0x00FF_0000)
                })
            })
            .await?;
        return Ok(());
    } else if arguments.rest().contains("\u{200B}") {
        message
            .channel_id
            .send_message(&context, |message| {
                message.embed(|embed| {
                    embed.title("Error: Zero width space detected.");
                    embed.description(
                        "A zero width space was detected in your message's content. This \
                        is not allowed. Please send a string without a zero width space included.",
                    );
                    embed.color(0x00FF_0000)
                })
            })
            .await?;
        return Ok(());
    }

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;

    let text = arguments.rest();

    let font_url = "https://artii.herokuapp.com/fonts_list";
    let font_in_text = text.split_whitespace().next().unwrap_or("");
    let fonts_request = client.get(font_url).send().await?.text().await?;

    // when checking for fonts, make sure a check is made, checking for both the font name
    // in question & that there is a space. this will not work with zero width spaces due
    // to the check that is already in place, and any spaces that are inserted without any
    // additional characters are already trimmed or removed by Discord.
    let request = if fonts_request.split_whitespace().any(|x| x == font_in_text) && text.contains('\u{0020}') {
        // i don't know why this required a type annotation...but anyway
        // this took me way longer than was necessary to figure out.
        let font_name: &str = &text[0..font_in_text.chars().count()];
        let string = &text[font_in_text.chars().count()..];
        client.get("https://artii.herokuapp.com/make").query(&[("text", string), ("font", font_name)]).send().await?
    } else {
        client.get("https://artii.herokuapp.com/make").query(&[("text", text)]).send().await?
    };

    let response = request.text().await?;

    message
        .channel_id
        .send_message(&context, |message| message.content(format!("```Markup\n{}```", response)))
        .await?;

    Ok(())
}
