use chrono::Utc;

use graphql_client::{GraphQLQuery, Response};

use reqwest::blocking::Client;

use serenity::client::Context;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;

use serenity::model::prelude::Message;

type DateTime = chrono::DateTime<Utc>;
type URI = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graphql/schemas/github.graphql",
    query_path = "src/graphql/queries/github/UserQuery.graphql",
    response_derives = "Debug"
)]
struct UserQuery;

#[command]
#[description("Displays information about a specified user on GitHub.")]
pub fn user(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.send_message(&context, move |m| {
            m.embed(move |e| {
                e.title("Error: No username provided.");
                e.description("You did not provide a username. Please enter one and then try again.")
            })
        })?;
        return Ok(());
    }

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    let token: String = std::env::var("GITHUB_KEY").expect("No API key detected");
    let client = Client::builder().user_agent(user_agent).build()?;
    let endpoint = "https://api.github.com/graphql";
    let query = UserQuery::build_query(user_query::Variables { username: args.rest().to_string() });

    let response: Response<user_query::ResponseData> = client.post(endpoint).bearer_auth(token).json(&query).send()?.json()?;
    let response_data: user_query::ResponseData = response.data.expect("missing response data");

    let user = response_data.user.unwrap();
    let username = user.login;
    let url = user.url;
    let avatar = user.avatar_url;
    let created_at = user.created_at.format("%A, %B %e, %Y @ %l:%M %P");
    let following = user.following.total_count;
    let followers = user.followers.total_count;
    let repositories = user.repositories.total_count;
    let real_name = if user.name.as_ref().unwrap().is_empty() { username } else { user.name.unwrap() };

    let location = if !user.location.as_ref().is_none() {
        user.location.as_ref().unwrap().as_str()
    } else {
        "No location available."
    };

    let biography = if !user.bio.as_ref().is_none() {
        if !user.bio.as_ref().unwrap().is_empty() {
            let mut bio = user.bio.as_ref().unwrap().to_string();
            bio.push_str("\n\n");
            bio
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

    let status = if user.status.as_ref().is_none() {
        "No status available."
    } else {
        let status = user.status.as_ref().unwrap();
        status.message.as_ref().unwrap().as_str()
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(real_name);
                author.icon_url(avatar);
                author.url(url)
            });
            embed.color(0x0033_3333);
            embed.description(format!(
                "{}\
                **__Basic details__**:\n\
                **Status**: {}\n\
                **Joined GitHub**: {}\n\
                **Repositories**: {}\n\
                **Location**: {}\n\
                **Following**: {}\n\
                **Followers**: {}\n\
                ",
                biography, status, created_at, repositories, location, following, followers,
            ));
            embed.footer(|footer| footer.text("Powered by the GitHub GraphQL API."))
        })
    })?;

    Ok(())
}
