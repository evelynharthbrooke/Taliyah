use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message
};

#[command]
#[only_in(guilds)]
#[owners_only]
#[required_permissions(CHANGE_NICKNAME)]
/// Sets the bot's nickname in the current guild.
pub async fn bnick(context: &Context, message: &Message, args: Args) -> CommandResult {
    let gid = message.guild_id.unwrap();
    let gid_u64 = gid.as_u64();

    if args.is_empty() {
        context.http.edit_nickname(*gid_u64, None).await?;
        message.channel_id.say(context, "Cleared my nickname / left it the same.").await?;
        return Ok(())
    }

    let nick = args.rest();
    context.http.edit_nickname(*gid_u64, Some(nick)).await?;
    message.channel_id.say(context, format!("Set my nickname to \"{}\".", nick)).await?;

    Ok(())
}
