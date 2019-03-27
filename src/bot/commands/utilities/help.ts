/**
 * help.js -- The help command. Sends a message containing all of
 * the available command, and allows users to get more information
 * on a command by doing !help <command name>.
 *
 * Copyright (c) 2019-present Kamran Mackey.
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

import { Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';

export default class HelpCommand extends Command {
  public constructor() {
    super('help', {
      aliases: ['help', 'halp'],
      category: 'Utilities',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Retrieves all commands & allows you to get info on individual commands.',
        usage: '<blank> or <command>',
      },
      args: [
        {
          id: 'command',
          type: 'commandAlias',
        },
      ],
    });
  }

  public async exec(message: Message, { command }: { command: Command }) {
    const prefix = (this.handler.prefix as String[])[0];

    if (!command) {
      const embed = new MessageEmbed();
      embed.setTitle('Help Information for Ellie');
      embed.setColor(0xFF4922);
      embed.setThumbnail(this.client.user!.displayAvatarURL({ format: 'png', size: 1024 }));
      embed.setDescription(
        'This is a list of all commands available to Ellie. For information on a specific command, ' +
        `please type **${prefix}help <command>**.`);
      embed.setFooter(`${this.handler.modules.size} commands/sub-commands`);

      for (const category of this.handler.categories.values()) {
        embed.addField(`❯ ${category.id}`, `${category.filter(
          c => c.aliases.length > 0).map((c: Command) => `\`${c.aliases[0]}\``).join(' ')}`,
        );
      }



      return message.channel.send(embed);
    }

    function getAliases() {
      let aliases: string;
      if (command.aliases.length > 1) {
        aliases = `**Aliases**: ${command.aliases.join(', ')}`;
      } else {
        aliases = '';
      }
      return aliases;
    }

    const examples = command.description.examples
      ? command.description.examples.join(', ')
      : 'No examples given!';

    const embed = new MessageEmbed()
      .setColor(0xFF4922)
      .setThumbnail(this.client.user!.displayAvatarURL({ format: 'png', size: 1024 }))
      .setTitle(`Help information for command ${command.aliases[0]}`)
      .setDescription(
        `${command.description.content || '\u200b'}\n\n` +
        `**Category**: ${command.category}\n` +
        `**Usage**: ${prefix}${command.aliases[0]} ${command.description.usage ? command.description.usage : ''}\n` +
        `**Examples**: ${examples}\n` +
        `**Permissions**: ${command.userPermissions || 'No permissions necessary.'}\n` +
        `${getAliases()}`,
      );
    return message.channel.send(embed);
  }
}
