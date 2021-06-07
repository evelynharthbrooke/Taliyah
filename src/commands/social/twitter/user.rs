use serde::{Deserialize, Serialize};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use chrono::{DateTime, Utc};

use crate::data::ReqwestContainer;
use crate::read_config;
use crate::utils::format_int;

use std::fmt::Write;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub data: UserData
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserData {
    pub id: String,                        // The user's Twitter identifier.
    pub name: String,                      // The user's display name.
    pub username: String,                  // The user's username / handle.
    pub created_at: DateTime<Utc>,         // The user's date of when they joined Twitter, in UTC.
    pub protected: bool,                   // The user's protected account status, e.g. whether or not tweets are private.
    pub location: Option<String>,          // The user's provided location, if available.
    pub description: String,               // The user's description / bio.
    pub verified: bool,                    // The user's verified status.
    pub profile_image_url: String,         // The user's profile image.
    pub public_metrics: UserPublicMetrics  // The user's publicly available metrics, such as followers / following.
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserPublicMetrics {
    pub followers_count: u64, // The amount of people that follow the given user.
    pub following_count: u64, // The amount of people that the given user is following.
    pub tweet_count: u64      // The total amount of times the given user has Tweeted.
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTweets {
    pub data: Option<Vec<UserTweet>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserTweet {
    pub text: String
}

#[command]
#[min_args(1)]
#[max_args(1)]
/// Displays information about a given user on Twitter.
pub async fn user(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let user: String = args.single().unwrap();

    let config = read_config("config.toml");
    let bearer = config.api.social.twitter.bearer_token;

    let user_fields = [("user.fields", "created_at,protected,location,public_metrics,description,verified,profile_image_url")];
    let tweet_fields = [("max_results", "5"), ("exclude", "retweets,replies")];

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();

    let mut endpoint = format!("https://api.twitter.com/2/users/by/username/{}", user);
    let mut request = client.get(&endpoint).bearer_auth(&bearer).query(&user_fields).send().await?;

    let user: UserData = request.json::<User>().await?.data;
    let user_id = &user.id;
    let user_name = &user.name;
    let user_handle = user.username;
    let user_joined = user.created_at.format("%B %-e, %Y").to_string();
    let user_protected = if user.protected { "Yes" } else { "No" }.to_string();
    let user_location = user.location;
    let user_url = format!("https://twitter.com/{}", user_handle);
    let user_description = &user.description;
    let user_verified = user.verified;
    let user_avatar = &user.profile_image_url.replace("normal", "400x400").to_string();
    let user_followers = format_int(user.public_metrics.followers_count as usize);
    let user_following = format_int(user.public_metrics.following_count as usize);
    let user_tweet_count = format_int(user.public_metrics.tweet_count as usize);

    endpoint = format!("https://api.twitter.com/2/users/{}/tweets", user_id);
    request = client.get(&endpoint).bearer_auth(&bearer).query(&tweet_fields).send().await?;

    let user_tweets_response: UserTweets = request.json().await?;
    let user_latest_tweet = match user_tweets_response.data {
        Some(tweets) => tweets.first().unwrap().text.clone(),
        None => "Tweet not available.".to_string()
    };

    let mut user_header = String::new();

    write!(user_header, "{} {}", user_name, if user_verified { "\\✔️" } else { "" })?;

    let user_fields = vec![
        ("Username", user_handle, true),
        ("Join Date", user_joined, true),
        ("Protected", user_protected, true),
        ("Location", if user_location != None { user_location.unwrap() } else { "None".to_string() }, true),
        ("Following", user_following, true),
        ("Followers", user_followers, true),
        ("Tweets", user_tweet_count, true),
        ("Latest Tweet", user_latest_tweet, false),
    ];

    message
        .channel_id
        .send_message(context, |message| {
            message.embed(|embed| {
                embed.title(user_header);
                embed.url(user_url);
                embed.thumbnail(user_avatar);
                embed.color(0x00acee);
                embed.description(user_description);
                embed.fields(user_fields);
                embed.footer(|footer| footer.text(format!("User ID: {} | Powered by Twitter.", user_id)));
                embed
            })
        })
        .await?;

    Ok(())
}
