use crate::data::ConfigContainer;
use crate::utils::parsing::parse_user;

use chrono::{DateTime, NaiveDateTime, Utc};

use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::{gateway::Activity, prelude::Message}
};

#[command]
#[description = "Shows yours or another user's Spotify status."]
#[aliases("np", "nowplaying")]
async fn status(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.guild(guild_id).ok_or("Unable to retrieve guild")?.clone();
    let member = if message.mentions.is_empty() {
        if arguments.is_empty() {
            message.member(&context).await.map_err(|_| "Could not find member.")?
        } else {
            match parse_user(arguments.rest(), guild_id, context).await {
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
    if denied_ids.contains(&user.id.get()) {
        message.reply(context, "This user's status cannot be viewed; they are in the deny list.").await?;
        return Ok(());
    }

    let name = &user.name;

    if guild.presences.get(&user.id).is_some() {
        let presence = guild.presences.get(&user.id).unwrap();
        if presence.activities.first().is_none() {
            message.reply(&context, format!("**{name}** does not have an active activity.")).await?
        } else {
            let activities = presence.activities.iter().filter(|a| a.name == "Spotify").collect::<Vec<&Activity>>();
            if !activities.is_empty() {
                let activity = activities.first().unwrap();
                let assets = activity.assets.as_ref().unwrap();
                let track = activity.details.as_ref().unwrap();
                let album = assets.large_text.as_ref().unwrap();
                let mut artists = activity.state.as_ref().unwrap().to_string();
                let icon = "https://upload.wikimedia.org/wikipedia/commons/7/71/Spotify.png";
                let id = activity.sync_id.as_ref().unwrap();
                let url = format!("https://open.spotify.com/track/{id}");

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
                    let replacer = artists.replace(';', ",");
                    let commas = replacer.matches(", ").count();
                    let rfind = artists.rfind(';').unwrap();
                    let (left, right) = replacer.split_at(rfind);
                    let format_string = if commas >= 2 {
                        format!("{left}{}", right.replace(',', ", &"))
                    } else {
                        format!("{left} {}", right.replace(',', "&"))
                    };

                    artists.clear();
                    artists.push_str(&format_string);
                }

                let artwork = assets.large_image.as_ref().unwrap().replace("spotify:", "");
                let artwork_url = format!("https://i.scdn.co/image/{artwork}");

                let embed = CreateEmbed::new()
                    .author(CreateEmbedAuthor::new(format!("Now playing on Spotify for {name}:")).icon_url(icon))
                    .title(track)
                    .colour(0x001D_B954)
                    .url(url)
                    .description(format!("**{artists}** | {album}"))
                    .thumbnail(artwork_url)
                    .footer(CreateEmbedFooter::new(format!("Length: {length}")));

                message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?
            } else {
                message.reply(&context, format!("**{name}** is not currently playing anything on Spotify.")).await?
            }
        }
    } else {
        message.reply(&context, format!("**{name}** is currently offline / doesn't have a presence.")).await?
    };

    Ok(())
}
