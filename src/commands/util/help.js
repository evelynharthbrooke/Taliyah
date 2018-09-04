/**
 * help.js -- Retrieves help info for all commands, or a specific command.
 * 
 * Erica is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * Erica is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with Erica. If not, see <https://www.gnu.org/licenses/>.
 */
const { Command } = require('discord-akairo');
const { MessageEmbed } = require('discord.js');
const config = require('../../../config.json');

class HelpCommand extends Command {
    constructor() {
        super('help', {
            aliases: ['help'],
            category: 'Utilities',
            clientPermissions: ['EMBED_LINKS'],
            description: {
                content: 'Retrieves all commands & allows you to get info on individual commands.',
                usage: 'leave blank or enter cmd name'
            },
            args: [
                {
                    id: 'command',
                    type: 'commandAlias'
                }
            ]
        })
    }

    async exec(message, { command }) {
        const prefix = config.prefix;
        if (!command) {
            const embed = new MessageEmbed()
                .setAuthor(this.client.user.username, this.client.user.displayAvatarURL({ format: 'png', size: 512 }))
                .setColor(0xFF4922)
                .setURL('https://github.com/KamranMackey/Erica')
                .setDescription(`Here's a list of Erica's commands. For information on a specific command, ` +
                                `please type \`${prefix}help <command>\`.`)
                .setFooter(`${this.handler.modules.size} commands/sub-commands`)

			for (const category of this.handler.categories.values()) {
                embed.addField(`❯ ${category.id}`, `${category.filter(
                    cmd => cmd.aliases.length).map(cmd => `\`${cmd.aliases[0]}\``).join(' ')}`);
            }

			return message.util.send(embed);
        }
        
        const embed = new MessageEmbed()
            .setColor(0xFF4922)
            .setThumbnail(this.client.user.displayAvatarURL({ format: 'png', size: 1024 }))
            .setTitle(`Information on command ${command.aliases[0]}.`)
            .setDescription(command.description.content || '\u200b')
            .addField('❯ Category', command.category, true)
            .addField('❯ Usage', `\`${command.aliases[0]} ${command.description.usage ? command.description.usage : ''}\``, true)

        if (command.aliases.length > 1) embed.addField('❯ Command Aliases', `\`${command.aliases.join('` `')}\``, true);
        
        return message.util.send(embed);
    }
}

module.exports = HelpCommand;
