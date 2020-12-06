use crate::DatabasePool;

use serenity::{
    client::Context,
    model::{
        gateway::{Activity, Ready},
        user::OnlineStatus
    }
};

use tracing::info;

/// Ready event handler
///
/// This handles a couple of setup tasks while the bot is
/// initializing, such as setting the bot's presence, and
/// doing any logging, as necessary.
pub async fn ready(context: Context, ready: Ready) {
    // Retrieve the currently active database connection saved in Serenity's
    // datastore.
    let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();
    // Get the bot's registered owner.
    let owner = context.http.get_current_application_info().await.unwrap().owner;

    let version_query = sqlx::query!("SELECT version();").fetch_optional(&pool).await.unwrap();
    let version = version_query.unwrap().version.unwrap().to_string();
    let name_query = sqlx::query!("SELECT current_database();").fetch_optional(&pool).await.unwrap();
    let name = name_query.unwrap().current_database.unwrap().to_string();

    // Log a basic bit of information, like the username and ID
    // of the user logging into the Discord API, the number of guilds
    // the bot has connected to, as well as the gateway version currently
    // being used by the bot.
    info!("Successfully logged into Discord as the following user:");
    info!("Bot username: {}", ready.user.tag());
    info!("Bot user ID: {}", ready.user.id);
    info!("Bot owner: {}", owner.tag());
    info!("Bot owner ID: {}", owner.id);

    let guilds = ready.guilds.len();

    info!("Connected to version {} of the Discord gateway.", ready.version);
    info!("Connected to database {} running {}.", name, version);
    info!("Connected to {} guild(s).", guilds);

    let presence_string = format!("on {} guilds | e.help", guilds);

    // Set a basic presence. This will be improved later on.
    context.set_presence(Some(Activity::playing(&presence_string)), OnlineStatus::Online).await
}
