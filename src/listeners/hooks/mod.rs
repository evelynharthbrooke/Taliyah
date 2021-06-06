use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::channel::Message
};
use tracing::error;

#[hook]
pub async fn after(context: &Context, message: &Message, command: &str, error: CommandResult) {
    if let Err(why) = &error {
        error!("Error while running command {}", &command);
        error!("{:?}", &error);
        if message.channel_id.say(context, &why).await.is_err() {
            let channel = &message.channel_id.name(&context).await.unwrap();
            error!("Unable to send messages to channel {}", &channel);
        };
    }
}

#[hook]
pub async fn dispatch_error(context: &Context, message: &Message, error: DispatchError) {
    let error_response: String;
    match error {
        DispatchError::Ratelimited(secs) => {
            error_response = format!("This command has been rate limited. Try again in {} second(s).", secs.as_secs());
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::CommandDisabled(command) => {
            error_response = format!("The {} command has been disabled and cannot be used.", command);
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::OnlyForDM => {
            error_response = "This command is only available in Direct Messages.".to_string();
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::OnlyForGuilds => {
            error_response = "This command is only available in guilds.".to_string();
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::OnlyForOwners => {
            error_response = "This command is restricted to bot owners.".to_string();
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::LackingRole => {
            error_response = "You lack the necessary role to use this command.".to_string();
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::LackingPermissions(perms) => {
            error_response = format!("You lack the permissions required to use this command. Permissions needed: {}", perms);
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::NotEnoughArguments { min, given } => {
            error_response = format!("This command needs {} arguments, but got {}.", min, given);
            let _ = message.channel_id.say(context, error_response).await;
        }
        DispatchError::TooManyArguments { max, given } => {
            error_response = format!("Max arguments allowed is {}, but got {}.", max, given);
            let _ = message.channel_id.say(context, error_response).await;
        }
        _ => tracing::warn!("Unhandled Dispatch error: {:?}", error)
    }
}

#[hook]
pub async fn prefix_only(context: &Context, message: &Message) {
    let _ = message
        .channel_id
        .say(
            &context,
            "Hello! I noticed that you provided my prefix but did not send a \
            command. If you would like to get help on how to use my functionality, \
            please run the help command."
        )
        .await;
}
