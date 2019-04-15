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
    // The GitHub user embed.
    const GITHUB_EMBED = new MessageEmbed().setColor(0xFFFFFF);
    // The GitHub error embed.
    const GITHUB_ERROR_EMBED = new MessageEmbed().setColor(0xB00020);

    // If no query for user was provided, send an error message
    // letting the user know that no username was provided.
    if (!user) {
      GITHUB_ERROR_EMBED.setTitle('Error: No username provided.');
      GITHUB_ERROR_EMBED.setDescription(
        'You did not provide the username of the user you would ' +
        'like to get information on. Please provide one, and then try again.\n\n' +
        '**Examples**: nat, afollestad',
      );
      return message.channel.send(GITHUB_ERROR_EMBED);
    }

    // The GitHub API token.
    const GITHUB_TOKEN = this.client.config.github.token;
    // Initialize the GraphQL API wrapper.
    const GITHUB_API = graphql.defaults({ headers: { Authorization: `bearer ${GITHUB_TOKEN}` } });
    // Build the GitHub user API query.
    const USER_QUERY = `query {
      user(login: "${user}") {
        login
        name
        location
        bio
        company
        url
        createdAt
        avatarUrl
        websiteUrl
        isBountyHunter
        isCampusExpert
        isDeveloperProgramMember
        isEmployee
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

    // The GitHub API request.
    const { user: GH_USER } = await GITHUB_API(USER_QUERY);

    // The name of the GitHub user.
    const GH_USER_NAME = GH_USER.name ? GH_USER.name : GH_USER.login;
    // The avatar belonging to the GitHub user.
    const GH_USER_AVATAR = GH_USER.avatarUrl;
    // The URL linking to the user's GitHub profile.
    const GH_USER_URL = GH_USER.url;
    // The date of when the user joined GitHub.
    const GH_USER_JOIN_DATE = moment(GH_USER.createdAt).format('LL');
    // The biography listed on the user's GitHub profile.
    const GH_USER_BIOGRAPHY = GH_USER.bio ? GH_USER.bio : '';
    // The amount of users the user is following.
    const GH_USER_FOLLOWING = GH_USER.following.totalCount;
    // The amount of users who are following the user.
    const GH_USER_FOLLOWERS = GH_USER.followers.totalCount;
    // The website listed on the user's profile.
    const GH_USER_WEBSITE = GH_USER.websiteUrl ? GH_USER.websiteUrl : 'No website listed.';
    // The status listed on the user's profile.
    const GH_USER_STATUS = GH_USER.status ? GH_USER.status.message : 'No status listed.';
    // The location of the user.
    const GH_USER_LOCATION = GH_USER.location ? GH_USER.location : 'No location listed.';
    // The company listed on the user's profile.
    const GH_USER_COMPANY = GH_USER.company ? GH_USER.company : 'No company listed.';
    // The amount of repositories the user has created or forked.
    const GH_USER_REPOSITORIES = GH_USER.repositories.totalCount;
    // Whether or not the user is a developer program member.
    const GH_USER_DEV_PROGRAM_MEMBER = GH_USER.isDeveloperProgramMember ? 'Yes' : 'No';
    // Whether or not this user is a participant in the GitHub Security Bug Bounty.
    const GH_USER_BUG_BOUNTY = GH_USER.isBountyHunter ? 'Yes' : 'No';
    // Whether or not this user is a participant in the GitHub Campus Experts Program.
    const GH_USER_CAMPUS_EXPERT = GH_USER.isCampusExpert ? 'Yes' : 'No';
    // Whether or not this user is a GitHub employee.
    const GH_USER_EMPLOYEE = GH_USER.isEmployee ? 'Yes' : 'No';

    // Set the title of the embed to the GitHub user's name on file.
    GITHUB_EMBED.setTitle(GH_USER_NAME);
    // Set the thumbnail of the embed to the GitHub user's avatar.
    GITHUB_EMBED.setThumbnail(GH_USER_AVATAR);
    // Set the URL of the embed to the GitHub user's profile URL.
    GITHUB_EMBED.setURL(GH_USER_URL);
    // Set the description of the embed to contain all of the GitHub user information.
    GITHUB_EMBED.setDescription(
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
      `**Bounty Hunter**: ${GH_USER_BUG_BOUNTY}\n` +
      `**Campus Expert**: ${GH_USER_CAMPUS_EXPERT}\n` +
      `**Developer Program Member**: ${GH_USER_DEV_PROGRAM_MEMBER}\n` +
      `**GitHub Employee**: ${GH_USER_EMPLOYEE}`);
    // Set the embed footer.
    GITHUB_EMBED.setFooter('Powered by the GitHub GraphQL API.');
    // Set the embed's timestamp.
    GITHUB_EMBED.setTimestamp();

    return message.channel.send(GITHUB_EMBED);
  }
}
