use serenity::client::Context;
use serenity::model::gateway::{Activity, Ready};
use serenity::model::user::OnlineStatus;

/// Ready event handler
/// 
/// This handles a couple of setup tasks while the bot is
/// initializing, such as setting the bot's presence, and
/// doing any logging, as necessary.
pub fn ready(context: Context, ready: Ready) {
    // Log a basic bit of information, like the username and ID
    // of the user logging into the Discord API, the number of guilds
    // the bot has connected to, as well as the gateway version currently
    // being used by the bot.
    println!("Successfully logged into Discord as the following user:");
    println!("Bot username: {}#{}", ready.user.name, ready.user.discriminator);
    println!("Bot user ID: {}", ready.user.id);
    println!("Connected using Discord gateway version {}.", ready.version);
    println!("Connected to {} guild(s).", ready.guilds.len());
    // Set a basic presence. This will be improved later on.
    context.set_presence(Some(Activity::playing("!help")), OnlineStatus::Online);
}
