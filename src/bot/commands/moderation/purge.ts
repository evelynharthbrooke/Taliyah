/**
 * purge.js -- The purge command. Allows users with the proper permissions
 * to delete up to 99 messages from the current Discord guild channel.
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

import { GuildChannel, Message } from 'discord.js';

import { Command } from 'discord-akairo';
import pluralize from 'pluralize';

export default class PurgeCommand extends Command {
  public constructor() {
    super('purge', {
      aliases: ['purge', 'prune', 'clean'],
      category: 'Moderation',
      description: {
        content: 'Deletes up to 100 messages from the current guild channel.',
        usage: '<number of messages>',
      },
      cooldown: 15000,
      ratelimit: 5,
      clientPermissions: ['READ_MESSAGE_HISTORY', 'MANAGE_MESSAGES'],
      userPermissions: ['MANAGE_MESSAGES'],
      args: [
        {
          id: 'count',
          type: 'number',
        },
      ],
    });
  }

  public async exec(message: Message, { count }: { count: number }) {
    const channel = message.channel as GuildChannel;
    const messageCount = pluralize('message', count, true);

    if (message.channel.type === 'dm') {
      return message.channel.send('This command cannot be used in direct messages.');
    }

    if (!message.member.hasPermission(['MANAGE_MESSAGES'])) {
      return message.channel.send('Sorry, it looks like you don\'t have the Manage Messages permission, so '
        + 'you cannot use this command!');
    }

    if (count > 99 || !count) {
      return message.channel.send('You either didn\'t enter a number, or you entered a number larger than 99. '
        + 'Please try again.');
    }

    try {
      const messages = await message.channel.messages.fetch({ limit: count + 1 });
      await message.channel.bulkDelete(messages, true);
      await message.channel.send(`Deleting ${messageCount}, please wait...`).then((res) => {
        (res as Message).edit(`Deleted ${messageCount}.`).then(res => res.delete({ timeout: 15000 }));
      });
      this.client.logger.info(`Deleted ${messageCount} from #${channel.name} in the guild ${message.guild}.`);
    } catch (err) {
      this.client.logger.error(`Unable to delete messages!\n\n${err}`);
      message.channel.send(`Sorry, I was unable to delete any messages!\n\`\`\`${err}\`\`\``);
    }
  }
}
