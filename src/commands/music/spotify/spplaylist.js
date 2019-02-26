/**
 * spplaylist.js -- Sub-command for Spotify that retrieves information
 * about a specified playlist.
 *
 * Copyright (c) 2019-present Kamran Mackey.
 *
 * Erica is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Erica is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Erica. If not, see <https://www.gnu.org/licenses/>.
 */

const { Command } = require('discord-akairo');
const { MessageEmbed } = require('discord.js');
const sanitizeHTML = require('sanitize-html');
const config = require('../../../../config.json');
const moment = require('moment');
require("moment-duration-format");
// Spotify stuff
const sp_node = require('spotify-web-api-node');
const spotify = new sp_node({
    clientId: config.sp_clientid,
    clientSecret: config.sp_clientsecret,
    redirectUri: 'erica://callback'
})

class SpotifyPlaylistCommand extends Command {
    constructor() {
        super('spotify-playlist', {
            category: 'Music',
            description: {
                content: 'Searches Spotify for a specified playlist.',
                usage: '[playlist]'
            },
            args: [
                {
                    id: 'playlist',
                    match: 'content',
                    type: 'string'
                }
            ]
        })
    }

    async exec(message, { playlist }) {
        spotify.clientCredentialsGrant().then(data => {
            spotify.setAccessToken(data.body['access_token']);
            
            if (!playlist) {
                return message.channel.send("Please enter a playlist ID.")
            }
            
            // Get playlist metadata.
            spotify.getPlaylist(playlist).then(res => {
                let name = res.body.name;
                let owner = res.body.owner.display_name;
                let ownerURL = res.body.owner.external_urls.spotify;
                let tracks = res.body.tracks.total;
                let trackList = res.body.tracks.items.slice(0, 50).map(tracks => {
                    let length = moment.duration(tracks.track.duration_ms, 
                        "milliseconds").format('h[h] mm[m] s[s]');

                    return `${tracks.track.name} - ${length}`;
                }).join("\n");
                let desc = sanitizeHTML(res.body.description);
                let image = res.body.images[0].url;
                let publicity = res.body.public;
                let collaborative = res.body.collaborative;

                if (collaborative === false) {
                    collaborative = "No";
                } else {
                    collaborative = "Yes";
                };

                if (publicity === true) {
                    publicity = "Public";
                } else {
                    publicity = "Private";
                };

                let spPlaylistEmbed = new MessageEmbed();

                spPlaylistEmbed.setTitle(`Information on playlist ${name}`);
                spPlaylistEmbed.setThumbnail(image);
                spPlaylistEmbed.setDescription(`${desc}\n\n` + 
                                               `**Owner**: [${owner}](${ownerURL})\n` +
                                               `**Tracks**: ${tracks}\n` +
                                               `**Collaborative?** ${collaborative}\n` +
                                               `**Publicity:** ${publicity}\n\n` +
                                               `**Tracks in Playlist (up to 50)**:\n` +
                                               `${trackList}`);
                spPlaylistEmbed.setFooter('Information provided by the Spotify Web API.');
                message.channel.send(spPlaylistEmbed);
            })
        })
    }
}

module.exports = SpotifyPlaylistCommand;