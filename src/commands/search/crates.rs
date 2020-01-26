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
    package: Crate,
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
    name: String,
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

#[command]
#[description = "Looks up a crate on crates.io and displays information about it."]
#[usage = "<crate name>"]
#[aliases("crates", "cratesio", "cio")]
pub fn crates(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |m| {
            m.embed(|e| {
                e.title("Error: Invalid crate name provided.");
                e.description("You have provided an invalid crate name. Please try again.");
                e.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    let crate_name = arguments.single::<String>()?.to_string();

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).build()?;

    let request_url = format!("https://crates.io/api/v1/crates/{}", crate_name.trim());
    let response = client.get(&request_url).send()?;

    let crate_result: Response = response.json()?;
    let crate_name = crate_result.package.name;
    let crates_img = "https://raw.githubusercontent.com/rust-lang/crates.io/master/public/assets/Cargo-Logo-Small.png";
    let crate_url = format!("https://crates.io/crates/{}", crate_name);

    let crate_keywords = if crate_result.package.keywords.is_empty() {
        "No keywords are available for this crate.".to_string()
    } else {
        crate_result.package.keywords.join(", ")
    };

    let crate_crated_at = crate_result.package.created_at.format("%B %e, %Y - %I:%M %p");
    let crate_last_updated = crate_result.package.updated_at.format("%B %e, %Y - %I:%M %p");
    let crate_latest_version = crate_result.package.newest_version;
    let crate_recent_downloads = format_int(crate_result.package.recent_downloads);
    let crate_downloads = format_int(crate_result.package.downloads);

    let crate_homepage = match crate_result.package.homepage {
        Some(homepage) => homepage.to_string(),
        None => "No homepage is available for this crate.".to_string(),
    };

    let crate_repository = match crate_result.package.repository {
        Some(repository) => repository.to_string(),
        None => "No repository is available for this crate.".to_string(),
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(crate_name);
                author.url(crate_url);
                author.icon_url(crates_img)
            });
            embed.description(format!(
                "\
                **Homepage**: {}\n\
                **Repository**: {}\n\
                **Keywords**: {}\n\
                **Latest version**: {}\n\
                **Creation date**: {}\n\
                **Last updated**: {}\n\
                **Recent downloads**: {}\n\
                **Total downloads**: {}\n",
                crate_homepage, crate_repository, crate_keywords, crate_latest_version, 
                crate_crated_at, crate_last_updated, crate_recent_downloads, crate_downloads
            ))
        })
    })?;

    return Ok(());
}
