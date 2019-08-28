/**
 * changelog.ts -- Gets the most recent commits (changes) made to
 * Ellie via the GitHub GraphQL API.
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

const { graphql } = require('@octokit/graphql');

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
    const embed = new MessageEmbed();
    const token = this.client.config.github.token;
    const owner = this.client.config.github.repo_owner;
    const name  = this.client.config.github.repo_name;
    const query = `query commits($owner: String!, $repo: String!) {
      repository(owner: $owner, name: $repo) {
        url
        defaultBranchRef {
          name
          target {
            ... on Commit {
              history(first: 10) {
                edges {
                  node {
                    ... on Commit {
                      messageHeadline
                      oid
                      url
                    }
                  }
                }
              }
            }
          }
        }
      }
    }`;

    const result = await graphql(query, {
      owner: `${owner}`,
      repo: `${name}`,
      headers: {
        authorization: `bearer ${token}`,
      },
    });

    const target = result.repository.defaultBranchRef.target;
    const url = result.repository.url;
    const branch = result.repository.defaultBranchRef.name;
    const commits = target.history.edges.map((c: any) => {
      const commit = c.node;
      const title = commit.messageHeadline;
      const hash = `[\`${commit.oid.slice(0, 7)}\`](${commit.url})`;
      return `${hash} ${title}`;
    }).join('\n');

    embed.setTitle('Recent Changes');
    embed.setURL(`${url}/commits/${branch}`);
    embed.setDescription(commits);
    embed.setFooter('Powered by the GitHub GraphQL API.');
    embed.setTimestamp();

    return message.channel.send(embed);
  }
}
