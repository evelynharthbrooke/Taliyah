use crate::DatabasePool;
use serenity::{client::Context, model::prelude::Message};

/// Message handler
///
/// Upon message receive events, Ellie will automatically add the
/// author's user id to the profiles table in the database. This
/// event might get expanded later on to handle other things, too.
///
/// Bots are blacklisted from being added to the database, due to them
/// not being actual users, so bots having their own profile sort of
/// holds no value.
pub async fn message(context: Context, message: Message) {
    let pool = context.data.read().await.get::<DatabasePool>().cloned().unwrap();

    let user_id = message.author.id.0 as i64;
    let user_tag = message.author.tag();

    if !message.author.bot {
        sqlx::query("INSERT INTO profile_data (user_id, user_tag) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(user_id)
            .bind(user_tag)
            .execute(&pool)
            .await
            .unwrap();
    }
}
