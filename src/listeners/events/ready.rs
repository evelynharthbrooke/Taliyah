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

    let api_version = ready.version;
    let bot_gateway = http.get_bot_gateway().await.unwrap();
    let t_sessions = bot_gateway.session_start_limit.total;
    let r_sessions = bot_gateway.session_start_limit.remaining;
    let bot_owner = http.get_current_application_info().await.unwrap().owner;
    let db_name: String = sqlx::query("SELECT current_database()").fetch_one(&pool).await.unwrap().get(0);
    let db_version: String = sqlx::query("SELECT version()").fetch_one(&pool).await.unwrap().get(0);

    info!("Successfully logged into Discord as the following user:");
    info!("Bot username: {}", ready.user.tag());
    info!("Bot user ID: {}", ready.user.id);
    info!("Bot owner: {}", bot_owner.tag());

    let guild_count = ready.guilds.len();

    info!("Connected to the Discord API (version {}) with {}/{} sessions remaining.", api_version, r_sessions, t_sessions);
    info!("Connected to database {} running {}.", db_name, db_version);
    info!("Connected to and serving a total of {} guild(s).", guild_count);

    let presence_string = format!("on {} guilds | e.help", guild_count);

    context.set_presence(Some(Activity::playing(&presence_string)), OnlineStatus::Online).await
}
