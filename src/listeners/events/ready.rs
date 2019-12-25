use serenity::client::Context;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::user::OnlineStatus;

/// When the ready event happens, print a message to the
/// console letting us know which user authenticated with
/// the Discord API, and also print the amount of guilds
/// we are currently connected to.
pub fn ready(ctx: Context, ready: Ready) {
    // Print that we have logged into the Discord API.
    println!(
        "Successfully logged into the Discord API as {}#{}. (ID: {})",
        ready.user.name, ready.user.discriminator, ready.user.id
    );

    println!("Connected using Discord gateway version {}.", ready.version);
    println!("Connected to {} guild(s).", ready.guilds.len());

    ctx.set_presence(Some(Activity::playing("!help")), OnlineStatus::Online);
}
