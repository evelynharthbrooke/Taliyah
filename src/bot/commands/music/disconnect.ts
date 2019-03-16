/**
 * leave.ts -- Leaves the voice channel the bot is currently connected
 * to.
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

import { Message, VoiceChannel } from 'discord.js';

import { Command } from 'discord-akairo';

export default class DisconnectCommand extends Command {
  public constructor() {
    super('disconnect', {
      aliases: ['disconnect'],
      category: 'Music',
      description: {
        content: 'Disconnects from the voice channel the bot is currently connected to.',
      },
    });
  }

  public async exec(message: Message) {
    if (!message.guild.voiceConnection) {
      return message.channel.send('I\'m not currently in a voice channel, so I can\'t ' +
        'disconnect from any.');
    }

    const channel = message.guild.voiceConnection.channel;

    channel.leave();

    return message.channel.send(`:musical_note: Disconnected from the ${channel.name} channel.`);
  }
}
