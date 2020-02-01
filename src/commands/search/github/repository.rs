use crate::utilities::color_utils;

use byte_unit::Byte;

use chrono::Utc;

use graphql_client::{GraphQLQuery, Response};

use reqwest::blocking::Client;

use serenity::client::Context;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;

use serenity::model::prelude::Message;

use serenity::utils::Colour;

type DateTime = chrono::DateTime<Utc>;
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schemas/github.graphql",
    query_path = "src/graphql/queries/github/repository.graphql",
    response_derives = "Debug"
)]
struct Repository;

#[command]
#[description("Displays information about a specified GitHub repository")]
#[aliases("repo", "repository")]
pub fn repository(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.is_empty() {
        message.channel_id.send_message(&context, move |m| {
            m.embed(move |e| {
                e.title("Error: No repository details provided.");
                e.description("You did not provide any repository details. Please provide them and then try again.")
            })
        })?;
        return Ok(());
    }

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let token: String = std::env::var("GITHUB_KEY").expect("No API key detected");
    let mut color: Colour = Colour::new(0x0033_3333);
    let client = Client::builder().user_agent(user_agent).build()?;
    let endpoint = "https://api.github.com/graphql";

    let query = Repository::build_query(repository::Variables {
        owner: arguments.single::<String>()?,
        name: arguments.single::<String>()?,
    });

    let resp: Response<repository::ResponseData> = client.post(endpoint).bearer_auth(token).json(&query).send()?.json()?;
    let resp_data: repository::ResponseData = resp.data.expect("missing response data");

    let repository = resp_data.repository.unwrap();
    let name = repository.name_with_owner;
    let url = repository.url;
    let stars = repository.stargazers.total_count;
    let forks = repository.fork_count;
    let created = repository.created_at.format("%A, %B %e, %Y @ %l:%M %P");
    let updated = repository.updated_at.format("%A, %B %e, %Y @ %l:%M %P");

    let website = if !repository.homepage_url.as_ref().unwrap().is_empty() {
        format!("[Click here]({})", repository.homepage_url.as_ref().unwrap())
    } else {
        "No website is available.".to_string()
    };

    let disk_usage = match repository.disk_usage {
        Some(usage) => {
            let bytes_in_kb = byte_unit::n_kb_bytes!(usage as u128);
            let bytes = Byte::from_bytes(bytes_in_kb);
            let friendly_bytes = bytes.get_appropriate_unit(false);
            friendly_bytes.format(2)
        }
        None => "No disk usage data is available.".to_string(),
    };

    let description = match repository.description {
        Some(description) => format!("{}\n\n", description),
        None => "".to_string(),
    };

    let language = match repository.primary_language {
        Some(language) => {
            let code: &str = language.color.as_ref().unwrap();

            match color_utils::RGB::from_hex_code(&code) {
                Ok(rgb) => color = Colour::from_rgb(rgb.r, rgb.g, rgb.b),
                Err(_) => println!("{} isn't a valid color code...", code),
            }

            language.name
        }
        None => "No language is available.".to_string(),
    };

    let code_of_conduct = match repository.code_of_conduct {
        Some(conduct) => {
            if conduct.url.is_none() {
                conduct.name
            } else {
                format!("[{}]({})", conduct.name, conduct.url.unwrap())
            }
        }
        None => "No code of conduct is available.".to_string(),
    };

    let owner = repository.owner.login;
    let owner_url = repository.owner.url;
    let owner_avatar = repository.owner.avatar_url;

    let license = match repository.license_info {
        Some(license) => {
            if license.name == "Other" {
                license.name
            } else {
                format!("[{}]({})", license.name, license.url.unwrap())
            }
        }
        None => "No license available.".to_string(),
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title(name);
            embed.url(url);
            embed.thumbnail(owner_avatar);
            embed.color(color);
            embed.description(format!(
                "{}\
                **Owner**: [{}]({})\n\
                **License**: {}\n\
                **Language**: {}\n\
                **Website**: {}\n\
                **Code of Conduct**: {}\n\
                **Created on**: {}\n\
                **Last updated**: {}\n\
                **Disk usage**: {}\n\
                **Star Count**: {}\n\
                **Fork Count**: {}\n",
                description, owner, owner_url, license, language, website, code_of_conduct, created, updated, disk_usage, stars, forks
            ));
            embed.footer(|footer| footer.text("Powered by the GitHub GraphQL API."))
        })
    })?;

    Ok(())
}
