/**
 * track.ts -- Retrieves information on the specified Spotify track.
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

import { Command } from 'discord-akairo';
import { Message, MessageEmbed } from 'discord.js';

export default class TrackCommand extends Command {
  public constructor() {
    super('spotify-track', {
      category: 'Music',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Displays information about the specified Spotify track.',
        usage: '<track>',
      },
      args: [
        {
          id: 'track',
          match: 'content',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { track }: { track: string }) {
    return message.channel.send('nothing to see here...yet');
  }
}
