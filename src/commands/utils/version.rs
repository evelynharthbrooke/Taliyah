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
pub fn version(ctx: &mut Context, message: &Message) -> CommandResult {

    let repo = Repository::open(&env::var("CARGO_MANIFEST_DIR").unwrap())?;

    let git_description = built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v));
    let name = ctx.cache.read().user.name.to_string();
    let version = built_info::PKG_VERSION;
    let codename = env::var("BOT_CODENAME").unwrap();
    let branch = show_branch(&repo);
    let revision = show_head_rev(&repo);
    let build_target = built_info::TARGET;
    let build_date = built_info::BUILT_TIME_UTC;
    let rust_runtime = built_info::RUSTC_VERSION;

    let _ = message.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.title(format_args!("{} version information", name));
            e.description(format_args!(
                "
                **Version**: {} {}\n\
                **Branch**: {}\n\
                **Revision**: {}\n\
                **Codename**: {}\n\
                **Built for**: {}\n\
                **Built at**: {}\n\
                **Rust runtime**: {}",
                version, git_description, branch, revision, codename, 
                build_target, build_date, rust_runtime
            ))
        })
    });

    Ok(())
}
