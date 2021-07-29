use crate::{data::DatabasePool, utils::read_config};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::{GuildChannel, Message},
        prelude::{Activity, OnlineStatus}
    },
    model::{gateway::Ready, guild::Guild}
};
use sqlx::Row;
use tracing::info;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
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

    async fn guild_create(&self, context: Context, guild: Guild, _is_new: bool) {
        let config = read_config("config.toml");
        let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();

        let guild_name = guild.name;
        let guild_id = guild.id.0 as i64;
        let guild_prefix = config.bot.general.prefix;

        sqlx::query("INSERT INTO guild_info (guild_id, guild_name, guild_prefix) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(&guild_id)
            .bind(&guild_name)
            .bind(&guild_prefix)
            .execute(&pool)
            .await
            .unwrap();

        info!("Guild {} recognized and loaded.", guild_name);
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        if let Err(e) = thread.id.join_thread(ctx.http).await {
            info!("Failed to join thread (ID: {}) successfully: {}", thread.id, e)
        } else {
            let name = &thread.name;
            let guild = thread.clone().guild(ctx.cache).await.unwrap().name;
            let id = thread.id.as_u64();
            info!("Joined new thread: {} (Server: {}, ID: {})", name, guild, id)
        }
    }

    /// Message handler
    ///
    /// Upon message receive events, Ellie will automatically add the
    /// author's user id to the profiles table in the database. This
    /// event might get expanded later on to handle other things, too.
    ///
    /// Bots are blacklisted from being added to the database, due to them
    /// not being actual users, so bots having their own profile sort of
    /// holds no value.
    async fn message(&self, context: Context, message: Message) {
        let id = message.author.id.0 as i64;
        let tag = message.author.tag();
        let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();
        if !message.author.bot {
            sqlx::query("INSERT INTO profile_data (user_id, user_tag) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(id)
                .bind(tag)
                .execute(&pool)
                .await
                .unwrap();
        }
    }
}
