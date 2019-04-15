/**
 * user.ts -- The GitHub user command. Retrieves information on a
 * GitHub user.
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

import moment from 'moment';
import graphql from '@octokit/graphql';

export default class GitHubUserCommand extends Command {
  public constructor() {
    super('github-user', {
      category: 'Search',
      description: {
        content: 'Gets information on a specified GitHub user.',
        usage: '<user>',
      },
      args: [
        {
          id: 'user',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { user }: { user: string }) {
    const GITHUB_EMBED = new MessageEmbed(); // The GitHub user embed.
    const GITHUB_TOKEN = this.client.config.github.token; // The GitHub API token.
    const GITHUB_API = graphql.defaults({ headers: { Authorization: `bearer ${GITHUB_TOKEN}` } });
    const USER_QUERY = `query {
      user(login: "${user}") {
        login
        name
        location
        bio
        company
        url
        createdAt
        isEmployee
        avatarUrl
        websiteUrl
        status {
          message
        }
        repositories {
          totalCount
        }
        following {
          totalCount
        }
        followers {
          totalCount
        }
      }
    }`;

    const { user: GH_USER } = await GITHUB_API(USER_QUERY); // The GitHub API request.

    const GH_USER_NAME = GH_USER.name ? GH_USER.name : GH_USER.login; // The name of the GitHub user.
    const GH_USER_AVATAR = GH_USER.avatarUrl; // The avatar belonging to the GitHub user.
    const GH_USER_URL = GH_USER.url; // The URL linking to the user's GitHub profile.
    const GH_USER_JOIN_DATE = moment(GH_USER.createdAt).format('LL'); // The date of when the user joined GitHub.
    const GH_USER_BIOGRAPHY = GH_USER.bio ? GH_USER.bio : ''; // The biography listed on the user's GitHub profile.
    const GH_USER_FOLLOWING = GH_USER.following.totalCount; // The amount of users the user is following.
    const GH_USER_FOLLOWERS = GH_USER.followers.totalCount; // The amount of users who are following the user.
    const GH_USER_WEBSITE = GH_USER.websiteUrl ? GH_USER.websiteUrl : 'No website listed.'; // The site listed on the user's profile.
    const GH_USER_STATUS = GH_USER.status ? GH_USER.status.message : 'No status listed.'; // The status listed on the user's profile.
    const GH_USER_LOCATION = GH_USER.location ? GH_USER.location : 'No location listed.'; // The location of the user.
    const GH_USER_COMPANY = GH_USER.company ? GH_USER.company : 'No company listed.'; // The company listed on the user's profile.
    const GH_USER_REPOSITORIES = GH_USER.repositories.totalCount; // The amount of repositories the user has created or forked.
    const GH_USER_EMPLOYEE = GH_USER.isEmployee ? 'Yes' : 'No'; // Whether or not the user is an employee.

    GITHUB_EMBED.setTitle(GH_USER_NAME); // Set the title of the embed to the GitHub user's name on file.
    GITHUB_EMBED.setThumbnail(GH_USER_AVATAR); // Set the thumbnail of the embed to the GitHub user's avatar.
    GITHUB_EMBED.setURL(GH_USER_URL); // Set the URL of the embed to the GitHub user's profile URL.
    GITHUB_EMBED.setDescription( // Set the description of the embed to contain all of the GitHub user information.
      `${GH_USER_BIOGRAPHY}\n\n` +
      '**__Basic Details__**:\n' +
      `**Status**: ${GH_USER_STATUS}\n` +
      `**Joined**: ${GH_USER_JOIN_DATE}\n` +
      `**Repositories**: ${GH_USER_REPOSITORIES}\n` +
      `**Location**: ${GH_USER_LOCATION}\n` +
      `**Following**: ${GH_USER_FOLLOWING}\n` +
      `**Followers**: ${GH_USER_FOLLOWERS}\n` +
      `**Website**: ${GH_USER_WEBSITE}\n` +
      `**Company**: ${GH_USER_COMPANY}\n\n` +
      '**__Other Details__**:\n' +
      `**Is GitHub Employee**: ${GH_USER_EMPLOYEE}`);
    GITHUB_EMBED.setFooter('Powered by the GitHub GraphQL API.'); // Set the embed footer.
    GITHUB_EMBED.setTimestamp(); // Set the embed's timestamp.

    return message.channel.send(GITHUB_EMBED);
  }
}
