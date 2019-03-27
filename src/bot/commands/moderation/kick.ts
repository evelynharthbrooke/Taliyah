/**
 * kick.js -- Kicks a Discord guild member from the guild the command
 * was sent from. User must be a member of the guild.
 *
 * Copyright (c) 2019-present Kamran Mackey.
 *
 * Ellie is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Ellie is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ellie. If not, see <https://www.gnu.org/licenses/>.
 */

import { GuildMember, Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';

export default class KickCommand extends Command {
  public constructor() {
    super('kick', {
      aliases: ['kick'],
      category: 'Moderation',
      description: {
        content: 'Kicks a specified member from the current Discord guild.',
        usage: '<member>',
      },
      args: [
        {
          id: 'member',
          type: 'member',
        },
        {
          id: 'reason',
          match: 'rest',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { member, reason }: { member: GuildMember, reason: string }) {

    if (message.channel.type === 'dm') {
      return message.channel.send('This command cannot be used in direct messages.');
    }

    if (!member) {
      return message.channel.send('You did not mention the member you would like to kick!');
    }

    if (!reason) {
      return message.channel.send('You did not give a reason as to why you want to kick this member!');
    }

    await member.kick(reason).then(() => {
      const KICK_EMBED = new MessageEmbed();

      KICK_EMBED.setTitle('Member kicked!');
      KICK_EMBED.setColor(member.displayHexColor);
      KICK_EMBED.setThumbnail(member.user.displayAvatarURL());
      KICK_EMBED.setDescription(
        `**Guild**: ${member.guild.name}\n` +
        `**Member**: ${member.user.tag}\n` +
        `**Reason**: ${reason}`,
      );

      message.channel.send(KICK_EMBED);

    }).catch((err) => {
      message.channel.send(`I was unable to kick member **${member.user.tag}**.`);
      this.client.logger.error(err);
    });

  }
}
