use serenity::{
    builder::{CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateMessage},
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message
};

#[command]
#[aliases(roleinfo)]
#[description("Displays information about a server role.")]
#[only_in(guilds)]
async fn role(context: &Context, message: &Message, arguments: Args) -> CommandResult {
    let cache = &context.cache;
    let guild_id = message.guild_id.ok_or("Failed to get GuildID from Message.")?;
    let cached_guild = cache.guild(guild_id).ok_or("Unable to retrieve guild")?.clone();
    let guild_icon = cached_guild.icon_url().unwrap();

    if arguments.is_empty() {
        message.channel_id.say(&context, "You didn't provide a role to lookup!").await?;
        return Ok(());
    }

    let role_name: &str = arguments.rest();
    let role = match cached_guild.role_by_name(role_name) {
        Some(role) => role,
        None => {
            message.channel_id.say(context, "That is an invalid role!").await?;
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

    let role_fields = vec![
        ("Integration Managed", managed.to_string(), true),
        ("Hoisted", hoisted.to_string(), true),
        ("Mentionable", mentionable.to_string(), true),
        ("Position", position.to_string(), true),
        ("Created", created.to_string(), false),
        ("Permissions", permissions.to_string(), false),
    ];

    let embed = CreateEmbed::new()
        .author(CreateEmbedAuthor::new(name).icon_url(guild_icon))
        .color(color)
        .fields(role_fields)
        .footer(CreateEmbedFooter::new(format!("Role ID: {id}")));

    message.channel_id.send_message(&context, CreateMessage::new().embed(embed)).await?;

    Ok(())
}
