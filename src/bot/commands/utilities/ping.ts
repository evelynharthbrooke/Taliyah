/**
 * ping.ts -- Checks the bot's API latency as well as checks the time
 * it takes for the bot to respond.
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

import { Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';

export default class PingCommand extends Command {
  public constructor() {
    super('ping', {
      aliases: ['ping'],
      description: {
        content: 'Checks Erica\'s response to the Discord API ' +
          'and checks the message latency.',
      },
      category: 'Utilities',
      ratelimit: 2,
    });
  }

  public async exec(message: Message) {
    const msg = await message.channel.send(':ping_pong: Pinging!') as Message;
    const msgLatency = Math.round(msg.createdTimestamp - message.createdTimestamp);
    const apiLatency = Math.round(this.client.ws.ping);

    const pingEmbed = new MessageEmbed();
    pingEmbed.setColor(0x8b0000);
    pingEmbed.setTitle('Latency Information');
    pingEmbed.setDescription(
      'Pong! :ping_pong:\n\n' +
      `**Message Latency**: \`${msgLatency}ms\`\n` +
      `**API Latency**: \`${apiLatency}ms\``);

    return msg.edit(pingEmbed);
  }
}
