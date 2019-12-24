use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use std::process::Command;

#[command]
#[description("Retrieves the bot version, as well as the underlying Rust / Cargo versions.")]
#[usage("<blank>")]
pub fn version(ctx: &mut Context, message: &Message) -> CommandResult {

    let bot_version = env!("CARGO_PKG_VERSION");

    let rust_version = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C rustc -V").output().expect("failed to get rust version")
    } else {
        Command::new("sh").arg("rustc -V").output().expect("failed to get rust version")
    };

    let cargo_version = if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C cargo -V").output().expect("failed to get cargo version")
    } else {
        Command::new("sh").arg("cargo -V").output().expect("failed to get cargo version")
    };

    let _ = message.channel_id.say(
        &ctx,
        format!(
            "**Version:**: {}\n\
            **Rust:** {}\
            **Cargo:** {}",
            bot_version,
            String::from_utf8_lossy(&rust_version.stdout),
            String::from_utf8_lossy(&cargo_version.stdout)
        ),
    );

    Ok(())
}
