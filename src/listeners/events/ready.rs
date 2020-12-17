use crate::DatabasePool;

use serenity::{
    client::Context,
    model::{
        gateway::{Activity, Ready},
        user::OnlineStatus
    }
};

use sqlx::Row;
use tracing::info;

pub async fn ready(context: Context, ready: Ready) {
    let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();
    let http = &context.http;

    let bot_gateway = http.get_bot_gateway().await.unwrap();
    let bot_owner = http.get_current_application_info().await.unwrap().owner;

    let sessions_total = bot_gateway.session_start_limit.total;
    let sessions_remaining = bot_gateway.session_start_limit.remaining;

    let name: String = sqlx::query("SELECT current_database()").fetch_one(&pool).await.unwrap().get(0);
    let version: String = sqlx::query("SELECT version()").fetch_one(&pool).await.unwrap().get(0);

    info!("Successfully logged into Discord as the following user:");
    info!("Bot username: {}", ready.user.tag());
    info!("Bot user ID: {}", ready.user.id);
    info!("Bot owner: {}", bot_owner.tag());
    info!("Bot owner ID: {}", bot_owner.id);

    let guilds = ready.guilds.len();

    info!("Connected to the Discord bot API gateway, version {}.", ready.version);
    info!("Gateway reports {}/{} sessions remaining.", sessions_remaining, sessions_total);
    info!("Connected to database {} running {}.", name, version);
    info!("Connected to {} guild(s).", guilds);

    let presence_string = format!("on {} guilds | e.help", guilds);

    context.set_presence(Some(Activity::playing(&presence_string)), OnlineStatus::Online).await
}
