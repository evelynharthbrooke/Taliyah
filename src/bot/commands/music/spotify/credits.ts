/**
 * credits.ts -- Retrieves credits on a specified Spotify track.
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

export default class SpotifyCreditsCommand extends Command {
  public constructor() {
    super('spotify-credits', {
      category: 'Music',
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Displays the credits of a specified Spotify track.',
        usage: '<track>',
      },
      args: [
        {
          id: 'track',
          match: 'content',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { track }: { track: string }) {

    if (!track) {
      return message.channel.send("You didn't input a track to get credits for. Please try again.");
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);
      this.client.spotify.searchTracks(track, { limit: 1, offset: 0 }, async (res: any) => {
        const embed = new MessageEmbed();

        try {
          const track = res.body.tracks.items[0];
          const parent = track.album;
          const id = track.id;
          const name = track.name;
          const link = track.external_urls.spotify;
          const cover = parent.images[1].url;

          const endpoint = `https://spclient.wg.spotify.com/track-credits-view/v0/track/${id}/credits`;
          const agent = 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) '
            + 'Chrome/78.0.3886.0 Safari/537.36 Edg/78.0.257.0';
          const url = await request.get('https://open.spotify.com').set({ 'User-Agent': agent });
          const token = url.header['set-cookie'][3].split('=')[1].split(';')[0];

          const credits = await request.get(endpoint).set({ Authorization: 'Bearer ' + token, 'User-Agent': agent });
          const performers = credits.body.roleCredits[0].artists.map((a: any) => a.name).join('\n');
          const writers = credits.body.roleCredits[1].artists.map((a: any) => a.name).join('\n');
          const producers = credits.body.roleCredits[2].artists.map((a: any) => a.name).join('\n');
          const source = credits.body.source.value;

          embed.setTitle(`${name} Credits`);
          embed.setThumbnail(cover);
          embed.setURL(link);
          embed.setColor(0x1DB954);
          embed.setDescription(
            '**Performed by**:\n' +
            `${performers}\n\n` +
            '**Written by**:\n' +
            `${writers}\n\n` +
            '**Produced by**:\n' +
            `${producers}`,
          );
          embed.setFooter(`Credits provided by ${source}.`);

          return message.channel.send(embed);
        } catch (error) {
          console.log(error.request);
          console.log(error.message);
        }
      });
    });
  }
}
