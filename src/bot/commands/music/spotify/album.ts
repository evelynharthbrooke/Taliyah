/**
 * album.ts -- Retrieves information about an album available on the
 * Spotify platform.
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

const moment = require('moment');
require('moment-duration-format');
import pluralize from 'pluralize';

export default class SpotifyAlbumCommand extends Command {
  public constructor() {
    super('spotify-album', {
      category: 'Music',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Displays information about a specified album on Spotify.',
        usage: '<album>',
      },
      args: [
        {
          id: 'album',
          match: 'rest',
          type: 'string',
        },
        {
          id: 'market',
          type: 'string',
          match: 'option',
          flag: ['--market', '-m'],
          default: 'US',
        },
      ],
    });
  }

  public async exec(message: Message, { album, market }: { album: string, market: string }) {
    const embed = new MessageEmbed().setColor(0x1DB954);
    const error = new MessageEmbed().setColor(0xB00020);

    if (!album) {
      error.setTitle('Error: No album name provided.');
      error.setDescription('You did not provide the name of the album you would '
        + 'like to get information on. Please provide one and then try again.');

      return message.channel.send(error);
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);
      this.client.spotify.searchAlbums(album, { market, limit: 1, offset: 0 }).then((resp) => {

        const id = resp.body.albums.items[0].id;

        this.client.spotify.getAlbum(id).then((res) => {
          const name = res.body.name;
          const image = res.body.images[0].url;
          const count = pluralize('track', res.body.tracks.total, true);
          const link = res.body.external_urls.spotify;
          const artists = res.body.artists.map(a => `[${a.name}](${a.external_urls.spotify})`).join(', ');
          const released = moment(res.body.release_date).format('LL');

          let copyright: string;
          if (Object.entries(res.body.copyrights).length === 0) {
            copyright = res.body.label;
          } else {
            copyright = res.body.copyrights[0].text;
          }

          const tracklist = res.body.tracks.items.map((track) => {
            const name = track.name;
            const number = track.track_number;
            const link = track.external_urls.spotify;
            const length = moment.duration(track.duration_ms, 'milliseconds').format();
            const explicit = track.explicit ? ' — Explicit' : '';
            return `**${number}.** [${name}](${link}) ${explicit} — ${length}`;
          }).join('\n');

          embed.setTitle(name);
          embed.setURL(link);
          embed.setThumbnail(image);
          embed.setDescription(
            `**Artist(s)**: ${artists}\n` +
            `**Release Date**: ${released}\n` +
            `**Tracks**: ${count}\n\n` +
            '**Tracklist**:\n' +
            `${tracklist}\n\n`,
          );
          embed.setFooter(`${copyright} | Powered by the Spotify API.`);

          // Unfortunately, due to Discord's API limitations with regards to rich embeds,
          // do not display any albums that surpass Discord's embed description character
          // limit. I pray that this will eventually be increased, but I don't have high
          // hopes that it will. Oh well. Kind of silly how Discord imposes char limits
          // on specific components such as descriptions instead of setting a hard limit
          // too.
          if (embed.length >= 2048) {
            error.setTitle('Error: Album tracklist too big.');
            error.setDescription('Unfortunately, it looks like this album\'s tracklist is too ' +
              'big to display. Please try a different album.');
            message.channel.send(error);
          } else {
            message.channel.send(embed);
          }
        }).catch((err) => {
          console.log(err);
        });
      }).catch((err) => {
        if (err.name === 'TypeError') {
          error.setTitle('Error: Invalid album name provided.');
          error.setDescription('You did not provide a valid album name. Please ' +
            'provide one and then try again.');
          message.channel.send(error);
        } else {
          console.log(err);
        }
      });
    });
  }
}
