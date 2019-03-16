/**
 * user.ts -- The user command. Allows a user to get information on a
 * specified user.
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

import { Message, MessageEmbed, User } from 'discord.js';

import { Command } from 'discord-akairo';
import Constants from '../../util/Constants';
import moment from 'moment';

export default class UserCommand extends Command {
  public constructor() {
    super('user', {
      aliases: ['user'],
      category: 'Information',
      description: {
        content: 'Retrieves detailed information on a user, if available.',
        usage: '<blank> or <user>',
      },
      args: [
        {
          id: 'user',
          type: 'user',
        },
      ],
    });
  }

  public async exec(message: Message, { user }: { user: User }) {
    if (message.channel.type === 'dm') {
      return message.channel.send('This command cannot be used in private messages!');
    }

    if (!user) user = message.author;

    if (typeof user === 'undefined') {
      return message.channel.send('I couldn\'t find this user! Please try again.');
    }

    const USER_EMBED = new MessageEmbed();
    const ACCOUNT_CREATION_DATE = moment.utc(user.createdAt).format(Constants.DATE_FORMAT);
    const ACCOUNT_ID = user.id;
    const ACCOUNT_TAG = user.tag;
    const ACCOUNT_TYPE = user.bot ? 'Bot' : 'User';
    const ACCOUNT_PRESENCE = user.presence.activity
      ? `, ${Constants.ACTIVITY_NAMES[user.presence.activity.type]} **${user.presence.activity.name}**`
      : '';
    const GUILD_ACCOUNT = await message.guild.members.fetch(ACCOUNT_ID);
    const GUILD_ACCOUNT_JOIN_DATE = moment.utc(GUILD_ACCOUNT.joinedAt).format(Constants.DATE_FORMAT);
    const GUILD_ACCCOUNT_COLOR = GUILD_ACCOUNT.displayHexColor;
    const GUILD_ACCOUNT_NICK = GUILD_ACCOUNT.nickname || 'No nickname.';
    const GUILD_MAIN_ROLE = GUILD_ACCOUNT.roles.hoist ? GUILD_ACCOUNT.roles.hoist.name : 'No main role.';
    const GUILD_ROLES = GUILD_ACCOUNT.roles.filter(r =>
      r !== message.guild.defaultRole).map(r => r.name).join(' | ') || 'No roles.';
    const GUILD_ROLE_COUNT = GUILD_ACCOUNT.roles.filter(r => r !== message.guild.defaultRole).size;

    let ACCOUNT_STATUS: string;
    if (user.presence.status === 'online') {
      ACCOUNT_STATUS = 'Online';
    } else if (user.presence.status === 'idle') {
      ACCOUNT_STATUS = 'Idle';
    } else if (user.presence.status === 'dnd') {
      ACCOUNT_STATUS = 'Do Not Disturb';
    } else {
      if (user.bot) {
        ACCOUNT_STATUS = 'Unavailable';
      } else {
        ACCOUNT_STATUS = 'Offline';
      }
    }

    USER_EMBED.setTitle(`Information on user ${user.username}`);
    USER_EMBED.setThumbnail(user.displayAvatarURL());
    USER_EMBED.setColor(GUILD_ACCCOUNT_COLOR);
    USER_EMBED.setDescription(
      '**__General__**:\n' +
      `**Status**: ${ACCOUNT_STATUS}${ACCOUNT_PRESENCE}\n` +
      `**Type**: ${ACCOUNT_TYPE}\n` +
      `**Tag**: ${ACCOUNT_TAG}\n` +
      `**ID**: ${ACCOUNT_ID}\n` +
      `**Creation Date**: ${ACCOUNT_CREATION_DATE}\n\n` +
      '**__Guild-specific Info__**:\n' +
      `**Joined**: ${GUILD_ACCOUNT_JOIN_DATE}\n` +
      `**Nickname**: ${GUILD_ACCOUNT_NICK}\n` +
      `**Display Color**: ${GUILD_ACCCOUNT_COLOR}\n` +
      `**Main Role**: ${GUILD_MAIN_ROLE}\n` +
      `**Roles (${GUILD_ROLE_COUNT})**: ${GUILD_ROLES}\n`,
    );

    return message.channel.send(USER_EMBED);
  }
}
