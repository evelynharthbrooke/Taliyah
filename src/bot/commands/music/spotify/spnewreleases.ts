/**
 * spnewreleases.ts -- Retrieves information about a region's
 * latest Spotify releases.
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

import countries from 'i18n-iso-countries';

const moment = require('moment');
require('moment-duration-format');

export default class SpotifyNewReleasesCommand extends Command {
  public constructor() {
    super('spotify-newreleases', {
      category: 'Music',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Displays information about a market\'s newest Spotify releases.',
        usage: '<market> <limit> <offset> (limit and offset are optional.)',
      },
      args: [
        {
          id: 'market',
          type: 'string',
        },
        {
          id: 'nrLimit',
          type: 'number',
          default: 20,
        },
        {
          id: 'nrOffset',
          type: 'number',
          default: 0,
        },
      ],
    });
  }

  public async exec(message: Message, { market, nrLimit, nrOffset }: {
    market: string,
    nrLimit: number,
    nrOffset: number,
  }) {
    // The base error embed.
    const errorEmbed = new MessageEmbed().setColor(0xB00020);
    // The new releases embed.
    const newReleasesEmbed = new MessageEmbed().setColor(0x1DB954);

    if (!market) {
      errorEmbed.setTitle('Error: No market name provided.');
      errorEmbed.setDescription('You did not provide the market name of where you\'d like to get the ' +
        'latest releases. Please provide a market name and then try again.\n\n' +
        '**Examples**: CA, US');
      return message.channel.send(errorEmbed);
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);

      this.client.spotify.getNewReleases({
        country: market,
        limit: nrLimit,
        offset: nrOffset,
      }).then((res) => {
        const newReleases = res.body.albums.items.map((item) => {
          // The name of the track.
          const trackName = item.name;
          // The artists on the track.
          const trackArtists = item.artists.map(artist => artist.name).join(', ');
          // The track's release date.
          const trackReleaseDate = moment(new Date(item.release_date)).format('ll');
          // Return the full string.
          return `**${trackName}** — ${trackArtists} — ${trackReleaseDate}`;
        }).join('\n');

        const marketName = countries.getName(market, 'en');

        newReleasesEmbed.setTitle(`New Releases on Spotify for ${marketName}`);
        newReleasesEmbed.setDescription(newReleases);
        newReleasesEmbed.setFooter('Powered by the Spotify API.');

        message.channel.send(newReleasesEmbed);
      });
    });

  }
}
