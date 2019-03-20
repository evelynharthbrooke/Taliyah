/**
 * ready.ts -- The listener for the ready event, checks to
 * make sure the bot is ready to accept commands and other
 * input, and also sets the bot's activity.
 *
 * Copyright (c) 2018-present Kamran Mackey.
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

import { Message, MessageEmbed, TextChannel } from 'discord.js';

import Constants from '../../utils/Constants';
import { Listener } from 'discord-akairo';
import moment from 'moment';

export default class MessageDeleteListener extends Listener {
  public constructor() {
    super('messagedelete', {
      emitter: 'client',
      event: 'messageDelete',
      category: 'client',
    });
  }

  async exec(message: Message) {
    const LOG_EMBED = new MessageEmbed();
    const LOG_CHANNEL = message.guild.channels.find(channel => channel.name === 'mod-logs') as TextChannel;

    /** Do not log to channel if channel doesn't exist */
    if (typeof LOG_CHANNEL === 'undefined') {
      this.client.logger.info(
        "Channel doesn't exist. Cannot log message deletion.",
      );
      return null;
    }

    /** Ignore messages from bots. */
    if (message.member.user.bot) {
      this.client.logger.info(`Message deleted was sent from a bot. Not logging to #${LOG_CHANNEL.name}.`);
      return null;
    }

    LOG_EMBED.setTitle('Message deleted!');
    LOG_EMBED.setColor(message.member.displayHexColor);
    LOG_EMBED.setDescription(
      'Looks like a message was deleted...\n\n' +
      `**Channel of Origin**: ${message.channel}\n` +
      `**Message Author**: ${message.author} (${message.author.id})\n` +
      `**Message Contents**: ${message.content}`,
    );

    LOG_EMBED.setFooter(
      `Message created: ${moment
        .utc(message.createdTimestamp)
        .format(Constants.DATE_FORMAT)} (UTC)`,
    );

    LOG_CHANNEL.send(LOG_EMBED);
  }
}
