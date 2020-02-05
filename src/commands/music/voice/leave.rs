use crate::VoiceManager;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::channel::Message;

#[command]
#[description = "Leaves the currently active voice channel, if the bot is in one."]
#[only_in(guilds)]
fn leave(context: &mut Context, message: &Message) -> CommandResult {
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

    let channel_name = channel.name(&context.cache).unwrap();
    let manager_lock = context.data.read().get::<VoiceManager>().cloned().expect("Expected VoiceManager in ShareMap.");
    let mut manager = manager_lock.lock();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        manager.remove(guild_id);
        message.channel_id.say(&context, format!("Stopped any current playback and left **{}**.", channel_name))?;
    } else {
        message.channel_id.say(&context, "I am not in a voice channel.")?;
    }

    Ok(())
}
