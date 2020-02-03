use crate::utilities::parse_user;

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::gateway::Activity;
use serenity::model::prelude::Message;

#[command]
#[description = "Shows yours or another user's Spotify status."]
pub fn status(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.read().guild(guild_id).ok_or("Unable to retrieve guild")?;
    let member = if message.mentions.is_empty() {
        if args.is_empty() {
            message.member(&context).ok_or("Could not find member.")?
        } else {
            match parse_user(&args.rest(), Some(&guild_id), Some(&context)) {
                Some(i) => guild_id.member(&context, i)?,
                None => return Ok(()),
            }
        }
    } else {
        guild_id.member(&context, message.mentions.first().ok_or("Failed to get user mentioned.")?)?
    };

    let user = member.user.read();
    let guild = cached_guild.read();

    if !guild.presences.get(&user.id).is_none() {
        let presence = guild.presences.get(&user.id).unwrap();

        if presence.activity.is_none() {
            message.channel_id.say(&context, format!("**{}** does not have an activity active.", &user.name))?
        } else {
            if !presence.activities.iter().filter(|a| a.name == "Spotify").collect::<Vec<&Activity>>().is_empty() {
                let activities = presence.activities.iter().filter(|a| a.name == "Spotify").collect::<Vec<&Activity>>();
                let activity = activities.first().unwrap();

                let assets = activity.assets.as_ref().unwrap();
                let song = activity.details.as_ref().unwrap();
                let artists = activity.state.as_ref().unwrap();
                let album = assets.large_text.as_ref().unwrap();
                let uri = activity.sync_id.as_ref().unwrap();
                let url = format!("https://open.spotify.com/track/{}", uri);
                let mut artist_string = artists.to_string();

                let timestamp_start = activity.timestamps.as_ref().unwrap().start.unwrap() as i64 / 1000;
                let timestamp_end = activity.timestamps.as_ref().unwrap().end.unwrap() as i64 / 1000;
                let start = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_start, 0), Utc).timestamp();
                let end = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_end, 0), Utc).timestamp();

                let length = if end - start < 60 {
                    NaiveDateTime::from_timestamp(end - start, 0).format("%-S seconds")
                } else if end - start > 3600 { // this might be a redundant check...but might as well have it
                    NaiveDateTime::from_timestamp(end - start, 0).format("%-H hours, %-M minutes, %-S seconds")
                } else {
                    NaiveDateTime::from_timestamp(end - start, 0).format("%-M minutes, %-S seconds")
                };

                if artists.contains(';') {
                    let replacer = artist_string.replace(";", ",");
                    let commas = replacer.matches(", ").count();
                    let rfind = artist_string.rfind(';').unwrap();
                    let (left, right) = replacer.split_at(rfind);

                    let format_string = if commas >= 2 {
                        format!("{}{}", left, right.replace(",", ", &"))
                    } else {
                        format!("{} {}", left, right.replace(",", "&"))
                    };

                    artist_string.clear();
                    artist_string.push_str(&format_string);
                }

                let artwork = assets.large_image.as_ref().unwrap().replace("spotify:", "");
                let artwork_url = format!("https://i.scdn.co/image/{}", artwork);

                message.channel_id.send_message(&context, |message| {
                    message.embed(|embed| {
                        embed.author(|author| {
                            author.icon_url("https://upload.wikimedia.org/wikipedia/commons/7/71/Spotify.png");
                            author.name(format!("Spotify status for {}", &user.name))
                        });
                        embed.colour(0x001D_B954);
                        embed.thumbnail(artwork_url);
                        embed.field("Song", format!("[{}]({})", song, url), false);
                        embed.field("Artists", artist_string, true);
                        embed.field("Album", album, true);
                        embed.field("Song length", length, false)
                    })
                })?
            } else {
                message.channel_id.say(&context, format!("**{}** is not currently playing anything on Spotify.", &user.name))?
            }
        }
    } else {
        message.channel_id.say(&context, format!("**{}** is currently offline / doesn't have a presence.", &user.name))?
    };

    Ok(())
}
