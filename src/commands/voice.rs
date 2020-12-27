use anyhow::{bail, Error, Result};

use crate::data::Lavalink;

use lavalink_rs::LavalinkClient;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    prelude::Context
};

use std::sync::Arc;
use tokio::process::Command;
use tracing::instrument;

#[instrument(skip(context))]
async fn _join(context: &Context, message: &Message) -> Result<String, Error> {
    let guild = message.guild(&context.cache).await.unwrap();
    let guild_id = guild.id;
    let channel_id = guild.voice_states.get(&message.author.id).and_then(|state| state.channel_id);
    let channel = match channel_id {
        Some(channel) => channel,
        None => {
            message.reply(context, "You are not currently connected to a voice channel.").await?;
            bail!("No voice channel connected.");
        }
    };

    let manager = songbird::get(context).await.unwrap().clone();
    let (_, handler) = manager.join_gateway(guild_id, channel).await;

    match handler {
        Ok(connection) => {
            let mut data = context.data.write().await;
            let client_lock = data.get_mut::<Lavalink>().expect("Expected an active Lavalink client in the TypeMap.");
            client_lock.lock().await.create_session(guild_id, &connection).await?;
            Ok(channel.name(context).await.unwrap())
        }
        Err(why) => {
            message.reply(context, "Error joining voice channel.").await?;
            bail!("Error encountered while joining voice channel: {}", why)
        }
    }
}

#[command]
#[aliases("connect", "summon")]
#[description = "Connects the bot to the user's current voice channel."]
#[only_in(guilds)]
async fn join(context: &Context, message: &Message) -> CommandResult {
    let channel = _join(context, message).await?;
    message.reply(context, &format!("Joined voice channel **{}**.", channel)).await?;
    Ok(())
}

#[command]
#[aliases("disconnect", "unsummon")]
#[description = "Leaves the current voice channel."]
#[only_in(guilds)]
async fn leave(context: &Context, message: &Message) -> CommandResult {
    let guild = message.guild(&context.cache).await.unwrap();
    let guild_id = guild.id;

    if !guild.voice_states.contains_key(&message.author.id) {
        message.channel_id.say(context, "You are not in a voice channel.").await?;
        return Ok(());
    }

    let manager = songbird::get(context).await.unwrap().clone();

    if let Err(e) = manager.remove(guild_id).await {
        message.reply(context, format!("Failed: {:?}", e)).await?;
    }

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_data.lock().await;

    lava_client.destroy(guild_id).await?;
    lava_client.nodes.remove(&guild_id.0);

    if let Some(pos) = lava_client.loops.iter().position(|x| *x == guild_id.0) {
        lava_client.loops.remove(pos);
    }

    message.reply(context, "Disconnected from voice.").await?;

    Ok(())
}

#[command]
#[min_args(1)]
#[aliases(playlist, playplaylist, play_list, pl, playl, plist)]
#[only_in(guilds)]
async fn play_playlist(context: &Context, message: &Message, args: Args) -> CommandResult {
    let query = args.message().to_string();

    let guild_id = match context.cache.guild_channel(message.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            message.channel_id.say(context, "Error finding channel info").await?;

            return Ok(());
        }
    };

    let manager = songbird::get(context).await.unwrap().clone();

    if let Some(_handler_lock) = manager.get(guild_id) {
        let lava_client_lock = {
            let data_read = context.data.read().await;
            data_read.get::<Lavalink>().unwrap().clone()
        };

        let lava_client = lava_client_lock.lock().await;

        let mut iter = 0;
        let query_information = loop {
            iter += 1;
            let res = lava_client.auto_search_tracks(&query).await?;

            if res.tracks.is_empty() {
                if iter == 5 {
                    message.channel_id.say(context, "Couldn't find any videos matching this search query.").await?;
                    return Ok(());
                }
            } else {
                break res;
            }
        };

        drop(lava_client);

        for track in query_information.clone().tracks {
            LavalinkClient::play(guild_id, track.clone())
                .requester(message.author.id)
                .queue(Arc::clone(&lava_client_lock))
                .await?;
        }

        message
            .channel_id
            .send_message(context, |m| {
                m.content("Added playlist to queue.");
                m.embed(|e| {
                    e.title(query_information.playlist_info.name.unwrap());
                    e.url(query);
                    e.footer(|f| f.text(format!("Requested by: {}", &message.author.name)))
                })
            })
            .await?;
    } else {
        message.reply(context, "Please connect the bot to voice first before attempting to play anything.").await?;
    }

    Ok(())
}

#[command]
#[min_args(1)]
#[aliases(p)]
#[description = "Plays music in a voice channel."]
#[only_in(guilds)]
async fn play(context: &Context, message: &Message, args: Args) -> CommandResult {
    let mut query = args.message().to_string();

    let guild_id = match context.cache.guild_channel(message.channel_id).await {
        Some(channel) => channel.guild_id,
        None => {
            message.reply(context, "Error finding channel information.").await?;
            return Ok(());
        }
    };

    let _ = _join(context, message).await?;

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();
    let lava_client = lava_data.lock().await;

    let mut iter = 0;
    let mut already_checked = false;

    let info = loop {
        iter += 1;
        let res = lava_client.auto_search_tracks(&query).await?;

        if res.tracks.is_empty() {
            if iter == 5 {
                if !already_checked {
                    already_checked = true;

                    let output: std::process::Output = Command::new("youtube-dl").arg("-g").arg(&query).output().await?;

                    if !output.stdout.is_empty() {
                        let stdout = String::from_utf8(output.stdout)?;
                        let mut stdout = stdout.split('\n').collect::<Vec<_>>();
                        stdout.pop();
                        let url = stdout.last().unwrap();

                        iter = 0;
                        query = url.to_string();

                        continue;
                    }
                }
                message.channel_id.say(&context, "Could not find any video of the search query.").await?;
                return Ok(());
            }
        } else {
            if query.starts_with("http") && res.tracks.len() > 1 {
                message.channel_id.say(context, "If you would like to play the playlist, use `play_playlist`.").await?;
            }
            break res;
        }
    };

    drop(lava_client);

    LavalinkClient::play(guild_id, info.tracks[0].clone())
        .requester(message.author.id)
        .queue(Arc::clone(&lava_data))
        .await?;

    message
        .channel_id
        .send_message(context, |m| {
            m.content("Added to queue:");
            m.embed(|e| {
                e.title(&info.tracks[0].info.as_ref().unwrap().title);
                e.image(format!("https://i.ytimg.com/vi/{}/mqdefault.jpg", info.tracks[0].info.as_ref().unwrap().identifier));
                e.url(&info.tracks[0].info.as_ref().unwrap().uri);
                e.footer(|f| f.text(format!("Requested by: {}", &message.author.name)));
                e.field("Uploaded by", &info.tracks[0].info.as_ref().unwrap().author, true);
                e.field(
                    "Length",
                    format!("{}:{}", info.tracks[0].info.as_ref().unwrap().length / 1000 % 3600 / 60, {
                        let x = info.tracks[0].info.as_ref().unwrap().length / 1000 % 3600 % 60;
                        if x < 10 {
                            format!("0{}", x)
                        } else {
                            x.to_string()
                        }
                    }),
                    true
                );
                e
            })
        })
        .await?;

    Ok(())
}

#[command]
#[description = "Pauses the currently playing song."]
#[only_in(guilds)]
async fn pause(context: &Context, message: &Message) -> CommandResult {
    let guild = message.guild(&context.cache).await.unwrap();
    let guild_id = guild.id;

    if !guild.voice_states.contains_key(&message.author.id) {
        message.channel_id.say(context, "You are not in a voice channel.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();

    let mut lava_client = lava_data.lock().await;
    lava_client.set_pause(guild_id, true).await?;

    message.reply(context, "Paused playback; use the resume command to resume playback.").await?;

    Ok(())
}

#[command]
#[description = "Resumes playback of the current song in a given guild."]
#[only_in(guilds)]
async fn resume(context: &Context, message: &Message) -> CommandResult {
    let guild = message.guild(&context.cache).await.unwrap();
    let guild_id = guild.id;

    if !guild.voice_states.contains_key(&message.author.id) {
        message.channel_id.say(context, "You are not in a voice channel.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();

    let mut lava_client = lava_data.lock().await;
    lava_client.set_pause(guild_id, false).await?;

    message.reply(context, "Resumed playback.").await?;

    Ok(())
}

#[command]
#[description = "Stops anything currently playing in a given guild."]
#[only_in(guilds)]
async fn stop(context: &Context, message: &Message) -> CommandResult {
    let guild = message.guild(&context.cache).await.unwrap();
    let guild_id = guild.id;

    if !guild.voice_states.contains_key(&message.author.id) {
        message.channel_id.say(context, "You are not in a voice channel.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();

    let mut lava_client = lava_data.lock().await;
    lava_client.stop(guild_id).await?;

    message.reply(context, "Stopped current playback.").await?;

    Ok(())
}

#[command]
#[description = "Sets the given voice volume for the current voice channel."]
#[only_in(guilds)]
#[min_args(1)]
#[max_args(1)]
async fn volume(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let guild = message.guild(&context.cache).await.unwrap();
    let volume = arguments.single::<u16>().unwrap();

    if !(guild.voice_states.contains_key(&message.author.id)) {
        message.channel_id.say(context, "You are not in a voice channel.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();

    let mut lava_client = lava_data.lock().await;
    lava_client.volume(guild.id, volume).await?;

    message.reply(context, format!("Set the current volume to {}%.", volume)).await?;

    Ok(())
}

#[command]
#[aliases(np, nowplaying)]
#[description = "Displays information about the currently playing track."]
#[only_in(guilds)]
async fn now_playing(context: &Context, message: &Message) -> CommandResult {
    let guild = message.guild(context).await.unwrap();
    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();
    let lava_client = lava_data.lock().await;

    let node = match lava_client.nodes.get(&guild.id.as_u64()) {
        Some(node) => node,
        None => {
            message.reply(context, "Not connected to a node. Please connect me to one first.").await?;
            return Ok(());
        }
    };

    let now_playing = node.now_playing.as_ref();

    if let Some(t) = now_playing {
        let requested_by = if let Some(user) = t.requester {
            user.to_serenity().to_user(context).await.unwrap_or_default().name
        } else {
            "Unknown".to_string()
        };

        let track_info = t.track.info.as_ref().unwrap();

        message
            .channel_id
            .send_message(context, |m| {
                m.embed(|embed| {
                    embed.title(format!("Now Playing: {}", &track_info.title));
                    embed.image(format!("https://i.ytimg.com/vi/{}/default.jpg", track_info.identifier));
                    embed.url(&track_info.uri);
                    embed.footer(|footer| footer.text(format!("Requested by: {}", requested_by)));
                    embed.field("Uploaded by", &track_info.author, true);
                    embed.field(
                        "Current Position",
                        format!(
                            "{}:{} - {}:{}",
                            track_info.position / 1000 % 3600 / 60,
                            {
                                let x = track_info.position / 1000 % 3600 % 60;
                                if x < 10 {
                                    format!("0{}", x)
                                } else {
                                    x.to_string()
                                }
                            },
                            track_info.length / 1000 % 3600 / 60,
                            {
                                let x = track_info.length / 1000 % 3600 % 60;
                                if x < 10 {
                                    format!("0{}", x)
                                } else {
                                    x.to_string()
                                }
                            }
                        ),
                        true
                    );
                    embed
                });
                m
            })
            .await?;
    } else {
        message.reply(context, "Nothing is currently playing.").await?;
    }

    Ok(())
}

#[command]
#[description = "Check a guild's current queue of songs."]
#[only_in(guilds)]
async fn queue(context: &Context, message: &Message) -> CommandResult {
    let guild = message.guild(context).await.unwrap();

    if !guild.voice_states.contains_key(&message.author.id) {
        message.channel_id.say(context, "You are not in a voice channel.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();
    let lava_client = lava_data.lock().await;

    let node = match lava_client.nodes.get(&guild.id.as_u64()) {
        Some(node) => node,
        None => {
            message.channel_id.say(context, "Not connected to a channel / node. Please connect me to one first.").await?;
            return Ok(());
        }
    };

    let queue = &node.queue;

    if queue.is_empty() && node.now_playing.is_none() {
        message.channel_id.say(context, "No songs are currently in the queue.").await?;
        return Ok(());
    }

    message
        .channel_id
        .send_message(context, |m| {
            m.embed(|e| {
                e.title(format!("Track Queue for {}", guild.name));
                e.color(0x8c4ed9);

                if let Some(t) = node.now_playing.as_ref() {
                    let track_info = t.track.info.as_ref().unwrap();
                    e.field("Now playing", format!("[{}]({})", track_info.title, track_info.uri), false);
                }

                if queue.len() > 1 {
                    let mut queue_string = String::new();

                    // Only take 10 songs total from the queue count; Discord has a
                    // somewhat-restrictive embed character limit that restricts the
                    // total amount of characters that can be in an embed, especially
                    // in the embed description.
                    for (num, t) in queue.iter().enumerate().skip(1).take(10) {
                        let track_info = t.track.info.as_ref().unwrap();
                        queue_string.push_str(&format!("{}. {}\n", num, track_info.title));
                    }

                    e.field("Next 10 Songs", queue_string, false);
                }

                e.footer(|footer| footer.text(format!("{} song(s) are in the queue.", queue.len())))
            })
        })
        .await?;

    Ok(())
}

#[command]
#[aliases(cque, clearqueue, clearque, cqueue)]
#[description = "Clears the guild's current queue of tracks."]
#[only_in(guilds)]
async fn clear_queue(context: &Context, message: &Message) -> CommandResult {
    let guild = message.guild(context).await.unwrap();

    if !guild.voice_states.contains_key(&message.author.id) {
        message.reply(context, "You are not in a voice channel.").await?;
        return Ok(());
    }

    let data = context.data.read().await;
    let lava_data = data.get::<Lavalink>().cloned().unwrap();
    let mut lava_client = lava_data.lock().await;

    let node = match lava_client.nodes.get_mut(&guild.id.as_u64()) {
        Some(node) => node,
        None => {
            message.reply(context, "Not connected to a node. Please connect me to one first.").await?;
            return Ok(());
        }
    };

    if !node.queue.is_empty() {
        node.queue = vec![];
        message.reply(context, "The queue has been successfully emptied.").await?;
    } else {
        message.reply(context, "The queue is already empty.").await?;
    }

    Ok(())
}
