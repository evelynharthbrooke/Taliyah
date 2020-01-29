use itertools::Itertools;

use reqwest::blocking::Client;
use reqwest::Url;

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[derive(Debug, Deserialize)]
struct Response {
    languages: Vec<Language>,
    total: Language,
}

#[derive(Debug, Deserialize)]
pub struct Language {
    name: String,
    files: usize,
    lines: usize,
    code: usize,
    comments: usize,
    blanks: usize,
}

#[command("sloc")]
#[description = "Fetches the source lines of code for a GitHub Repository. **Note**: Does not work with large repositories."]
#[usage = "<username> <repository>"]
#[min_args(2)]
#[aliases("tokei")]
pub fn sloc(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.is_empty() {
        message.channel_id.send_message(&context, |m| {
            m.embed(|e| {
                e.title("Error: No repository details provided.");
                e.description("You didn't provide repository details. Please provide them and try again.");
                e.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    let repository_owner = arguments.single::<String>()?;
    let repository_name = arguments.rest().to_string();

    let mut msg = message.channel_id.send_message(&context, |message| {
        message.content(format!("Getting statistics for `{}/{}`, please wait...", repository_owner, repository_name))
    })?;

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;
    let url = Url::parse(format!("https://tokei.now.sh/{}/{}", repository_owner, repository_name).as_str())?;
    let request: Response = client.get(url).send()?.json()?;

    let mut language_string: String = String::new();
    
    let title = format!("**Code statistics for repository `{}/{}`**:", repository_owner, repository_name);

    language_string.push_str(title.as_str());
    language_string.push_str("\n\n");

    let languages = request.languages.iter().map(|language: &Language| {
        let name = language.name.as_str();
        let files = language.files;
        let lines = language.lines;
        let code = language.code;
        let comments = language.comments;
        let blank_lines = language.blanks;

        format!(
            "**{}**: {} files, {} total lines, {} code lines, {} comments, {} blank lines",
            name, files, lines, code, comments, blank_lines
        )
    }).join("\n");

    language_string.push_str(languages.as_str());
    language_string.push_str("\n\n");

    let total_name = request.total.name;
    let total_lines = request.total.lines;
    let total_files = request.total.files;
    let total_code_lines = request.total.code;
    let total_comments = request.total.comments;
    let total_blank_lines = request.total.blanks;

    let total = format!(
        "**{}**: {} files, {} lines, {} code lines, {} comments, {} blank lines",
        total_name, total_files, total_lines, total_code_lines, total_comments, total_blank_lines
    );

    language_string.push_str(total.as_str());

    msg.edit(&context, |message| message.content(language_string))?;

    return Ok(());
}
