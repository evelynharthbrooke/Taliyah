use crate::commands::info::changelog::commits_query::CommitsQueryRepositoryRefTargetOn::Commit;

use graphql_client::{GraphQLQuery, Response};

use itertools::Itertools;

use reqwest::blocking::Client;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

type GitObjectID = String;
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schemas/github.graphql",
    query_path = "src/graphql/queries/github/CommitsQuery.graphql",
    response_derives = "Debug"
)]
struct CommitsQuery;

#[command]
#[description("Displays the most recent commits for the bot.")]
pub fn changelog(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let token: String = std::env::var("GITHUB_KEY").expect("No API key detected");
    let client = Client::builder().user_agent(user_agent).build()?;
    let endpoint = "https://api.github.com/graphql";

    let mut commits: i64 = 10;

    match arguments.single::<i64>() {
        Ok(value) => commits = value,
        Err(_e) => {}
    };

    let query = CommitsQuery::build_query(commits_query::Variables {
        owner: "KamranMackey".to_string(),
        name: "Ellie".to_string(),
        branch: "master".to_string(),
        commits,
    });

    let response: Response<commits_query::ResponseData> = client.post(endpoint).bearer_auth(token).json(&query).send()?.json()?;
    let response_data: commits_query::ResponseData = response.data.expect("missing response data");

    let repository = response_data.repository.unwrap();
    let target_on = &repository.ref_.as_ref().unwrap().target.on;
    let url = repository.url;
    let branch = &repository.ref_.as_ref().unwrap().name;
    let commits = match target_on {
        Commit(c) => {
            let history = &c.history;
            let edges = history.edges.as_ref();
            edges.unwrap().iter().map(|commit| {
                let commit = commit.as_ref().unwrap();
                let node = &commit.node.as_ref().unwrap();
                let title = &node.message_headline;
                let url = &node.url[0..52];
                let sha = &node.oid[0..7];
                format!("[`{}`]({}) {}", sha, url, title)
            }).join("\n") 
        },
        _ => "".to_string()
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title(format!("Recent commits on {}", branch));
            embed.url(format!("{}/commits/{}", url, branch));
            embed.description(commits);
            embed.footer(|footer| footer.text("Powered by the GitHub GraphQL API."))
        }) 
    })?;

    Ok(())
}