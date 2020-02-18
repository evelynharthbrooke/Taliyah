pub mod cast;
pub mod collection;
pub mod movie;
pub mod show;

use self::cast::*;
use self::collection::*;
use self::movie::*;
use self::show::*;

use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[sub_commands(cast, collection, movie, show)]
/// Gets a variety of information from the API provided by The Movie Database.
///
/// **Subcommands:**
/// `movie <title>`: Retrieves details about a specified movie. **Tip:** y: or year:
/// notation syntax can be used to refine a search.
/// `collection <name>`: Retrieves details about a specified collection.
/// `show <name>`: Retrieves details about a specified television show.
fn tmdb(context: &mut Context, message: &Message) -> CommandResult {
    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.title("Error: Invalid / No Subcommand Entered!");
            embed.description(
                "You did not enter a valid subcommand! Please check \
                `<prefix>help tmdb` for the command usage.",
            )
        })
    })?;

    Ok(())
}
