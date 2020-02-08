use crate::utilities::color_utils::RGB;
use crate::utilities::format_int;
use crate::utilities::format_string;

use chrono::prelude::*;

use itertools::Itertools;

use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use reqwest::StatusCode;

use serde::Deserialize;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;
use serenity::utils::Colour;

#[derive(Debug, Deserialize)]
pub struct AboutResponse {
    pub data: UserData,
}

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub is_employee: bool,        // Whether or not the user is a Reddit employee.
    pub icon_img: Option<String>, // The icon image pertaining to the user.
    pub name: String,             // The name of the user.
    pub created: f64,             // When the user was registered according to Reddit's host servers.
    pub has_subscribed: bool,     // Whether or not the user has subscribed to any subreddits.
    pub hide_from_robots: bool,   // Whether or not the user is hidden from being crawled by search engines.
    pub created_utc: f64,         // When the user was registered according to Coordinated Universal Time.
    pub link_karma: usize,        // The user's total karma gained from posts they have made.
    pub comment_karma: usize,     // The user's total karma gained from posting comments.
    pub is_gold: bool,            // Whether or not the user is a Reddit Gold subscriber.
    pub is_mod: bool,             // Whether or not the user moderates a subreddit.
    pub verified: bool,           // Whether or not the user is verified.
    pub subreddit: UserSubreddit, // The user's own subreddit, if available.
    pub has_verified_email: bool, // Whether or not the user has a verified email address.
    pub id: String,               // The reddit object ID of the user.
}

#[derive(Debug, Deserialize)]
pub struct UserSubreddit {
    pub title: String,                 // The title of the user's subreddit.
    pub restrict_posting: bool,        // Whether or not posting is restricted in the subreddit.
    pub user_is_banned: Option<bool>,  // Whether or not the user has been banned from posting in the subreddit.
    pub display_name_prefixed: String, // The user's prefixed name.
    pub public_description: String,    // The description of the user's subreddit.
    pub subreddit_type: String,        // The type of the subreddit, e.g. User or Public.
}

impl UserData {
    /// Gets the Reddit API created UTC date object as a valid
    /// DateTime<Utc> object.
    fn created_utc(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.created_utc as i64, 0), Utc)
    }
}

#[derive(Debug, Deserialize)]
pub struct TrophiesResponse {
    pub data: Trophies,
}

#[derive(Debug, Deserialize)]
pub struct Trophies {
    pub trophies: Vec<Trophy>,
}

#[derive(Debug, Deserialize)]
pub struct Trophy {
    pub data: TrophyData, // The data containing information about the trophies.
}

#[derive(Debug, Deserialize)]
pub struct TrophyData {
    pub icon_70: String,             // The icon of the trophy.
    pub name: String,                // The name of the trophy.
    pub award_id: Option<String>,    // The ID of the trophy.
    pub description: Option<String>, // The description of the trophy.
}

#[command("user")]
#[description = "Looks up a crate on crates.io and displays information about it."]
#[usage = "<user>"]
#[aliases("uinfo", "user")]
pub fn user(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    if arguments.rest().is_empty() {
        message.channel_id.send_message(&context, |m| {
            m.embed(|e| {
                e.title("Error: Invalid subreddit name provided.");
                e.description("You have provided an invalid subreddit name. Please try again.");
                e.color(0x00FF_0000)
            })
        })?;
        return Ok(());
    }

    let user: String = arguments.single()?;

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), ", v", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).redirect(Policy::none()).build()?;

    let about_endpoint = format!("https://api.reddit.com/user/{}/about", user);
    let trophies_endpoint = format!("https://api.reddit.com/user/{}/trophies", user);
    let about_response = client.get(&about_endpoint).send()?;
    let trophies_response = client.get(&trophies_endpoint).send()?;
    let about_status_code = about_response.status();

    match about_status_code {
        StatusCode::FOUND | StatusCode::NOT_FOUND => {
            message.channel_id.send_message(&context, |message| {
                message.embed(|embed| {
                    embed.title("User Not Found Or Redirect Request Made");
                    embed.color(0x00FF_0000);
                    embed.description(
                        "Reddit either could not find this user, or tried to redirect \
                        to the Reddit Search API, usually indicating that the user \
                        you tried searching for does not exist or could not be found \
                        in Reddit's database. Please try searching for a different user.",
                    )
                })
            })?;
            return Ok(());
        }
        _ => (),
    }

    let about_result: AboutResponse = about_response.json()?;
    let trophy_result: TrophiesResponse = trophies_response.json()?;

    let name = &about_result.data.name;
    let link_karma = about_result.data.link_karma;
    let verified = about_result.data.verified;
    let is_employee = about_result.data.is_employee;
    let created = &about_result.data.created_utc().format("%A, %B %e, %Y @ %l:%M %P");
    let trophies = trophy_result.data.trophies.iter().map(|t| &t.data.name).join(" / ");
    let comment_karma = about_result.data.comment_karma;
    let combined_karma = link_karma + comment_karma;
    let object_id = about_result.data.id;
    let url = format!("https://reddit.com/user/{}", name);

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(name);
                author.url(url)
            });
            embed.color(Colour::TEAL);
            embed.description(format!(
                "\
                **Joined Reddit:** {}\n\
                **Verified:** {}\n\
                **Reddit employee:** {}\n\
                **Link Karma:** {}\n\
                **Comment Karma:** {}\n\
                **Total Karma:** {}\n\
                **Trophies:** {}\n",
                created,
                verified,
                is_employee,
                format_int(link_karma as usize),
                format_int(comment_karma as usize),
                format_int(combined_karma as usize),
                trophies,
            ));
            embed.footer(|footer| footer.text(format!("User Object ID: {}", object_id)))
        })
    })?;

    return Ok(());
}
