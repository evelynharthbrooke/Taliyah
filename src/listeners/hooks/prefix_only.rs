use serenity::{
    client::Context,
    framework::standard::macros::hook,
    model::channel::Message
};

#[hook]
pub async fn prefix_only(context: &Context, message: &Message) -> () {
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
