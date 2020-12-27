use crate::utils::parsing_utils::parse_channel;

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

#[command]
#[aliases("channelinfo", "cinfo")]
#[description("Displays information about a server channel.")]
#[only_in(guilds)]
pub async fn channel(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.guild(guild_id).await.ok_or("Unable to retrieve guild")?;
    let guild_icon = cached_guild.icon_url().unwrap();

    let channel_name = if arguments.is_empty() {
        message.channel_id.name(&context).await.unwrap()
    } else {
        arguments.rest().to_string()
    };

    let channel_id = match parse_channel(&channel_name, Some(&guild_id), Some(&context)).await {
        Some(channel_id) => channel_id,
        None => {
            message
                .channel_id
                .send_message(&context, |message| {
                    message.embed(|embed| {
                        embed.title("Error: Unknown channel provided.");
                        embed.description("This channel does not exist. Please try a different channel name.")
                    })
                })
                .await?;
            return Ok(());
        }
    };

    let cached_channel = channel_id.to_channel_cached(&context).await.unwrap();
    let guild_channel = cached_channel.guild().unwrap();

    let channel_name = &guild_channel.name;

    let channel_category = match guild_channel.category_id {
        Some(category) => category.name(&context).await.unwrap(),
        None => "No category available".to_string()
    };

    let channel_position = &guild_channel.position;
    let channel_id = &guild_channel.id;
    let channel_nsfw = &guild_channel.is_nsfw();

    let channel_topic = match &guild_channel.topic {
        Some(topic) => {
            if !topic.is_empty() {
                format!("{}\n\n", topic)
            } else {
                "".to_string()
            }
        }
        None => "".to_string()
    };

    let channel_kind = match guild_channel.kind.name() {
        "text" => "Text Channel",
        "voice" => "Voice Channel",
        "category" => "Channel Category",
        "news" => "News Channel",
        "store" => "Store Channel",
        _ => "Unrecognized channel type"
    };

    message
        .channel_id
        .send_message(&context, |message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.name(channel_name);
                    author.icon_url(guild_icon)
                });
                embed.color(serenity::utils::Colour::BLURPLE);
                embed.description(channel_topic);
                embed.fields(vec![
                    ("Category", channel_category, true),
                    ("Position", channel_position.to_string(), true),
                    ("Kind", channel_kind.to_string(), true),
                    ("NSFW", channel_nsfw.to_string(), true),
                ]);
                embed.footer(|footer| footer.text(format!("Channel ID: {}", channel_id)))
            })
        })
        .await?;

    Ok(())
}
