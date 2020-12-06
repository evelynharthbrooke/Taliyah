use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult},
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
pub async fn prefix_only(context: &Context, message: &Message) {
    let _ = message
        .channel_id
        .send_message(&context, |message| {
            message.content(
                "Hello! I noticed that you provided my prefix but didn't send a \
                command. If you would like to get help on how to use my functionality, \
                please run the help command."
            )
        })
        .await;
}
