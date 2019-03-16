/**
 * lastfm.ts -- Retrieve's a user's Last.fm state, along with the user's
 * information.
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
import Constants from '../../../util/Constants';
import moment from 'moment';
import numeral from 'numeral';

export default class LastFMCommand extends Command {
  public constructor() {
    super('lastfm', {
      aliases: ['lastfm', 'fm', 'lfm'],
      category: 'Music',
      userPermissions: ['EMBED_LINKS'],
      clientPermissions: ['EMBED_LINKS'],
      description: {
        content: 'Retrieves a user\'s Last.fm state, along with user info.',
        usage: '<user>',
      },
      args: [
        {
          id: 'member', // the last.fm member.
          match: 'content',
        },
      ],
    });
  }

  public async exec(message: Message, { member }: { member: string }) {
    if (!member) {
      message.channel.send("Looks like you haven't entered a last.fm username!");
      return;
    }

    const lastfmBase = 'https://ws.audioscrobbler.com/2.0/?method=';
    const lastfmKey = this.client.config.lastfm;
    const lastfmQuery = `&user=${member}&api_key=${lastfmKey}&limit=1&format=json`;
    const recentTracks = 'user.getRecentTracks';
    const userInfo = 'user.getInfo';

    await request.get(lastfmBase + recentTracks + lastfmQuery).then(async (res) => {
      const embed = new MessageEmbed();
      const track = res.body.recenttracks.track[0];
      const trackName = track.name;
      const trackArtist = track.artist['#text'];
      const trackAlbum = track.album['#text'];

      let playbackState: string;
      if (track.hasOwnProperty('@attr')) {
        playbackState = 'is currently listening to';
      } else {
        playbackState = 'last listened to';
      }

      const userRequest = await request.get(lastfmBase + userInfo + lastfmQuery);
      const user = userRequest.body.user;
      const userName = user.name;
      const userUrl = user.url;
      const userPlaycount = numeral(user.playcount).format('0.0a');
      const userCountry = user.country;
      const userJoinDate = moment.unix(user.registered.unixtime).format(Constants.DATE_FORMAT);
      const userStatistics =
        `**Scrobbles**: ${userPlaycount}\n` +
        `**Country**: ${userCountry}\n` +
        `**Join Date**: ${userJoinDate}`;

      embed.setTitle(`${userName}'s Last.fm`);
      embed.setURL(userUrl);
      embed.setColor(0xd51007);
      embed.setDescription(
        `${userName} ${playbackState} ${trackName} by ${trackArtist} on ${trackAlbum}.\n\n` +
        userStatistics,
      );
      embed.setFooter('Powered by the Last.fm API.');

      this.client.spotify.clientCredentialsGrant().then((data) => {
        this.client.spotify.setAccessToken(data.body['access_token']);
        this.client.spotify.searchTracks(`${trackName} ${trackArtist}`, { limit: 1 }).then((res) => {

          if (res.body.tracks.items.length === 0) {
            this.client.logger.info('Cannot find anything on Spotify for this track.');
          } else {
            embed.setThumbnail(res.body.tracks.items[0].album.images[1].url);
          }

          return message.channel.send(embed);

        });
      });
    }).catch((err) => {

      if (err.status === 404) {
        return message.channel.send('I was unable to find this last.fm user! Please try a different username.');
      }

      return message.channel.send('Sorry! Looks like I encountered an error. Please try again later.');

    });
  }
}
