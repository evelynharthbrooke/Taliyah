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

import { Command, version as akairo } from 'discord-akairo';
import { Message, MessageEmbed, version as discord } from 'discord.js';
import { Util } from '../../utils/Util';
import moment from 'moment';
import pluralize from 'pluralize';
import { version as typescript } from 'typescript';

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
    const embed = new MessageEmbed();
    // Bot Information
    const version = packageJSON.version;
    const codename = packageJSON.codename;
    const ownerID = this.client.ownerID as string;
    const owner = this.client.users.get(ownerID)!.tag;
    const memory = (process.memoryUsage().heapUsed / 1024 / 1024).toFixed(2);
    const uptime = moment.duration(-this.client.uptime!, 'milliseconds').humanize(true);
    const node = Util.parseNodeVersion(process.version);
    const v8 = process.versions.v8;
    // Statistics
    const filter = this.client.channels.filter(channel => channel.type !== 'category').size;
    const channels = pluralize('channel', filter, true);
    const guilds = pluralize('guild', this.client.guilds.size, true);
    const users = pluralize('user', this.client.guilds.map(g => g.memberCount).reduce((f, l) => f + l), true);

    embed.setTitle(`About ${this.client.user!.username}`);
    embed.setColor(0x00AE86);
    embed.setThumbnail(this.client.user!.displayAvatarURL({ format: 'png', size: 1024 }));
    embed.setDescription(
      'Information about Ellie, such as her uptime, used libraries, etc. You can view '
      + 'her source code on GitHub [here](https://github.com/KamranMackey/Ellie/) and check her '
      + 'most recent commits by using **!changelog**. You can also view the help information for '
      + 'Ellie by using **!help**.\n\n'
      + '**__General__**:\n'
      + `**Owner**: ${owner}\n`
      + `**Started**: ${uptime}\n`
      + `**Guilds**: ${guilds}\n`
      + `**Channels**: ${channels}\n`
      + `**Users**: ${users}\n`
      + `**Version**: ${version} ${codename}\n`
      + `**Memory Usage**: ${memory} MB\n\n`
      + '**__Dependencies__**:\n'
      + `**[Node.js](https://nodejs.org)**: ${node}\n`
      + `**[V8](https://v8.dev)**: ${v8}\n`
      + `**[TypeScript](https://www.typescriptlang.org)**: ${typescript.substring(0, 9)}\n`
      + `**[Discord.js](https://github.com/discordjs/discord.js)**: ${discord}\n`
      + `**[Akairo](https://github.com/1Computer1/discord-akairo)**: ${akairo}\n`,
    );

    return await message.channel.send(embed);
  }
}
