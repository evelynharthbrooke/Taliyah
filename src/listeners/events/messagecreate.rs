use crate::utilities::database::get_database;

use serenity::client::Context;
use serenity::model::prelude::Message;

/// Message handler
///
/// Upon message receive events, Ellie will automatically add the
/// author's user id to the profiles database. This event might get
/// expanded later on to handle other things, too.
pub fn message(_ctx: Context, new_message: Message) {
    let database = match get_database() {
        Ok(connection) => connection,
        Err(_) => return,
    };

    let user_id = new_message.author.id.to_string();
    let user_tag = new_message.author.tag().to_string();
    let _ = database.execute("INSERT INTO profile (user_id, user_tag) values (?1, ?2)", &[&user_id, &user_tag]);
}
