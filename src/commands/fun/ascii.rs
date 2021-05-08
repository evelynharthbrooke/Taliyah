use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::data::ReqwestContainer;

#[command]
#[usage = "<font> <string>"]
#[example = "speed this is ascii"]
/// Converts a string to various ASCII forms. Various ASCII font faces / font types
/// are supported and are available to send to the command to change / modify the
/// look of the fed ASCII text.
///
/// For the list containing the various fonts you can use, please visit the following
/// website: https://artii.herokuapp.com/fonts_list
///
/// **Note:** The ASCII text this command produces is best viewed on a desktop or
/// laptop computer, tablet, or a smartphone in landscape mode. Portrait mode will
/// not and does not work well due to various issues relating to the portrait nature
/// of that orientation.
pub async fn ascii(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "No ASCII string provided. Please provide one.").await?;
        return Ok(());
    } else if arguments.rest().contains('\u{200B}') {
        message.channel_id.say(context, "Zero width space detected. Don't send these.").await?;
        return Ok(());
    }

    let text = arguments.rest();

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
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

    message.channel_id.say(context, format!("```Markup\n{}```", response)).await?;

    Ok(())
}
