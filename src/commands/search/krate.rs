use crate::utilities::format_int;

use chrono::DateTime;
use chrono::Utc;

use reqwest::blocking::Client;

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "crate")]
    krate: Crate,
    versions: Vec<Version>,
    keywords: Vec<Keyword>,
}

#[derive(Deserialize, Debug)]
struct Crate {
    id: String,
    name: String,
    updated_at: DateTime<Utc>,
    versions: Vec<usize>,
    keywords: Vec<String>,
    categories: Vec<String>,
    created_at: DateTime<Utc>,
    downloads: usize,
    recent_downloads: usize,
    max_version: String,
    newest_version: String,
    description: Option<String>,
    homepage: Option<String>,
    repository: Option<String>,
    exact_match: bool,
}

#[derive(Deserialize, Debug)]
struct Version {
    id: usize,
    #[serde(rename = "crate")]
    crate_name: String,
    num: String,
    dl_path: String,
    readme_path: String,
    updated_at: DateTime<Utc>,
    created_at: DateTime<Utc>,
    downloads: usize,
    yanked: bool,
    license: String,
    crate_size: Option<usize>,
    published_by: Option<User>,
    audit_actions: Option<Vec<AuditAction>>,
}

#[derive(Deserialize, Debug)]
struct User {
    id: usize,
    login: String,
    name: Option<String>,
    avatar: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct AuditAction {
    action: String,
    user: User,
    time: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
struct Keyword {
    id: String,
    keyword: String,
    created_at: DateTime<Utc>,
    crates_cnt: usize,
}

#[command("crate")]
#[description = "Looks up a crate on crates.io and displays information about it."]
#[usage = "<crate name>"]
#[aliases("crates", "cratesio", "cio")]
pub fn krate(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |message| {
            message.embed(|embed| {
                embed.title("Error: Invalid crate name provided.");
                embed.color(0x00FF_0000);
                embed.description("You have provided an invalid crate name. Please try again.")
            })
        })?;
        return Ok(());
    }

    let krate = arguments.single::<String>()?;

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;

    let request_url = format!("https://crates.io/api/v1/crates/{}", krate);
    let response = client.get(&request_url).send()?;
    let result: Response = response.json()?;

    let name = result.krate.name;
    let image = "https://raw.githubusercontent.com/rust-lang/crates.io/master/public/assets/Cargo-Logo-Small.png";
    let url = format!("https://crates.io/crates/{}", name);

    let keywords = if result.krate.keywords.is_empty() {
        "No keywords are available for this crate.".to_string()
    } else {
        result.krate.keywords.join(", ")
    };

    let created_at = result.krate.created_at.format("%B %e, %Y - %I:%M %p");
    let last_updated = result.krate.updated_at.format("%B %e, %Y - %I:%M %p");
    let latest_version = result.krate.newest_version;
    let max_version = result.krate.max_version;
    let recent_downloads = format_int(result.krate.recent_downloads);
    let downloads = format_int(result.krate.downloads);

    let homepage = match result.krate.homepage {
        Some(homepage) => homepage,
        None => "No homepage is available for this crate.".to_string(),
    };

    let repository = match result.krate.repository {
        Some(repository) => repository,
        None => "No repository is available for this crate.".to_string(),
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(name);
                author.url(url);
                author.icon_url(image)
            });
            embed.description(format!(
                "\
                **Homepage**: {}\n\
                **Repository**: {}\n\
                **Keywords**: {}\n\
                **Latest version**: {} ({} max)\n\
                **Creation date**: {}\n\
                **Last updated**: {}\n\
                **Recent downloads**: {}\n\
                **Total downloads**: {}\n",
                homepage, repository, keywords, latest_version, max_version, created_at, last_updated, recent_downloads, downloads
            ))
        })
    })?;

    Ok(())
}
