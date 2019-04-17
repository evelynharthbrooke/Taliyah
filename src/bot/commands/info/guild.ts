/**
 * guild.ts -- Retrieves information about the current Discord
 * guild (server).
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

import { Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';
import Constants from '../../utils/Constants';
import moment from 'moment';

export default class GuildCommand extends Command {
  public constructor() {
    super('server', {
      aliases: ['server', 'guild', 'serverinfo', 'guildinfo'],
      category: 'Information',
      description: {
        content: 'Retrieves detailed information about the current Discord guild.',
        usage: '',
      },
      args: [
        {
          id: 'guild',
          type: 'guild',
        },
      ],
    });
  }

  public async exec(message: Message) {

    if (message.guild!.available) {
      const GUILD_EMBED = new MessageEmbed();
      const GUILD_NAME = message.guild!.name;
      const GUILD_ID = message.guild!.id;
      const GUILD_OWNER = message.guild!.owner!.user.tag;
      const GUILD_MEMBERS = message.guild!.members.size;
      const GUILD_MEMBERS_USERS = message.guild!.members.filter(m => !m.user.bot).size;
      const GUILD_MEMBERS_BOTS = message.guild!.members.filter(m => m.user.bot).size;
      const GUILD_PRESENCES = message.guild!.presences.size;
      const GUILD_PRESENCES_USERS = message.guild!.presences.filter(p => !p.user!.bot).size;
      const GUILD_PRESENCES_BOTS = message.guild!.presences.filter(p => p.user!.bot).size;
      const GUILD_CHANNELS = message.guild!.channels.filter(c => c.type !== 'category').size;
      const GUILD_CHANNELS_TEXT = message.guild!.channels.filter(c => c.type === 'text').size;
      const GUILD_CHANNELS_VOICE = message.guild!.channels.filter(c => c.type === 'voice').size;
      const GUILD_ROLES = message.guild!.roles.filter(r => r.name !== '@everyone').map((r) => { return r.name; }).join(', ');
      const GUILD_ROLES_COUNT = message.guild!.roles.filter(r => r.name !== '@everyone').size;
      const GUILD_ROLES_HIGHEST = message.guild!.roles.highest;
      const GUILD_EMOJIS = message.guild!.emojis.size || 'No emojis.';
      const GUILD_EMOJIS_NORMAL = message.guild!.emojis.filter(e => !e.animated).size;
      const GUILD_EMOJIS_ANIMATED = message.guild!.emojis.filter(e => e.animated).size;
      const GUILD_CREATION_DATE = moment.utc(message.guild!.createdAt).format(Constants.DATE_FORMAT);
      const GUILD_SERVER_REGION = message.guild!.region ? Constants.GUILD_REGIONS[message.guild!.region]
        : message.guild!.region;
      const GUILD_VERIFICATION_LEVEL = Constants.GUILD_VERIFICATION_LEVELS[message.guild!.verificationLevel];
      const GUILD_EXPLICIT_FILTER = Constants.GUILD_EXPLICIT_FILTER[message.guild!.explicitContentFilter];
      const GUILD_VERIFIED_STATUS = message.guild!.verified ? 'Yes' : 'No';

      GUILD_EMBED.setTitle(`Information on guild ${GUILD_NAME}`);
      GUILD_EMBED.setThumbnail(message.guild!.iconURL());
      GUILD_EMBED.setColor(GUILD_ROLES_HIGHEST.hexColor);
      GUILD_EMBED.setDescription(
        `**Name**: ${GUILD_NAME}\n` +
        `**Owner**: ${GUILD_OWNER}\n` +
        `**Members**: ${GUILD_MEMBERS} (${GUILD_MEMBERS_USERS} users, ${GUILD_MEMBERS_BOTS} bots)\n` +
        `**Members Online**: ${GUILD_PRESENCES} (${GUILD_PRESENCES_USERS} users, ${GUILD_PRESENCES_BOTS} bots)\n` +
        `**Channels**: ${GUILD_CHANNELS} (${GUILD_CHANNELS_TEXT} text, ${GUILD_CHANNELS_VOICE} voice)\n` +
        `**Emojis**: ${GUILD_EMOJIS} (${GUILD_EMOJIS_NORMAL} normal, ${GUILD_EMOJIS_ANIMATED} animated)\n` +
        `**Region**: ${GUILD_SERVER_REGION}\n` +
        `**Creation Date**: ${GUILD_CREATION_DATE}\n` +
        `**Verification Level**: ${GUILD_VERIFICATION_LEVEL}\n` +
        `**Explicit Content Filter**: ${GUILD_EXPLICIT_FILTER}\n` +
        `**Verified Server?** ${GUILD_VERIFIED_STATUS}\n` +
        `**Roles (${GUILD_ROLES_COUNT})**: ${GUILD_ROLES}`,
      );
      GUILD_EMBED.setFooter(`The ID belonging to ${GUILD_NAME} is ${GUILD_ID}.`);

      message.channel.send(GUILD_EMBED);
    } else {
      return message.channel.send('Guild not currently available, try again later.');
    }
  }
}
