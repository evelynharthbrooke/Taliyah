//! Last.fm scrobbles command

use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

use crate::utils::{format_int, get_profile_field, net_utils::*};

#[command]
#[description("Retrieves a given Last.fm user's scrobble count.")]
#[usage("<user>")]
async fn scrobbles(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let user = if !arguments.rest().is_empty() {
        if !message.mentions.is_empty() {
            get_profile_field(context, "user_lastfm_id", message.mentions.first().unwrap().id).await.unwrap()
        } else {
            arguments.single::<String>().unwrap()
        }
    } else {
        match get_profile_field(context, "user_lastfm_id", message.author.id).await {
            Ok(user) => user,
            Err(_) => match arguments.single::<String>() {
                Ok(argument) => argument,
                Err(_) => {
                    message.channel_id.say(context, "No username found. Please set one via `profile set` or provide one.").await?;
                    return Ok(());
                }
            }
        }
    };

    let mut client = get_lastfm_client(context).await;
    let user_info = client.user_info(&user).await.send().await.unwrap().user;
    let name = &message.author.name;
    let scrobbles = format_int(user_info.scrobbles.parse::<u64>().unwrap());

    message.channel_id.say(context, format!("**{name}** has **{scrobbles}** scrobbles on Last.fm.")).await?;

    Ok(())
}
