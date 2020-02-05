use crate::VoiceManager;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;
use serenity::voice;

#[command]
#[description = "Begins playback of audio in a voice channel."]
#[only_in(guilds)]
fn play(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            message.channel_id.say(&context.http, "Must provide a URL to a video or audio")?;

            return Ok(());
        }
    };

    let guild_id = match context.cache.read().guild_channel(message.channel_id) {
        Some(channel) => channel.read().guild_id,
        None => {
            message.channel_id.say(&context, "Error finding channel info")?;
            return Ok(());
        }
    };

    let manager_lock = context.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();

    if let Some(handler) = manager.get_mut(guild_id) {
        let source = match voice::ytdl(&url) {
            Ok(source) => source,
            Err(why) => {
                println!("Error starting source: {:?}", why);
                message.channel_id.say(&context, "Error starting playback...Please try again later.")?;
                return Ok(());
            }
        };

        message.channel_id.say(&context, "Beginning song playback.")?;
        handler.play(source);
    } else {
        let guild = message.guild(&context.cache).unwrap();
        let guild_id = guild.read().id;
        let channel_id = guild.read().voice_states.get(&message.author.id).and_then(|channel| channel.channel_id);

        let channel = match channel_id {
            Some(channel) => channel,
            None => {
                message.channel_id.say(&context, format!("You are not in a voice channel, **{}**.", message.author.name))?;
                return Ok(());
            }
        };

        let name = channel.name(&context.cache).unwrap();

        if manager.join(guild_id, channel).is_some() {
            message.channel_id.say(&context, format!("Joined the voice channel **{}**, beginning playback.", name))?;
            let handler = manager.get_mut(guild_id).unwrap();
            let source = match voice::ytdl(&url) {
                Ok(source) => source,
                Err(why) => {
                    println!("Error starting source: {:?}", why);
                    message.channel_id.say(&context, "Error starting playback. Please try again later.")?;
                    return Ok(());
                }
            };
            handler.play(source);
        } else {
            message.channel_id.say(&context, format!("Error joining **{}**. Please try again later.", name))?;
        };
    }

    Ok(())
}
