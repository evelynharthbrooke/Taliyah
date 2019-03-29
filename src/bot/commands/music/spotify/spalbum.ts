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
          match: 'content',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { album }: { album: string }) {
    // Base album embed.
    const albumEmbed = new MessageEmbed().setColor(0x1DB954);
    // Base error embed.
    const errorEmbed = new MessageEmbed().setColor(0xB00020);

    if (!album) {
      errorEmbed.setTitle('Error: No album name provided.');
      errorEmbed.setDescription('You did not provide the name of the album you would '
        + 'like to get information on. Please provide one and then try again.');

      return message.channel.send(errorEmbed);
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);

      this.client.spotify.searchAlbums(album, { limit: 1, offset: 0 }).then((resp) => {
        // The album's Spotify ID.
        const albumId = resp.body.albums.items[0].id;
        // Use getAlbum to get all album information that the normal
        // searchAlbums endpoint doesn't provide.
        this.client.spotify.getAlbum(albumId).then((res) => {
          // The name of the release.
          const albumName = res.body.name;
          // The image attached to the release.
          const albumImage = res.body.images[0].url;
          // The number of tracks on the release.
          const albumTrackCount = pluralize('track', res.body.tracks.total, true);
          // The Spotify album URL of the release.
          const albumUrl = res.body.external_urls.spotify;
          // The artists listed on the release.
          const albumArtists = res.body.artists.map(a => `[${a.name}](${a.external_urls.spotify})`).join(', ');
          // The release date of the release.
          const albumReleaseDate = moment(res.body.release_date).format('LL');
          // The copyright information associated with this release.
          let albumCopyright: string;
          // Since some artists do not provide copyright text with their
          // releases when uploading them to Spotify, work around that by
          // replacing the copyright text with the record label/individual
          // entity who published the release.
          if (Object.entries(res.body.copyrights).length === 0) {
            albumCopyright = res.body.label;
          } else {
            albumCopyright = res.body.copyrights[0].text;
          }
          // The tracks part of the release.
          const albumTracks = res.body.tracks.items.map((track) => {
            // The name of the track.
            const trackName = track.name;
            // The track number of the track.
            const trackNumber = track.track_number;
            // The URL of the track.
            const trackUrl = track.external_urls.spotify;
            // The duration of the track.
            const trackLength = moment.duration(track.duration_ms, 'milliseconds').format();
            // Whether or not the track is explict.
            const trackExplicitness = track.explicit ? ' — Explicit' : '';
            // Return the track.
            return `**${trackNumber}.** [${trackName}](${trackUrl}) ${trackExplicitness} — ${trackLength}`;
          }).join('\n');

          albumEmbed.setTitle(albumName);
          albumEmbed.setURL(albumUrl);
          albumEmbed.setThumbnail(albumImage);
          albumEmbed.setDescription(
            `**Artist(s)**: ${albumArtists}\n` +
            `**Release Date**: ${albumReleaseDate}\n` +
            `**Tracks**: ${albumTrackCount}\n\n` +
            '**Track Listing**:\n' +
            `${albumTracks}\n\n`,
          );
          albumEmbed.setFooter(`${albumCopyright} | Powered by the Spotify API.`);

          // Unfortunately, due to Discord's API limitations with regards to rich embeds,
          // do not display any albums that surpass Discord's embed description character
          // limit. I pray that this will eventually be increased, but I don't have high
          // hopes that it will. Oh well. Kind of silly how Discord imposes char limits
          // on specific components such as descriptions instead of setting a hard limit
          // too.
          if (albumEmbed.length >= 2048) {
            errorEmbed.setTitle('Error: Album tracklist too big.');
            errorEmbed.setDescription('Unfortunately, it looks like this album\'s tracklist is too ' +
              'big to display. Please try a different album.');
            message.channel.send(errorEmbed);
          } else {
            message.channel.send(albumEmbed);
          }
        }).catch((err) => {
          console.log(err);
        });
      }).catch((err) => {
        if (err.name === 'TypeError') {
          errorEmbed.setTitle('Error: Invalid album name provided.');
          errorEmbed.setDescription('You did not provide a valid album name. Please ' +
            'provide one and then try again.');

          message.channel.send(errorEmbed);
        } else {
          console.log(err);
        }
      });
    });
  }
}
