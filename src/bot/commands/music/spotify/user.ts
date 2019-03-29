/**
 * user.ts -- Retrieves information on the specified Spotify user.
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

import pluralize from 'pluralize';

export default class SpotifyUserCommand extends Command {
  public constructor() {
    super('spotify-user', {
      category: 'Music',
      description: {
        content: 'Gets information on a specified Spotify user ID.',
        usage: '<user>',
      },
      args: [
        {
          id: 'user',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message, { user }: { user: string }) {
    const userEmbed = new MessageEmbed().setColor(0x1DB954);
    const errorEmbed = new MessageEmbed().setColor(0xB00020);

    if (!user) {
      errorEmbed.setTitle('Error: No username provided.');
      errorEmbed.setDescription(
        'You did not provide the username of the user you would ' +
        'like to get information on. Please provide one, and then try again.\n\n' +
        '**Examples**: mkbhd, emmablackery91',
      );
      return message.channel.send(errorEmbed);
    }

    this.client.spotify.clientCredentialsGrant().then((data) => {
      this.client.spotify.setAccessToken(data.body['access_token']);

      this.client.spotify.getUser(user).then(async (res) => {
        // The ID of the user.
        const userId = res.body.id;
        // The display name of the user.
        const userDisplayName = res.body.display_name;
        // The Spotify URL of the user.
        const userUrl = res.body.external_urls.spotify;
        // The follower count of this user.
        const userFollowers = res.body.followers.total;
        // The profile picture of the user.
        const userPicture = res.body.images[0].url;
        // The playlists the user has followed or created.
        const userPlaylists = await this.client.spotify.getUserPlaylists(user).then((res) => {
          const playlists = res.body.items.map((item) => {
            const playlistName = `[${item.name}](${item.external_urls.spotify})`;
            const playlistTracks = pluralize('tracks', item.tracks.total, true);

            return `${playlistName} — ${playlistTracks}`;
          }).join('\n');

          const playlistCount = pluralize('playlist', res.body.total, true);

          return `${playlists}\n\n` + `**Total playlists**: ${playlistCount}`;
        });

        userEmbed.setTitle(userDisplayName);
        userEmbed.setURL(userUrl);
        userEmbed.setThumbnail(userPicture);
        userEmbed.setDescription(
          `**Followers**: ${userFollowers}\n\n` +
          '**User Playlists**:\n' +
          `${userPlaylists}`,
        );
        userEmbed.setFooter(`User ID: ${userId} | Powered by the Spotify API.`);

        await message.channel.send(userEmbed);
      }).catch((err) => {
        if (err.statusCode === 404) {
          errorEmbed.setTitle('Error: Username/User ID could not be found.');
          errorEmbed.setDescription(
            'I was unable to locate this user in Spotify\'s database. Please ' +
            'try a different username.' ,
          );
          message.channel.send(errorEmbed);
        } else {
          console.log(err);
        }
      });
    }).catch((err) => {
      console.log(err);
    });
  }
}
