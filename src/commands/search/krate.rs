use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::Deserialize;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::{data::ReqwestContainer, utils::format_int};

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "crate")]
    krate: Crate,
    versions: Vec<Version>,
    keywords: Vec<Keyword>,
    categories: Vec<Category>
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
    exact_match: bool
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
    audit_actions: Option<Vec<AuditAction>>
}

#[derive(Deserialize, Debug)]
struct User {
    id: usize,
    login: String,
    name: Option<String>,
    avatar: String,
    url: String
}

#[derive(Deserialize, Debug)]
struct AuditAction {
    action: String,
    user: User,
    time: DateTime<Utc>
}

#[derive(Deserialize, Debug)]
struct Keyword {
    id: String,
    keyword: String,
    created_at: DateTime<Utc>,
    crates_cnt: usize
}

#[derive(Deserialize, Debug)]
struct Category {
    id: String,
    category: String,
    slug: String,
    description: String,
    created_at: DateTime<Utc>,
    crates_cnt: i64
}

#[command("crate")]
#[description = "Looks up a crate on crates.io and displays information about it."]
#[usage = "<crate name>"]
#[aliases("crates", "cratesio", "cio")]
pub async fn krate(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.say(context, "Invalid crate name provided. Please try again.").await?;
        return Ok(());
    }

    let krate = arguments.single::<String>()?;

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let request_url = format!("https://crates.io/api/v1/crates/{}", krate);
    let response = client.get(&request_url).send().await?;
    let result: Response = response.json().await?;

    let crate_name = result.krate.name;
    let crate_url = format!("https://crates.io/crates/{}", crate_name);
    let crate_image = "https://raw.githubusercontent.com/rust-lang/crates.io/master/public/assets/Cargo-Logo-Small.png";

    let crate_description = if result.krate.description.is_none() {
        "".to_string()
    } else {
        result.krate.description.unwrap()
    };

    let crate_homepage = match result.krate.homepage {
        Some(homepage) => format!("[Website]({})", homepage),
        None => "None".to_string()
    };

    let crate_repository = match result.krate.repository {
        Some(repository) => format!("[Repo for {}]({})", crate_name, repository),
        None => "None".to_string()
    };

    let crate_categories = if result.categories.is_empty() {
        "None".to_string()
    } else {
        result.categories.iter().map(|c| c.category.as_str()).join("\n")
    };

    let crate_keywords = if result.keywords.is_empty() {
        "None".to_string()
    } else {
        result.keywords.iter().map(|k| k.keyword.as_str()).join("\n")
    };

    let crate_creation_date = result.krate.created_at.format("%B %e, %Y").to_string();
    let crate_last_updated = result.krate.updated_at.format("%B %e, %Y").to_string();
    let crate_latest_version = result.krate.newest_version;
    let crate_recent_dls = format_int(result.krate.recent_downloads);
    let crate_total_dls = format_int(result.krate.downloads);

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.name(crate_name);
                    author.url(crate_url);
                    author.icon_url(crate_image)
                });
                embed.color(0xe43a25);
                embed.description(crate_description);
                embed.fields(vec![
                    ("Homepage", crate_homepage, true),
                    ("Repository", crate_repository, true),
                    ("Categories", crate_categories, true),
                    ("Keywords", crate_keywords, true),
                    ("Latest Version", crate_latest_version, true),
                    ("Creation Date", crate_creation_date, true),
                    ("Last Updated", crate_last_updated, true),
                    ("Recent DLs", crate_recent_dls, true),
                    ("Total DLs", crate_total_dls, true),
                ]);
                embed.footer(|footer| footer.text("Powered by your friendly neighbourhood crates.io API."));
                embed
            })
        })
        .await?;

    Ok(())
}
