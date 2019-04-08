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
    const errEmbed = new MessageEmbed();

    if (!artist) {
      errEmbed.setColor(0x1DB954);
      errEmbed.setTitle('Error: No artist name provided.');
      errEmbed.setDescription('You didn\'t provide an artist name. Please provide one and then '
        + 'try again!',
      );

      return message.channel.send(errEmbed);
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);

      // I really hate having to do this, but because Spotify for some unknown reason
      // decided to not expose artist biographies in their public API. As such, I have
      // to sadly resort to using their backend API used with their app clients and web
      // player to be able to retrieve artist biographies. I really hope this will only
      // be a short-term solution, but honestly you never know with Spotify.
      const spotifyBackendUrl = 'https://spclient.wg.spotify.com/open-backend-2/v1';
      const spotifyBackendEndpoint = '/artists/';
      const spotifyBackendFullUrl = spotifyBackendUrl + spotifyBackendEndpoint;

      this.client.spotify.searchArtists(artist, { limit: 1, offset: 0 }, (err, res) => {
        const artistId = res.body.artists.items[0].id;

        this.client.spotify.getArtist(artistId).then(async (res) => {
          const artistEmbed = new MessageEmbed();
          const userAgent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
            + 'Chrome/75.0.3759.0 Safari/537.36';
          const accessTokenUrl = await request.get('https://open.spotify.com').set({ 'User-Agent': userAgent });
          // TODO: Store token and only refresh the token when it expires.
          const accessToken = accessTokenUrl.header['set-cookie'][4].split('=')[1].split(';')[0];

          const artistAbout = await request.get(spotifyBackendFullUrl + artistId).set({
            Authorization: 'Bearer ' + accessToken,
            'User-Agent': userAgent,
          });

          // Artist Information
          let artistBiography: string;
          const artistName = res.body.name;
          const artistGenres = res.body.genres.join(', ');
          const artistLink = res.body.external_urls.spotify;
          const artistImage = res.body.images[0].url;
          // Statistics
          const artistFollowers = res.body.followers.total;
          const artistListeners = artistAbout.body.artistInsights.monthly_listeners;

          if (artistAbout.body.hasOwnProperty('bio')) {
            artistBiography = stripHtml(Util.shorten(artistAbout.body.bio, 1000));
          } else {
            artistBiography = 'No biography available.';
          }

          artistEmbed.setTitle(artistName);
          artistEmbed.setColor(0x1DB954);
          artistEmbed.setURL(artistLink);
          artistEmbed.setThumbnail(artistImage);
          artistEmbed.setDescription(
            `${artistBiography}\n\n` +
            '**__Artist Stats__:**\n' +
            `**Followers**: ${artistFollowers}\n` +
            `**Listeners**: ${artistListeners ? artistListeners : `No users listen to ${artistName}.`}\n` +
            `**Genres**: ${artistGenres ? artistGenres : 'No genres available.'}\n\n`);

          return message.channel.send(artistEmbed);
        });
      });
    });
  }
}
