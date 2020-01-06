use git_testament::{git_testament, render_testament};
use lazy_static::lazy_static;
use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

git_testament!(TESTAMENT);

fn version_info() -> &'static str {
    lazy_static! {
        static ref RENDERED: String = render_testament!(TESTAMENT, "rust_rewrite");
    }
    &RENDERED
}

#[command]
#[description("Retrieves the bot version, as well as the underlying rustc version.")]
#[usage("<blank>")]
pub fn version(ctx: &mut Context, message: &Message) -> CommandResult {
    let bot_version = version_info();
    let rustc_version = env!("RUSTC_VERSION");

    let _ = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.description(format_args!("**Bot Version**: {}\n**Rust Version**: {}", bot_version, rustc_version))
        })
    });

    Ok(())
}
