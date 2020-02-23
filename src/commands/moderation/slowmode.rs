use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::{Channel::Guild, Message};

#[command("slowmode")]
#[usage = "<num of secs>"]
/// Sets the slowmode rate for a channel.
///
/// NOTE: Setting a slowmode rate for a specific channel is not yet supported,
/// moderators and server owners / administrators have to send the command in
/// the appropriate channel they want to apply slowmode to.
pub fn slowmode(context: &mut Context, message: &Message, mut arguments: Args) -> CommandResult {
    let slowmode_content = if let Ok(slowmode_rate) = arguments.single::<u64>() {
        if let Err(why) = message.channel_id.edit(&context, |c| c.slow_mode_rate(slowmode_rate)) {
            println!("Error setting channel's slowmode rate: {:?}", why);
            format!("Failed to set slowmode to `{}` seconds.", slowmode_rate)
        } else if slowmode_rate == 0 {
            "Successfully cleared the channel's slowmode rate.".to_string()
        } else {
            format!("Successfully set the slowmode rate to `{}` seconds.", slowmode_rate)
        }
    } else if let Some(Guild(channel)) = message.channel_id.to_channel_cached(&context) {
        format!("Current slowmode rate is `{}` seconds.", channel.read().slow_mode_rate.unwrap_or(0))
    } else {
        "Failed to find channel in cache.".to_string()
    };

    if let Err(why) = message.channel_id.say(&context, slowmode_content) {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}
