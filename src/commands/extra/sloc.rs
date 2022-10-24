use crate::data::ReqwestContainer;
use itertools::Itertools;
use serde::Deserialize;
use serenity::{
    builder::EditMessage,
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

#[derive(Debug, Deserialize)]
struct Response {
    languages: Vec<Language>,
    total: Language
}

#[derive(Debug, Deserialize)]
pub struct Language {
    name: String,
    files: u64,
    lines: u64,
    code: u64,
    comments: u64,
    blanks: u64
}

#[command("sloc")]
#[description = "Fetches the source lines of code for a GitHub Repository. **Note**: Does not work with large repositories."]
#[usage = "<username> <repository>"]
#[delimiters("/", " ")]
#[aliases("tokei")]
async fn sloc(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.is_empty() {
        message.channel_id.say(context, "No repository details provided. Please provide them & try again.").await?;
        return Ok(());
    }

    let owner = arguments.single::<String>()?;
    let name = arguments.single::<String>()?;

    let mut msg = message.channel_id.say(context, format!("Getting statistics for `{owner}/{name}`, please wait...")).await?;

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let request: Response = client.get(format!("https://tokei.vercel.app/{owner}/{name}")).send().await?.json().await?;

    let title = format!("**Code statistics for repository `{owner}/{name}`**:");

    let mut language_string: String = String::new();
    language_string.push_str(format!("{title}\n\n").as_str());

    #[rustfmt::skip]
    let languages = request.languages.iter().map(|lang| {
        let name = lang.name.as_str();
        let files = lang.files;
        let lines = lang.lines;
        let code = lang.code;
        let comments = lang.comments;
        let blanks = lang.blanks;
        format!("**{name}**: {files} files, {lines} total lines, {code} code lines, {comments} comments, {blanks} blank lines")
    }).join("\n");

    language_string.push_str(format!("{languages}\n\n").as_str());

    let name = request.total.name;
    let files = request.total.files;
    let lines = request.total.lines;
    let code_lines = request.total.code;
    let comments = request.total.comments;
    let blanks = request.total.blanks;
    let total = format!("**{name}**: {files} files, {lines} lines, {code_lines} code lines, {comments} comments, {blanks} blank lines");
    language_string.push_str(total.as_str());

    msg.edit(&context, EditMessage::new().content(language_string)).await?;

    Ok(())
}
