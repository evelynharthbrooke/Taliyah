/**
 * spotify.ts -- The base Spotify command. Gives users easy access
 * to the other commands.
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

import { Command, Flag } from 'discord-akairo';
import { Message } from 'discord.js';

export default class SpotifyCommand extends Command {
  public constructor() {
    super('spotify', {
      aliases: ['spotify', 'sp', 'splookup'],
      category: 'Music',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Gets a variety of information from the Spotify API, such as artist '
          + 'information, album information, song information, and more. ',
        usage: '<subcommand>',
        examples: [
          'artist Alicia Keys',
        ],
      },
      ratelimit: 2,
    });
  }

  public *args() {
    const method = yield {
      type: [
        ['spotify-artist', 'artist'],
      ],
      otherwise: (msg: Message) => {
        const cmdPrefix = this.handler.prefix;
        return msg.channel.send('You did not enter a valid subcommand! Please check '
          + `${cmdPrefix}help spotify to view the available commands.`);
      },
    };

    return Flag.continue(method);
  }
}
