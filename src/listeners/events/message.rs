use crate::utilities::database::get_database;

use serenity::client::Context;
use serenity::model::prelude::Message;

/// Message handler
///
/// Upon message receive events, Ellie will automatically add the
/// author's user id to the profiles database. This event might get
/// expanded later on to handle other things, too.
///
/// Bots are blacklisted from being added to the database, due to them
/// not being actual users, so having profiles holds no value.
pub fn message(_ctx: Context, message: Message) {
    let database = match get_database() {
        Ok(connection) => connection,
        Err(_) => return,
    };

    let user_id = message.author.id.to_string();
    let user_tag = message.author.tag().to_string();

    if message.author.bot {
        return;
    } else {
        let _ = database.execute("INSERT INTO profile (user_id, user_tag) values (?1, ?2)", &[&user_id, &user_tag]);
    }
}
