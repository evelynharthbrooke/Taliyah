/**
 * join.ts -- Joins the voice channel the message sender is in
 * when the user uses the join command.
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

export default class ConnectCommand extends Command {
  public constructor() {
    super('connect', {
      aliases: ['connect'],
      category: 'Music',
      description: {
        content: 'Joins the current voice channel the command sender is in.',
        usage: '',
      },
    });
  }

  public async exec(message: Message) {
    if (!message.guild!.voiceConnection) {
      const channel = message.member!.voice.channel as VoiceChannel;
      if (!channel) {
        return message.channel.send('You need to be in a voice channel before I can connect to it!');
      }

      if (!channel.joinable) {
        return message.channel.send(`I couldn't connect to the ${channel.name} voice channel. ` +
          'Please check permissions!');
      }

      return channel.join().then(() => message.channel.send(`:musical_note: Connected to the ${channel.name} channel!`));
    }

    return message.channel.send('An error occurred while trying to join the voice channel. Try again later!');
  }
}
