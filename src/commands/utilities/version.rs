use crate::utilities::built_info;
use crate::utilities::git_utils::{show_branch, show_head_rev};

use git2::Repository;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

use std::env;

#[command]
#[description(
    "Retrieves the bot version, as well as the version of Rust that \
    the bot was built with."
)]
#[usage("<blank>")]
pub fn version(context: &mut Context, message: &Message) -> CommandResult {
    let repo = Repository::open(&env::var("CARGO_MANIFEST_DIR").unwrap())?;

    let git_version = built_info::GIT_VERSION.map_or_else(|| "Unknown git version".to_owned(), |v| format!(" (git {})", v));
    let name = context.cache.read().user.name.to_string();
    let version = built_info::PKG_VERSION;
    let codename = env::var("BOT_CODENAME").unwrap();
    let branch = show_branch(&repo);
    let revision = show_head_rev(&repo);
    let build_host = built_info::HOST;
    let build_target = built_info::TARGET;
    let build_date = built_info::BUILT_TIME_UTC;
    let rust_runtime = built_info::RUSTC_VERSION;

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title(format!("{} version information", name));
            embed.description(format!(
                "**Version**: v{} {}\n\
                **Branch**: {}\n\
                **Revision**: {}\n\
                **Codename**: {}\n\
                **Build date**: {}\n\
                **Build host**: {}\n\
                **Build target**: {}\n\
                **Rust runtime**: {}",
                version, git_version, branch, revision, codename, 
                build_date, build_host, build_target, rust_runtime
            ))
        })
    })?;

    Ok(())
}
