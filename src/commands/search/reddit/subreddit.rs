use crate::utilities::color_utils::RGB;
use crate::utilities::format_int;
use crate::utilities::format_string;

use chrono::prelude::*;

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
pub struct Response {
    pub data: SubredditData,
}

#[derive(Debug, Deserialize)]
pub struct ForbiddenResponse {
    reason: String,                     // The reason for the 403 Forbidden error.
    message: String,                    // The status message.
    quarantine_message: Option<String>, // The reasoning behind the subreddit quarantine, if available.
    error: u64,                         // The status code of the response.
}

#[derive(Debug, Deserialize)]
pub struct SubredditData {
    pub restrict_posting: bool,         // Whether or not the ability to post on the subreddit has been restricted.
    pub wiki_enabled: bool,             // Whether or not the wiki feature is enabled on the subreddit.
    pub display_name: String,           // The non-prefixed display name of the subreddit, e.g. unixporn.
    pub title: Option<String>,          // The customized title of the subreddit, if available.
    pub primary_color: String,          // The primary color of the subreddit, if available.
    pub active_user_count: isize,       // The amount of users currently active on a subreddit.
    pub icon_img: Option<String>,       // The icon thumbnail of the subreddit.
    pub display_name_prefixed: String,  // The prefixed name of the subreddit, e.g. r/unixporn or r/tifu.
    pub subscribers: isize,             // The amount of users who have subscribed (or joined) the subreddit.
    pub name: String,                   // The Reddit object ID for the subreddit.
    pub quarantine: bool,               // Whether or not the subreddit is quarantined.
    pub public_description: String,     // The public description of the subreddit.
    pub community_icon: Option<String>, // The community icon for the subreddit.
    pub over18: bool,                   // Whether or not the subreddit is rated for users age 18 and over.
    pub created_utc: f64,               // The creation date of the subreddit, as a Unix timestamp.
}

impl SubredditData {
    /// Gets the Reddit API created UTC date object as a valid
    /// DateTime<Utc> object.
    fn created_utc(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.created_utc as i64, 0), Utc)
    }
}

#[command("subreddit")]
#[description = "Looks up a crate on crates.io and displays information about it."]
#[usage = "<subreddit name>"]
#[aliases("sr", "subreddit", "srinfo")]
pub fn subreddit(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
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

    let subreddit: String = arguments.single()?;

    let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), ", v", env!("CARGO_PKG_VERSION"));
    let client = Client::builder().user_agent(user_agent).redirect(Policy::none()).build()?;

    let request_url = format!("https://api.reddit.com/r/{}/about", subreddit);
    let response = client.get(&request_url).send()?;
    let status_code = response.status();

    match status_code {
        StatusCode::FORBIDDEN => {
            let response: ForbiddenResponse = response.json()?;
            let reason = response.reason;

            if reason == "quarantined".to_string() {
                let quarantined = response.quarantine_message.unwrap();
                message.channel_id.send_message(&context, |message| {
                    message.embed(|embed| {
                        embed.title("This subreddit has been quarantined.");
                        embed.color(0x00FF_0000);
                        embed.description(format_string(quarantined).replace("It is", "This subreddit has been"))
                    })
                })?;
                return Ok(());
            } else {
                message.channel_id.send_message(&context, |message| {
                    message.embed(|embed| {
                        embed.title("Could Not Access Resource");
                        embed.color(0x00FF_0000);
                        embed.description(
                            "Reddit has transmitted a 403 Forbidden error, meaning there \
                            was an issue in trying to get the result for this request. Please \
                            try again later.",
                        )
                    })
                })?;
                return Ok(());
            }
        }
        StatusCode::FOUND | StatusCode::NOT_FOUND => {
            message.channel_id.send_message(&context, |message| {
                message.embed(|embed| {
                    embed.title("Subreddit Not Found Or Redirect Request Made");
                    embed.color(0x00FF_0000);
                    embed.description(
                        "Reddit either could not find this subreddit, or tried to \
                        redirect to the Reddit Search API, usually indicating that the \
                        subreddit you tried searching for does not exist or could not \
                        be found in Reddit's database. Please try searching for a different \
                        subreddit.",
                    )
                })
            })?;
            return Ok(());
        }
        _ => (),
    }

    let result: Response = response.json()?;

    let created = result.data.created_utc().format("%A, %B %e, %Y @ %l:%M %P");
    let active_users = format_int(result.data.active_user_count as usize);
    let subscribers = format_int(result.data.subscribers as usize);
    let icon = result.data.icon_img.unwrap();
    let image = result.data.community_icon.unwrap();
    let object_id = result.data.name;
    let quarantined = if result.data.quarantine { "Yes" } else { "No" };
    let description = format_string(result.data.public_description);
    let over_eighteen = if result.data.over18 { "Yes" } else { "No" };
    let posts_restricted = if result.data.restrict_posting { "Yes" } else { "No" };
    let title = result.data.title.unwrap();
    let name = result.data.display_name_prefixed;
    let url = format!("https://reddit.com/{}", name);

    let color = if result.data.primary_color.is_empty() {
        Colour::BLURPLE
    } else {
        match RGB::from_hex_code(&result.data.primary_color) {
            Ok(rgb) => Colour::from_rgb(rgb.r, rgb.g, rgb.b),
            Err(_) => Colour::BLURPLE,
        }
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(name);
                author.icon_url(icon);
                author.url(url)
            });
            embed.color(color);
            embed.thumbnail(image);
            embed.description(format!(
                "\
                {}\n\n\
                **Title:** {}\n\
                **Active Users:** {}\n\
                **Subscribers:** {}\n\
                **Quarantined:** {}\n\
                **Over Eighteen?** {}\n\
                **Posting Restricted?** {}\n\
                **Created:** {}\n\
                ",
                description, title, active_users, subscribers, quarantined, over_eighteen, posts_restricted, created
            ));
            embed.footer(|footer| footer.text(format!("Subreddit ID: {}", object_id)))
        })
    })?;

    Ok(())
}
