use crate::{data::DatabasePool, utils::read_config};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::{
        channel::{GuildChannel, Message},
        prelude::OnlineStatus
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
        let bot_owner = http.get_current_application_info().await.unwrap().owner;
        let t_sessions = bot_gateway.session_start_limit.total;
        let r_sessions = bot_gateway.session_start_limit.remaining;
        let db_name: String = sqlx::query("SELECT current_database()").fetch_one(&pool).await.unwrap().get(0);
        let db_version: String = sqlx::query("SELECT version()").fetch_one(&pool).await.unwrap().get(0);

        info!("Successfully logged into Discord as the following user:");
        info!("Bot username: {}", ready.user.tag());
        info!("Bot user ID: {}", ready.user.id);
        info!("Bot owner: {}", bot_owner.tag());

        let guild_count = ready.guilds.len();

        info!("Connected to the Discord API (version {api_version}) with {r_sessions}/{t_sessions} sessions remaining.");
        info!("Connected to database {db_name} running {db_version}.");
        info!("Connected to and serving a total of {guild_count} guild(s).");

        let presence = format!("on {guild_count} guilds | e.help");
        context.set_presence(Some(ActivityData::playing(presence)), OnlineStatus::Online);
    }

    async fn guild_create(&self, context: Context, guild: Guild, _is_new: std::option::Option<bool>) {
        let config = read_config("config.toml");
        let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();

        let guild_name = guild.name;
        let guild_id = guild.id.get() as i64;
        let guild_prefix = config.bot.general.prefix;

        sqlx::query("INSERT INTO guild_info (guild_id, guild_name, guild_prefix) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
            .bind(guild_id)
            .bind(&guild_name)
            .bind(&guild_prefix)
            .execute(&pool)
            .await
            .unwrap();

        info!("Guild {guild_name} recognized and loaded.");
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        if let Err(err) = thread.id.join_thread(ctx.http).await {
            let thread_id = thread.id;
            info!("Failed to succesfully join thread (ID: {thread_id}): {err}")
        } else {
            let name = &thread.name;
            let guild = &thread.guild(&ctx.cache).unwrap().name;
            let id = thread.id.get();
            info!("Joined new thread: {name} (Server: {guild}, ID: {id})")
        }
    }

    /// Message handler
    ///
    /// Upon message receive events, Taliyah will automatically add the
    /// author's user id to the profiles table in the database. This
    /// event might get expanded later on to handle other things, too.
    ///
    /// Bots are blacklisted from being added to the database, due to them
    /// not being actual users, so bots having their own profile sort of
    /// holds no value.
    async fn message(&self, context: Context, message: Message) {
        let id = message.author.id.get() as i64;
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
