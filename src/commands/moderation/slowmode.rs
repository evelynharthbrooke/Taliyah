use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::{Channel::Guild, Message}
};

use tracing::error;

#[command("slowmode")]
#[usage = "<num of secs>"]
/// Sets the slowmode rate for a channel.
///
/// NOTE: Setting a slowmode rate for a specific channel is not yet supported,
/// moderators and server owners / administrators have to send the command in
/// the appropriate channel they want to apply slowmode to.
pub async fn slowmode(context: &Context, message: &Message, mut arguments: Args) -> CommandResult {
    let slowmode_content = if let Ok(slowmode_rate) = arguments.single::<u64>() {
        if let Err(why) = message.channel_id.edit(&context, |c| c.slow_mode_rate(slowmode_rate)).await {
            error!("Error setting channel's slowmode rate: {:?}", why);
            format!("Failed to set slowmode to `{}` seconds.", slowmode_rate)
        } else if slowmode_rate == 0 {
            "Successfully cleared the channel's slowmode rate.".to_string()
        } else {
            format!("Successfully set the slowmode rate to `{}` seconds.", slowmode_rate)
        }
    } else if let Some(Guild(channel)) = message.channel_id.to_channel_cached(&context).await {
        match channel.slow_mode_rate {
            Some(rate) => {
                if rate == 0 {
                    format!("Slowmode is not currently set in this channel.")
                } else {
                    format!("Current slowmode rate is set to `{}` seconds.", rate)
                }
            }
            None => format!("Slowmode is not available for this channel type.")
        }
    } else {
        "Failed to find channel in cache.".to_string()
    };

    if let Err(why) = message.channel_id.say(&context, slowmode_content).await {
        error!("Error sending message: {:?}", why);
    }

    Ok(())
}
