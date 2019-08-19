/**
 * github.ts -- The base GitHub command. Gives users easy access
 * to the other GitHub-related commands.
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

export default class GitHubCommand extends Command {
  public constructor() {
    super('github', {
      aliases: ['github', 'ghub', 'gh'],
      category: 'Search',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Gets various information from the GitHub GraphQL API.',
        usage: '<subcommand>',
        examples: [
          'user nat',
        ],
      },
      ratelimit: 2,
    });
  }

  public *args() {
    const method = yield {
      type: [
        ['github-user', 'user', 'u'],
      ],
      otherwise: (msg: Message) => {
        const cmdPrefix = this.handler.prefix;
        return msg.channel.send('You did not enter a valid subcommand! Please check '
          + `${cmdPrefix}help github to view the available commands.`);
      },
    };

    return Flag.continue(method);
  }
}
