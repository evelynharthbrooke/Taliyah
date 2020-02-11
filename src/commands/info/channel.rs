use crate::utilities::parsing_utils::parse_channel;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[aliases("channelinfo", "cinfo")]
#[description("Displays information about a server channel.")]
#[only_in(guilds)]
pub fn channel(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.read().guild(guild_id).ok_or("Unable to retrieve guild")?;
    let guild = cached_guild.read();
    let guild_name = &guild.name;
    let guild_icon = guild.icon_url().unwrap();

    if arguments.is_empty() {
        message.channel_id.send_message(&context, |message| message.content("You didn't provide a channel name."))?;
        return Ok(());
    }

    let channel_name: &str = arguments.rest();
    let channel_id = match parse_channel(channel_name, Some(&guild_id), Some(&context)) {
        Some(channel_id) => channel_id,
        None => {
            message.channel_id.send_message(&context, |message| {
                message.embed(|embed| {
                    embed.title("Error: Unknown channel provided.");
                    embed.description(format!(
                        "This channel does not exist as part of **{}**. Please \
                        try a different channel name.",
                        guild_name
                    ))
                })
            })?;
            return Ok(());
        }
    };

    let cached_channel = channel_id.to_channel_cached(&context).unwrap();
    let guild_channel = cached_channel.guild().unwrap();
    let channel = guild_channel.read();

    let channel_name = &channel.name;
    let channel_position = &channel.position;
    let channel_id = &channel.id;
    let channel_nsfw = &channel.is_nsfw();

    let channel_topic = match &channel.topic {
        Some(topic) => {
            if !topic.is_empty() {
                format!("{}\n\n", topic)
            } else {
                "".to_string()
            }
        }
        None => "".to_string(),
    };

    let channel_kind = match channel.kind.name() {
        "text" => "Text Channel",
        "voice" => "Voice Channel",
        "category" => "Channel Category",
        "news" => "News Channel",
        "store" => "Store Channel",
        _ => "Unrecognized channel type",
    };

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(channel_name);
                author.icon_url(guild_icon)
            });
            embed.color(serenity::utils::Colour::BLURPLE);
            embed.description(format!(
                "{}\
                **__Channel Attributes:__**\n\
                **NSFW:** {}\n\
                **Kind:** {}\n\
                **Position:** {}\n\
                ",
                channel_topic, channel_nsfw, channel_kind, channel_position
            ));
            embed.footer(|footer| footer.text(format!("Channel ID: {}", channel_id)))
        })
    })?;

    Ok(())
}
