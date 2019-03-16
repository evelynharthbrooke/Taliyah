/**
 * ban.js -- Bans a Discord guild member from the current Discord
 * guild.
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

import { DiscordAPIError, GuildMember, Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';

export default class BanCommand extends Command {
  public constructor() {
    super('ban', {
      aliases: ['ban'],
      category: 'Moderation',
      description: {
        content: 'Bans the specified member from the current Discord guild.',
        usage: '<member>',
      },
      args: [
        {
          id: 'member',
          type: 'member',
        },
        {
          id: 'days',
          type: 'integer',
          match: 'option',
          flag: ['--days', '-d'],
          default: 7,
        },
        {
          id: 'reason',
          match: 'rest',
          type: 'string',
          default: '',
        },
      ],
    });
  }

  public async exec(message: Message, { member, days, reason }: { member: GuildMember, days: number, reason: string }) {

    if (!member) {
      return message.channel.send('You did not mention the member you would like to ban.');
    }

    if (!reason) {
      return message.channel.send('You did not give a reason as to why you\'d like to ban this member.');
    }

    await member.ban({ reason, days }).then(() => {
      const BAN_EMBED = new MessageEmbed();

      BAN_EMBED.setTitle('Member banned.');
      BAN_EMBED.setColor(member.displayHexColor);
      BAN_EMBED.setThumbnail(member.user.displayAvatarURL());
      BAN_EMBED.setDescription(
        `**Guild**: ${member.guild.name}\n` +
        `**Member**: ${member.user.tag}\n` +
        `**Duration**: ${days} days\n` +
        `**Reason**: ${reason}`,
      );

      message.channel.send(BAN_EMBED);

    }).catch((err) => {

      if (err.code === 50013) {
        return message.channel.send('Sorry, it looks like I am missing the proper permissions ' +
          `so I cannot ban **${member.user.tag}**. Please make sure I have the right permissions ` +
          'and then try again. Also, make sure my role is at the top of the role list!');
      }

      this.client.logger.error(err);
      return message.channel.send(`I was unable to ban member **${member.user.tag}**.`);
    });
  }
}
