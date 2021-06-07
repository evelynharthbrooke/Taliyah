use crate::data::ConfigContainer;
use crate::utils::parsing_utils::parse_user;

use chrono::{DateTime, NaiveDateTime, Utc};

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{gateway::Activity, interactions::ButtonStyle, prelude::Message}
};

#[command]
#[description = "Shows yours or another user's Spotify status."]
#[aliases("np", "nowplaying")]
pub async fn status(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.guild(guild_id).await.ok_or("Unable to retrieve guild")?;
    let member = if message.mentions.is_empty() {
        if arguments.is_empty() {
            message.member(&context).await.map_err(|_| "Could not find member.")?
        } else {
            match parse_user(arguments.rest(), Some(&guild_id), Some(context)).await {
                Some(user_id) => guild_id.member(&context, user_id).await?,
                None => return Ok(())
            }
        }
    } else {
        guild_id.member(&context, message.mentions.first().ok_or("Failed to get user mentioned.")?).await?
    };

    let user = member.user;
    let guild = cached_guild;

    let data = context.data.read().await;
    let config = data.get::<ConfigContainer>().unwrap();
    let denied_ids = &config.bot.denylist.spotify.ids;
    if denied_ids.contains(&user.id.as_u64()) {
        message.channel_id.say(context, "You cannot view this user's Spotify status as they are in the deny list.").await?;
        return Ok(())
    }

    if !guild.presences.get(&user.id).is_none() {
        let presence = guild.presences.get(&user.id).unwrap();

        if presence.activities.first().is_none() {
            message.channel_id.say(&context, format!("**{}** does not have an active activity.", &user.name)).await?
        } else {
            let activities = presence.activities.iter().filter(|a| a.name == "Spotify").collect::<Vec<&Activity>>();
            if !activities.is_empty() {
                let activity = activities.first().unwrap();
                let assets = activity.assets.as_ref().unwrap();
                let track = activity.details.as_ref().unwrap();
                let album = assets.large_text.as_ref().unwrap();
                let mut artists = activity.state.as_ref().unwrap().to_string();
                let logo = "https://upload.wikimedia.org/wikipedia/commons/7/71/Spotify.png";
                let id = activity.sync_id.as_ref().unwrap();
                let url = format!("https://open.spotify.com/track/{}", id);

                let timestamp_start = activity.timestamps.as_ref().unwrap().start.unwrap() as i64 / 1000;
                let timestamp_end = activity.timestamps.as_ref().unwrap().end.unwrap() as i64 / 1000;
                let start = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_start, 0), Utc).timestamp();
                let end = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_end, 0), Utc).timestamp();

                let length = if end - start < 60 {
                    NaiveDateTime::from_timestamp(end - start, 0).format("%-S seconds")
                } else if end - start > 3600 {
                    // Some audio tracks on Spotify can actually go past the minutes mark, so
                    // if that ends up being the case, lets have a timestamp that shows hours
                    // as well.
                    NaiveDateTime::from_timestamp(end - start, 0).format("%-H hour(s), %-M minutes, %-S seconds")
                } else {
                    NaiveDateTime::from_timestamp(end - start, 0).format("%-M minutes, %-S seconds")
                };

                if artists.contains(';') {
                    let replacer = artists.replace(";", ",");
                    let commas = replacer.matches(", ").count();
                    let rfind = artists.rfind(';').unwrap();
                    let (left, right) = replacer.split_at(rfind);

                    let format_string = if commas >= 2 {
                        format!("{}{}", left, right.replace(",", ", &"))
                    } else {
                        format!("{} {}", left, right.replace(",", "&"))
                    };

                    artists.clear();
                    artists.push_str(&format_string);
                }

                let artwork = assets.large_image.as_ref().unwrap().replace("spotify:", "");
                let artwork_url = format!("https://i.scdn.co/image/{}", artwork);

                let status_fields = vec![
                    ("Track", track.to_string(), false),
                    ("Artist(s)", artists, false),
                    ("Album", album.to_string(), false),
                    ("Duration", length.to_string(), false),
                ];

                message
                    .channel_id
                    .send_message(&context, |message| {
                        message.embed(|embed| {
                            embed.author(|author| {
                                author.icon_url(logo);
                                author.name(format!("Spotify status for {}", &user.name));
                                author
                            });
                            embed.colour(0x001D_B954);
                            embed.thumbnail(artwork_url);
                            embed.fields(status_fields);
                            embed
                        });
                        message.components(|comps| {
                            comps.create_action_row(|row| {
                                row.create_button(|b| b.label(format!("Play {track} on Spotify")).style(ButtonStyle::Link).url(url));
                                row
                            });
                            comps
                        });
                        message
                    })
                    .await?
            } else {
                message
                    .channel_id
                    .say(&context, format!("**{}** is not currently playing anything on Spotify.", &user.name))
                    .await?
            }
        }
    } else {
        message
            .channel_id
            .say(&context, format!("**{}** is currently offline / doesn't have a presence.", &user.name))
            .await?
    };

    Ok(())
}
