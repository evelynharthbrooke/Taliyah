use lastfm_rs::{
    error::{Error, LastFMErrorResponse::InvalidParameters},
    Client
};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    utils::Colour
};

use crate::{
    read_config,
    utils::{get_profile_field, parsing_utils::parse_user},
    DatabasePool
};

const LASTFM_USER_BASE: &str = "https://www.lastfm/user";

#[command]
#[usage = "<user> or <blank>"]
#[sub_commands(set)]
#[only_in(guilds)]
/// Shows the profile of a given user.
///
/// To set your profile parameters, use the set command.
pub async fn profile(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let member = if message.mentions.is_empty() {
        if arguments.is_empty() {
            message.member(&context).await.map_err(|_| "Could not find member.")?
        } else {
            match parse_user(&arguments.rest(), Some(&guild_id), Some(&context)).await {
                Some(i) => guild_id.member(&context, i).await?,
                None => return Ok(())
            }
        }
    } else {
        guild_id.member(&context, message.mentions.first().ok_or("Failed to get user mentioned.")?).await?
    };

    let color = if member.colour(cache).await.is_none() {
        Colour::new(0x00FF_FFFF)
    } else {
        member.colour(cache).await.unwrap()
    };

    let user_name = member.user.tag();
    let user_id = member.user.id;

    let name = get_profile_field(context, "user_name", user_id).await?;
    let location = get_profile_field(context, "user_location", user_id).await?;
    let gender = get_profile_field(context, "user_gender", user_id).await?;
    let pronouns = get_profile_field(context, "user_pronouns", user_id).await?;
    let lastfm = get_profile_field(context, "user_lastfm_id", user_id).await?;
    let lastfm_url = format!("[{}]({}/{})", lastfm, LASTFM_USER_BASE, lastfm);

    let profile_fields = vec![
        ("Name", name, true),
        ("Location", location, true),
        ("Gender", gender, true),
        ("Pronouns", pronouns, true),
        ("Last.fm", lastfm_url, false),
    ];

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.name(format!("Profile for user {}", user_name));
                    author.icon_url(&member.user.face());
                    author
                });
                embed.color(color);
                embed.fields(profile_fields);
                embed
            })
        })
        .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
/// Sets your profile parameters. Available parameters are below.
///
/// `name`: Sets your display name. This is used in multiple places; mainly the profile and Last.fm commands.
/// `location`: Sets your location. Only use this if you don't mind listing your country or even province / state.
/// `gender`: Sets your gender. No forced gender options, so use what you want as long as its Male / Female, or non-binary.
/// `pronouns`: Sets your pronouns. This doesn't have any forced pronoun options, however please stick to the normal ones.
/// `lastfm`: Sets your Last.fm username. Used for the Last.fm command for listing Last.fm statistics.
pub async fn set(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();
    let property = arguments.single::<String>()?;
    let value = arguments.rest();
    let config = read_config("config.toml");
    let user_id = message.author.id.0 as i64;

    match property.as_str() {
        "location" => {
            if value.is_empty() {
                message.channel_id.say(&context, "You did not provide a location. Please provide one!").await?;
                return Ok(());
            };

            sqlx::query("UPDATE profile_data SET user_location = $1 WHERE user_id = $2;")
                .bind(&value)
                .bind(&user_id)
                .execute(&pool)
                .await
                .unwrap();

            message.channel_id.say(&context, format!("Your location has been set to `{}`.", &value)).await?;
        }
        "lastfm" => {
            if value.is_empty() {
                message.channel_id.say(&context, "You did not provide your Last.fm username. Please provide one!").await?;
                return Ok(());
            };

            let api_key = config.api.music.lastfm.api_key;
            let mut client: Client = Client::new(&api_key);

            match client.user_info(&value).await.send().await {
                Ok(_) => (),
                Err(e) => {
                    if let Error::LastFMError(InvalidParameters(e)) = e {
                        if let "User not found" = e.message.as_str() {
                            message.channel_id.say(context, "You cannot use this as your username.").await?;
                        }
                    }
                }
            }

            sqlx::query("UPDATE profile_data SET user_lastfm_id = $1 WHERE user_id = $2")
                .bind(&value)
                .bind(&user_id)
                .execute(&pool)
                .await
                .unwrap();

            message.channel_id.say(&context, format!("Your Last.fm username has been set to `{}`.", &value)).await?;
        }
        "name" => {
            if value.is_empty() {
                message.channel_id.say(&context, "You did not provide a name. Please provide one!").await?;
                return Ok(());
            };

            sqlx::query("UPDATE profile_data SET user_name = $1 WHERE user_id = $2")
                .bind(&value)
                .bind(&user_id)
                .execute(&pool)
                .await
                .unwrap();

            message.channel_id.say(&context, format!("Your name has been set to {}.", &value)).await?;
        }
        "gender" => {
            if value.is_empty() {
                message.channel_id.say(&context, "You did not provide your gender. Please provide it.").await?;
                return Ok(());
            };

            sqlx::query("UPDATE profile_data SET user_gender = $1 WHERE user_id = $2")
                .bind(&value)
                .bind(&user_id)
                .execute(&pool)
                .await
                .unwrap();

            message.channel_id.say(&context, format!("Your gender has been set to {}.", &value)).await?;
        }
        "pronouns" => {
            if value.is_empty() {
                message.channel_id.say(context, "You did not provide any pronouns. Please provide them.").await?;
                return Ok(());
            }

            sqlx::query("UPDATE profile_data SET user_pronouns = $1 WHERE user_id = $2")
                .bind(&value)
                .bind(&user_id)
                .execute(&pool)
                .await
                .unwrap();

            message.channel_id.say(&context, format!("Your pronouns have been set to {}.", &value)).await?;
        }
        _ => {
            message.channel_id.say(&context, "That is not a valid profile property.").await?;
        }
    }

    Ok(())
}
