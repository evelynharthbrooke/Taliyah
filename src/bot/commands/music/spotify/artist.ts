/**
 * artist.ts -- Retrieves information about an artist on Spotify.
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

import { Command } from 'discord-akairo';
import { Message, MessageEmbed } from 'discord.js';

import { Util } from '../../../utils/Util';

import stripHtml from 'string-strip-html';

export default class SpotifyArtistCommand extends Command {
  public constructor() {
    super('spotify-artist', {
      category: 'Music',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Displays information about an artist on Spotify.',
        usage: '<aritst>',
      },
      args: [
        {
          id: 'artist',
          match: 'content',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { artist }: { artist: string }) {
    if (!artist) {
      const embed = new MessageEmbed();
      embed.setColor(0x1DB954);
      embed.setTitle('Error: No artist name provided.');
      embed.setDescription('You didn\'t provide an artist name. Please provide one and then '
        + 'try again!',
      );
      return message.channel.send(embed);
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);

      // I really hate having to do this, but because Spotify for some unknown reason
      // decided to not expose artist biographies in their public API. As such, I have
      // to sadly resort to using their backend API used with their app clients and web
      // player to be able to retrieve artist biographies. I really hope this will only
      // be a short-term solution, but honestly you never know with Spotify.
      const backend = 'https://spclient.wg.spotify.com/open-backend-2/v1';
      const endpoint = '/artists/';
      const url = backend + endpoint;

      this.client.spotify.searchArtists(artist, { limit: 1, offset: 0 }, (err, res) => {
        const id = res.body.artists.items[0].id;

        this.client.spotify.getArtist(id).then(async (res) => {
          const embed = new MessageEmbed();
          const agent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
            + 'Chrome/78.0.3886.0 Safari/537.36 Edg/78.0.257.0';
          const tu = await request.get('https://open.spotify.com').set({ 'User-Agent': agent });
          const token = tu.header['set-cookie'][3].split('=')[1].split(';')[0];

          const about = await request.get(url + id).set({ Authorization: 'Bearer ' + token, 'User-Agent': agent });

          let biography: string;
          const name = res.body.name;
          const genres = res.body.genres.join(', ');
          const link = res.body.external_urls.spotify;
          const image = res.body.images[0].url;
          const followers = res.body.followers.total;
          const listeners = about.body.artistInsights.monthly_listeners;
          const delta = about.body.artistInsights.monthly_listeners_delta;
          const position = about.body.artistInsights.global_chart_position;

          if (about.body.hasOwnProperty('bio')) {
            biography = stripHtml(Util.shorten(about.body.bio, 1000));
          } else {
            biography = 'No biography available.';
          }

          embed.setTitle(name);
          embed.setColor(0x1DB954);
          embed.setURL(link);
          embed.setThumbnail(image);
          embed.setDescription(
            `${biography}\n\n` +
            '**__Artist Stats__:**\n' +
            `**Chart Position**: ${position}\n` +
            `**Followers**: ${followers}\n` +
            `**Listeners**: ${listeners ? listeners : `No users listen to ${name}.`} (${delta} delta)\n` +
            `**Genres**: ${genres ? genres : 'No genres available.'}\n\n`);

          return message.channel.send(embed);
        });
      });
    });
  }
}
