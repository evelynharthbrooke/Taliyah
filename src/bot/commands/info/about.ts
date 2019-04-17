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

import { Command, version as DiscordAkairoVersion } from 'discord-akairo';
import { Message, MessageEmbed, version as DiscordVersion } from 'discord.js';
import { Util } from '../../utils/Util';
import moment from 'moment';
import pluralize from 'pluralize';
import { version as TypeScriptVersion } from 'typescript';

import * as packageJSON from '../../../../package.json';

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
    const aboutEmbed = new MessageEmbed();
    /** Basic Bot Information */
    const botVersion = packageJSON.version;
    const botCodename = packageJSON.codename;
    const botOwnerID = this.client.ownerID as string;
    const botOwner = this.client.users.get(botOwnerID)!.tag;
    const botMemoryUsage = (process.memoryUsage().heapUsed / 1024 / 1024).toFixed(2);
    const botUptime = moment.duration(-this.client.uptime!, 'milliseconds').humanize(true);
    const nodeVersion = Util.parseNodeVersion(process.version);
    const v8Version = process.versions.v8;
    /** Statistics */
    const channelFilter = this.client.channels.filter(channel => channel.type !== 'category').size;
    const channelCount = pluralize('channel', channelFilter, true);
    const guildCount = pluralize('guild', this.client.guilds.size, true);
    const userCount = pluralize('user', this.client.guilds.map(g => g.memberCount).reduce((f, l) => f + l), true);

    aboutEmbed.setTitle(`About ${this.client.user!.username}`);
    aboutEmbed.setColor(0x00AE86);
    aboutEmbed.setThumbnail(this.client.user!.displayAvatarURL({ format: 'png', size: 1024 }));
    aboutEmbed.setDescription(
      'Information about Ellie, such as her uptime, used libraries, etc. You can view '
      + 'her source code on GitHub [here](https://github.com/KamranMackey/Ellie/) and check her '
      + 'most recent commits by using **!changelog**. You can also view the help information for '
      + 'Ellie by using **!help**.\n\n'
      + '**__General__**:\n'
      + `**Owner**: ${botOwner}\n`
      + `**Started**: ${botUptime}\n`
      + `**Guilds**: ${guildCount}\n`
      + `**Channels**: ${channelCount}\n`
      + `**Users**: ${userCount}\n`
      + `**Version**: ${botVersion} ${botCodename}\n`
      + `**Memory Usage**: ${botMemoryUsage} MB\n\n`
      + '**__Dependencies__**:\n'
      + `**[Node.js](https://nodejs.org)**: ${nodeVersion}\n`
      + `**[V8](https://v8.dev)**: ${v8Version}\n`
      + `**[TypeScript](https://www.typescriptlang.org)**: ${TypeScriptVersion.substr(0, 9)}\n`
      + `**[Discord.js](https://github.com/discordjs/discord.js)**: ${DiscordVersion}\n`
      + `**[Akairo](https://github.com/1Computer1/discord-akairo)**: ${DiscordAkairoVersion}\n`,
    );

    return await message.channel.send(aboutEmbed);
  }
}
