/**
 * changelog.ts -- Gets the most recent commits (changes) made to
 * Ellie.
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
import * as request from 'superagent';

import { Util } from '../../utils/Util';

export default class ChangelogCommand extends Command {
  public constructor() {
    super('changelog', {
      aliases: ['changelog'],
      category: 'Information',
      description: {
        content: 'Gets a list of the most recent changes made to Ellie.',
        usage: '<blank>',
      },
    });
  }

  public async exec(message: Message) {
    const CHANGELOG_EMBED = new MessageEmbed();
    const GITHUB_API_URL = 'https://api.github.com/';
    const GITHUB_COMMITS_ENDPOINT = GITHUB_API_URL + `repos/${this.client.config.github.repo}/commits`;
    const GITHUB_COMMITS_URL = `https://github.com/${this.client.config.github.repo}/commits/master`;
    const { body: GITHUB_COMMITS } = await request.get(GITHUB_COMMITS_ENDPOINT).set({
      Authorization: `token ${this.client.config.github.token}`,
    });

    const COMMITS = GITHUB_COMMITS.slice(0, 10);

    CHANGELOG_EMBED.setTitle('Most recent commits');
    CHANGELOG_EMBED.setURL(GITHUB_COMMITS_URL);
    CHANGELOG_EMBED.setColor(0x315665);
    CHANGELOG_EMBED.setDescription(COMMITS.map((commit: any) => {
      const sha = `[\`${commit.sha.slice(0, 7)}\`](${commit.html_url})`;
      return `${sha} ${Util.shorten(commit.commit.message.split('\n')[0], 50)} `
        + `(${commit.author.login})`;
    }).join('\n'));
    CHANGELOG_EMBED.setFooter('Powered by the GitHub REST API.');
    CHANGELOG_EMBED.setTimestamp();

    /** Send the changelog. */
    return message.channel.send(CHANGELOG_EMBED);
  }
}
