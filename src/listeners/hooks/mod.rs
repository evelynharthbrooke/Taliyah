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
        if message.channel_id.say(context, why.to_string()).await.is_err() {
            let channel = &message.channel_id.name(&context).await.unwrap();
            error!("Unable to send messages to channel {}", &channel);
        };
    }
}

#[hook]
pub async fn dispatch_error(context: &Context, message: &Message, error: DispatchError, command: &str) {
    let error_response: String;
    match error {
        DispatchError::Ratelimited(secs) => {
            error_response = format!("This command has been rate limited. Try again in {} second(s).", secs.as_secs());
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::CommandDisabled => {
            error_response = format!("The `{command}` command has been disabled and cannot be used.");
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::OnlyForDM => {
            error_response = "This command is only available in Direct Messages.".to_string();
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::OnlyForGuilds => {
            error_response = "This command is only available in guilds.".to_string();
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::OnlyForOwners => {
            error_response = "This command is restricted to bot owners.".to_string();
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::LackingRole => {
            error_response = "You lack the necessary role to use this command.".to_string();
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::LackingPermissions(perms) => {
            error_response = format!("You lack the permissions required to use this command. Permissions needed: {perms}");
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::NotEnoughArguments { min, given } => {
            error_response = format!("The `{command}` command needs {min} arguments, but got {given}.");
            drop(message.channel_id.say(context, error_response).await);
        }
        DispatchError::TooManyArguments { max, given } => {
            error_response = format!("Max arguments allowed is {max}, but got {given}.");
            drop(message.channel_id.say(context, error_response).await);
        }
        _ => tracing::warn!("Unhandled Dispatch error: {:?}", error)
    }
}

#[hook]
pub async fn prefix_only(context: &Context, message: &Message) {
    drop(message.channel_id.say(&context, "For info on my features, run the help command.").await);
}
