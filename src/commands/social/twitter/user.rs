use crate::data::ReqwestContainer;
use crate::read_config;
use crate::utils::format_int;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serenity::{
    builder::{CreateEmbed, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

#[derive(Deserialize)]
struct User {
    pub data: UserData
}

#[derive(Deserialize)]
struct UserData {
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

#[derive(Deserialize)]
struct UserPublicMetrics {
    pub followers_count: u64, // The amount of people that follow the given user.
    pub following_count: u64, // The amount of people that the given user is following.
    pub tweet_count: u64      // The total amount of times the given user has Tweeted.
}

#[derive(Deserialize)]
struct UserTweets {
    pub data: Option<Vec<UserTweet>>
}

#[derive(Deserialize)]
struct UserTweet {
    pub text: String // the text
}

#[command]
#[min_args(1)]
#[max_args(1)]
/// Displays information about a given user on Twitter.
async fn user(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    let user_fields = [("user.fields", "created_at,protected,location,public_metrics,description,verified,profile_image_url")];
    let tweet_fields = [("max_results", "5"), ("exclude", "retweets,replies")];
    let user: String = args.single().unwrap();

    let client = context.data.read().await.get::<ReqwestContainer>().cloned().unwrap();
    let config = read_config("config.toml");
    let bearer = config.api.social.twitter.core.bearer_token;

    let mut endpoint = format!("https://api.twitter.com/2/users/by/username/{user}");
    let mut request = client.get(&endpoint).bearer_auth(&bearer).query(&user_fields).send().await?;

    let user = request.json::<User>().await?.data;
    let id = &user.id;
    let name = &user.name;
    let handle = user.username;
    let joined = user.created_at.format("%B %-e, %Y").to_string();
    let protected = if user.protected { "Yes" } else { "No" }.to_string();
    let location = user.location;
    let url = format!("https://twitter.com/{handle}");
    let description = &user.description;
    let avatar = &user.profile_image_url.replace("normal", "400x400").to_string();
    let following = format_int(user.public_metrics.following_count);
    let followers = format_int(user.public_metrics.followers_count);
    let tweets = format_int(user.public_metrics.tweet_count);

    endpoint = format!("https://api.twitter.com/2/users/{id}/tweets");
    request = client.get(&endpoint).bearer_auth(&bearer).query(&tweet_fields).send().await?;

    let tweets_response: UserTweets = request.json().await?;
    let latest_tweet = match tweets_response.data {
        Some(tweets) => tweets.first().unwrap().text.clone(),
        None => "Tweet not available.".to_string()
    };

    let builder = CreateMessage::new().embed(
        CreateEmbed::new()
            .title(format!("{name}{verified}", verified = if user.verified { " \\✔️" } else { "" }))
            .url(url)
            .thumbnail(avatar)
            .color(0x00acee)
            .description(description)
            .fields(vec![
                ("Username", handle, true),
                ("Join Date", joined, true),
                ("Protected", protected, true),
                ("Location", if location.is_some() { location.unwrap() } else { "None".to_string() }, true),
                ("Following", following, true),
                ("Followers", followers, true),
                ("Tweets", tweets, true),
                ("Latest Tweet", latest_tweet, false),
            ])
            .footer(CreateEmbedFooter::new(format!("User ID: {id} | Powered by Twitter.")))
    );

    message.channel_id.send_message(context, builder).await?;

    Ok(())
}
