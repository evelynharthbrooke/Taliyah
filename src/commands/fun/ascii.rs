use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::data::ReqwestContainer;

#[command]
#[usage = "<font> <string>"]
#[example = "speed THIS. IS. ASCII."]
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
async fn ascii(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "No ASCII string provided. Please provide one.").await?;
        return Ok(());
    } else if arguments.rest().contains('\u{200B}') {
        message.channel_id.say(context, "Zero width space detected. Don't send these.").await?;
        return Ok(());
    }

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();

    let arg = arguments.rest();
    let font_url = "https://artii.herokuapp.com/fonts_list";
    let font_arg = arg.split_whitespace().next().unwrap_or("");
    let font_request = client.get(font_url).send().await?.text().await?;
    let request = if font_request.split_whitespace().any(|x| x == font_arg) && arg.contains('\u{0020}') {
        let font = &arg[0..font_arg.chars().count()];
        let text = &arg[font_arg.chars().count()..];
        client.get("https://artii.herokuapp.com/make").query(&[("text", text), ("font", font)]).send().await?
    } else {
        client.get("https://artii.herokuapp.com/make").query(&[("text", arg)]).send().await?
    };

    let response = request.text().await?;

    message.channel_id.say(context, format!("```Markup\n{response}```")).await?;

    Ok(())
}
