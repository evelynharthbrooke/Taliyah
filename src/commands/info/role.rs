use serenity::client::Context;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::Args;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::Message;

#[command]
#[aliases(roleinfo)]
#[description("Displays information about a server role.")]
#[only_in(guilds)]
pub fn role(context: &mut Context, message: &Message, arguments: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.read().guild(guild_id).ok_or("Unable to retrieve guild")?;
    let guild_icon = cached_guild.read().icon_url().unwrap();

    if arguments.is_empty() {
        message.channel_id.send_message(&context, |message| message.content("You didn't provide a role to lookup!"))?;
        return Ok(());
    }

    let guild = cached_guild.read();

    let role_name: &str = arguments.rest();
    let role = match guild.role_by_name(role_name) {
        Some(role) => role,
        None => {
            message.channel_id.send_message(&context, |message| message.content("That is an invalid role!"))?;
            return Ok(());
        }
    };

    let name = &role.name;
    let color = role.colour;
    let managed = role.managed;
    let hoisted = role.hoist;
    let mentionable = role.mentionable;
    let position = role.position;
    let id = role.id;
    let permissions = role.permissions;
    let created = id.created_at().format("%A, %B %e, %Y @ %l:%M %P");

    message.channel_id.send_message(&context, |message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.name(name);
                author.icon_url(guild_icon)
            });
            embed.color(color);
            embed.description(format!(
                "**Managed by Integration:** {}\n\
                **Hoisted:** {}\n\
                **Mentionable:** {}\n\
                **Position:** {}\n\
                **Created:** {}\n\
                **Permissions**: {:#?}\n\
                ",
                managed, hoisted, mentionable, position, created, permissions
            ));
            embed.footer(|footer| footer.text(format!("Role ID: {}", id)))
        })
    })?;

    Ok(())
}
