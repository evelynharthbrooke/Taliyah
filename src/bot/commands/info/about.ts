/**
 * about.ts -- Retrieves information about the bot such as uptime,
 * servers, channels, etc.
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

import { Command, version as DISCORD_AKAIRO_VERSION } from 'discord-akairo';
import { version as DISCORDJS_VERSION, Message, MessageEmbed } from 'discord.js';

import { version as BOT_VERSION } from '../../../../package.json';
import { version as TYPESCRIPT_VERSION } from 'typescript';
import moment from 'moment';
import pluralize from 'pluralize';

export default class AboutCommand extends Command {
  public constructor() {
    super('about', {
      aliases: ['about', 'info'],
      category: 'Information',
      description: {
        content: 'Retrieves information about the bot such as uptime, version, etc.',
      },
    });
  }

  public async exec(message: Message) {
    const INFO_EMBED = new MessageEmbed();
    const CHANNELS = this.client.channels.filter(channel => channel.type !== 'category').size;
    const CHANNEL_COUNT = pluralize('channel', CHANNELS, true);
    const GUILD_COUNT = pluralize('guild', this.client.guilds.size, true);
    const USER_COUNT = pluralize('user', this.client.guilds.map(g => g.memberCount).reduce((f, l) => f + l), true);
    const BOT_UPTIME = moment.duration(-this.client.uptime, 'milliseconds').humanize(true);
    const MEMORY_USAGE = (process.memoryUsage().heapUsed / 1024 / 1024).toFixed(2);
    const BOT_OWNER_ID = this.client.ownerID as string;
    const BOT_OWNER = this.client.users.get(BOT_OWNER_ID)!.tag;
    const NODEJS_VERSION = process.version.substr(0, 8).replace('v', '');
    const V8_VERSION = process.versions.v8;

    INFO_EMBED.setTitle(`About ${this.client.user!.username}`);
    INFO_EMBED.setColor(0x00AE86);
    INFO_EMBED.setThumbnail(this.client.user!.displayAvatarURL({ format: 'png', size: 1024 }));
    INFO_EMBED.setDescription(
      'Information about Ellie, such as her uptime, used libraries, etc. You can view '
      + 'her source code on GitHub [here](https://github.com/KamranMackey/Ellie/) and check her '
      + 'most recent commits by using **!changelog**. You can also view the help information for '
      + 'Ellie by using **!help**.\n\n'
      + '**__General__**:\n'
      + `**Owner**: ${BOT_OWNER}\n`
      + `**Started**: ${BOT_UPTIME}\n`
      + `**Guilds**: ${GUILD_COUNT}\n`
      + `**Channels**: ${CHANNEL_COUNT}\n`
      + `**Users**: ${USER_COUNT}\n`
      + `**Version**: ${BOT_VERSION}\n`
      + `**Memory Usage**: ${MEMORY_USAGE} MB\n\n`
      + '**__Dependencies__**:\n'
      + `**[Node.js](https://nodejs.org)**: ${NODEJS_VERSION}\n`
      + `**[V8](https://v8.dev)**: ${V8_VERSION}\n`
      + `**[TypeScript](https://www.typescriptlang.org)**: ${TYPESCRIPT_VERSION.substr(0, 9)}\n`
      + `**[Discord.js](https://github.com/discordjs/discord.js)**: ${DISCORDJS_VERSION}\n`
      + `**[Akairo](https://github.com/1Computer1/discord-akairo)**: ${DISCORD_AKAIRO_VERSION}\n`,
    );

    return message.channel.send(INFO_EMBED);
  }
}
