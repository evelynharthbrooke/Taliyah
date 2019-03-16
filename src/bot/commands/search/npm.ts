/**
 * npm.ts -- Retrieves information on a specified module hosted on
 * the NPM public registry.
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

import * as request from 'superagent';

import { Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';
import Constants from '../../util/Constants';
import moment from 'moment';

export default class NPMCommand extends Command {
  public constructor() {
    super('npm', {
      aliases: ['npm'],
      category: 'Search',
      description: {
        content: 'Retrieves information on the specified `npm` package.',
        usage: '<package name>',
      },
      args: [
        {
          id: 'query',
          match: 'content',
        },
      ],
    });
  }

  public async exec(message: Message, { query }: { query: string }) {
    const NPM_QUERY = query.toLowerCase();

    if (!NPM_QUERY) {
      return message.channel.send('You did not enter a name of an npm package.');
    }

    if (NPM_QUERY.startsWith('@types')) {
      return message.channel.send('Unfortunately, I cannot display information for TypeScript typings. ' +
        'Please try a different package.');
    }

    try {
      const NPM_REGISTRY_URL = `https://registry.npmjs.com/${NPM_QUERY}`;
      const { body: NPM_REQUEST } = await request.get(NPM_REGISTRY_URL);
      const NPM_EMBED = new MessageEmbed();
      const NPM_LOGO = 'https://raw.githubusercontent.com/npm/logos/master/npm%20square/n-large.png';
      const NPM_PACKAGE_NAME = NPM_REQUEST.name;
      const NPM_PACKAGE_DESCRIPTION = NPM_REQUEST.description || 'No description available.';
      const NPM_PACKAGE_URL = 'https://www.npmjs.com/package/' + query;
      const NPM_PACKAGE_VERSION_LATEST = NPM_REQUEST['dist-tags'].latest;
      const NPM_PACKAGE_VERSION_RC = NPM_REQUEST['dist-tags'].rc || 'No version available.';
      const NPM_PACKAGE_VERSION_NEXT = NPM_REQUEST['dist-tags'].next || 'No version available.';
      const NPM_PACKAGE_VERSIONS = Object.keys(NPM_REQUEST.versions).length;
      const NPM_PACKAGE_LICENSE = NPM_REQUEST.license || 'No license available.';
      const NPM_PACKAGE_AUTHOR = NPM_REQUEST.author ? NPM_REQUEST.author.name : 'No author found.';
      const NPM_PACKAGE_CREATION_DATE = moment(NPM_REQUEST.time.created).format(Constants.DATE_FORMAT);
      const NPM_PACKAGE_LAST_MODIFIED = moment(NPM_REQUEST.time.modified).format(Constants.DATE_FORMAT);
      const NPM_PACKAGE_WEBSITE = `[Click here](${NPM_REQUEST.homepage})` || 'No website URL available.';
      const NPM_PACKAGE_BUGS = `[Click here](${NPM_REQUEST.bugs.url})` || 'No bug tracker URL available.';
      const NPM_PACKAGE_MAIN_FILE = NPM_REQUEST.versions[NPM_REQUEST['dist-tags'].latest].main || 'Not available.';
      const NPM_PACKAGE_MAINTAINERS = NPM_REQUEST.maintainers.map((user: any) => user.name).join(', ');

      if (NPM_REQUEST.time.unpublished) {
        return message.channel.send('Looks like this package is no longer available on the npm registry.' +
          'Please try a different package.');
      }

      NPM_EMBED.setAuthor('npm', NPM_LOGO, 'https://www.npmjs.com/');
      NPM_EMBED.setTitle(NPM_PACKAGE_NAME);
      NPM_EMBED.setColor(0xCC3534);
      NPM_EMBED.setURL(NPM_PACKAGE_URL);
      NPM_EMBED.setThumbnail(NPM_LOGO);
      NPM_EMBED.setDescription(
        `${NPM_PACKAGE_DESCRIPTION}\n\n` +
        `**Latest Version**: ${NPM_PACKAGE_VERSION_LATEST}\n` +
        `**RC Version**: ${NPM_PACKAGE_VERSION_RC}\n` +
        `**Next Version**: ${NPM_PACKAGE_VERSION_NEXT}\n` +
        `**Total Versions**: ${NPM_PACKAGE_VERSIONS}\n` +
        `**License**: ${NPM_PACKAGE_LICENSE}\n` +
        `**Author**: ${NPM_PACKAGE_AUTHOR}\n` +
        `**Creation Date**: ${NPM_PACKAGE_CREATION_DATE}\n` +
        `**Last Modified**: ${NPM_PACKAGE_LAST_MODIFIED}\n` +
        `**Website**: ${NPM_PACKAGE_WEBSITE}\n` +
        `**Bug Tracker**: ${NPM_PACKAGE_BUGS}\n` +
        `**Main File**: ${NPM_PACKAGE_MAIN_FILE}\n` +
        `**Maintainers**: ${NPM_PACKAGE_MAINTAINERS}`,
      );
      NPM_EMBED.setFooter('Powered by the npm registry API.');
      NPM_EMBED.setTimestamp();

      message.channel.send(NPM_EMBED);

    } catch (e) {
      if (e.status === 404) {
        return message.channel.send(`I couldn't find \`${query}\` in the npm registry. Please try a ` +
          'differrent package name.');
      }

      this.client.logger.error('Encountered an error while getting npm registry results.');
      console.log(e);
      message.channel.send('I have encountered an error! Please try again later.');
    }
  }
}
