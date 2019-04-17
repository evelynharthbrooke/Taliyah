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

import { Message, MessageEmbed, GuildMember } from 'discord.js';

import { Command } from 'discord-akairo';
import Constants from '../../utils/Constants';
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
          id: 'member',
          match: 'content',
          type: 'member',
          default: (message: Message) => message.member,
        },
      ],
    });
  }

  public async exec(message: Message, { member }: { member: GuildMember }) {

    if (message.channel.type === 'dm') {
      return message.channel.send('This command cannot be used in direct messages!');
    }

    const USER_EMBED = new MessageEmbed();
    const ACCOUNT_CREATION_DATE = moment.utc(member.user.createdAt).format(Constants.DATE_FORMAT);
    const ACCOUNT_ID = member.id;
    const ACCOUNT_TAG = member.user.tag;
    const ACCOUNT_TYPE = member.user.bot ? 'Bot' : 'User';
    const ACCOUNT_PRESENCE = member.user.presence.activity
      ? `, ${Constants.ACTIVITY_NAMES[member.user.presence.activity.type]} **${member.user.presence.activity.name}**`
      : '';
    const GUILD_ACCOUNT_JOIN_DATE = moment.utc(member.joinedAt!).format(Constants.DATE_FORMAT);
    const GUILD_ACCCOUNT_COLOR = member.displayHexColor;
    const GUILD_ACCOUNT_NICK = member.nickname || 'No nickname.';
    const GUILD_MAIN_ROLE = member.roles.hoist ? member.roles.hoist.name : 'No main role.';
    const GUILD_ROLES = member.roles.filter(r =>
      r !== message.guild!.defaultRole).map(r => r.name).join(' | ') || 'No roles.';
    const GUILD_ROLE_COUNT = member.roles.filter(r => r !== message.guild!.defaultRole).size;

    let ACCOUNT_STATUS: string;
    if (member.user.presence.status === 'online') {
      ACCOUNT_STATUS = 'Online';
    } else if (member.user.presence.status === 'idle') {
      ACCOUNT_STATUS = 'Idle';
    } else if (member.user.presence.status === 'dnd') {
      ACCOUNT_STATUS = 'Do Not Disturb';
    } else {
      if (member.user.bot) {
        ACCOUNT_STATUS = 'Unavailable';
      } else {
        ACCOUNT_STATUS = 'Offline';
      }
    }

    USER_EMBED.setTitle(`Information on user ${member.user.username}`);
    USER_EMBED.setThumbnail(member.user.displayAvatarURL());
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
