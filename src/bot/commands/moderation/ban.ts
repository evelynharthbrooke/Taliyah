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

import { GuildMember, Message, MessageEmbed } from 'discord.js';

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
          type: 'number',
        },
        {
          id: 'reason',
          match: 'rest',
          type: 'string',
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

    await member.ban({ days, reason }).then(() => {
      const BAN_EMBED = new MessageEmbed();
    });
  }
}
