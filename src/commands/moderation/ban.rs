use crate::utils::parsing_utils::parse_user;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message
};

#[command("ban")]
#[usage = "<member>"]
#[required_permissions(BAN_MEMBERS)]
#[min_args(1)]
/// Bans the given member from the server.
async fn ban(context: &Context, message: &Message, mut args: Args) -> CommandResult {
    if message.is_private() {
        message.channel_id.say(context, "You can't kick anyone in private messages!").await?;
        return Ok(());
    }

    let mention = args.single_quoted::<String>()?;
    let guild_id = message.guild_id.unwrap();
    let user = parse_user(&mention, guild_id, context).await.unwrap();
    let guild = message.guild(&context.cache).unwrap().clone();
    let member = guild.member(context, user).await.unwrap();

    let name = &member.user.name;
    let disc = &member.user.discriminator;
    let id = &member.user.id;
    let reason = args.remains();

    if let Some(reason) = reason {
        member.ban_with_reason(context, 1, reason).await?;
        message.reply(context, format!("Banned member `{name}#{disc}` with id `{id}` for reason `{reason}`!")).await?;
        return Ok(());
    } else {
        member.ban(context, 1).await?;
        message.reply(context, format!("Banned member `{name}#{disc}` with id `{id}`.")).await?;
        return Ok(());
    }
}
