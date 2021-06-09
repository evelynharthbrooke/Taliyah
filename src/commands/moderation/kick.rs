use crate::utils::parsing_utils::parse_user;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message
};

#[command("kick")]
#[usage = "<member>"]
#[required_permissions(KICK_MEMBERS)]
#[min_args(1)]
/// Kicks the given member from the server.
pub async fn kick(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if message.is_private() {
        message.channel_id.say(context, "You can't kick anyone in private messages!").await?;
        return Ok(());
    }

    let mention = args.single_quoted::<String>()?;
    let guild = message.guild_id.unwrap();
    let user = parse_user(&mention, guild, context).await.unwrap();
    let member = message.guild(context).await.unwrap().member(context, user).await.unwrap();

    let name = &member.user.name;
    let disc = &member.user.discriminator;
    let id = &member.user.id;
    let reason = args.remains();

    if let Some(reason) = reason {
        member.kick_with_reason(context, reason).await?;
        message.reply(context, format!("Kicked member `{name}#{disc}` with id `{id}` for reason `{reason}`!")).await?;
        return Ok(());
    } else {
        member.kick(context).await?;
        message.reply(context, format!("Kicked member `{name}#{disc}` with id `{id}`.")).await?;
        return Ok(());
    }
}
