/**
 * sptrack.js -- Sub-command for Spotify that retrieves information
 * about a specified track.
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


class SpotifyTrackCommand extends Command {
    constructor() {
        super('spotify-track', {
            category: 'Music',
            description: {
                content: 'Searches Spotify for a specified track.',
                usage: '[track]'
            },
            args: [
                {
                    id: 'track',
                    match: 'content',
                    type: 'string'
                }
            ]
        })
    }

    async exec(message, { track }) {
        spotify.clientCredentialsGrant().then(data => {
            spotify.setAccessToken(data.body['access_token']);
            spotify.searchTracks(track, { limit: 1, offset: 0 }, (err, res) => {
                if (!track) {
                    return message.channel.send("You didn't enter a track name! Please enter one and then try again.");
                } else if (err) {
                    console.log(err)
                    return message.channel.send("Sorry, looks like something went wrong! Maybe try again?");
                } else {
                    try {
                        let title = res.body.tracks.items[0].name;
                        let rel_date = moment(res.body.tracks.items[0].album.release_date).format('LL');
                        let artist = res.body.tracks.items[0].artists.map(artist => { 
                            return `[${artist.name}](${artist.external_urls.spotify})` 
                        }).join(', ');
                        let album = res.body.tracks.items[0].album.name;
                        let explicit = res.body.tracks.items[0].explicit ? "Yes" : "No";

                        let length = moment.duration(res.body.tracks.items[0].duration_ms, "milliseconds").format('h[h] mm[m] s[s]');
                        let cover = res.body.tracks.items[0].album.images[1].url;
                        let playback_url = res.body.tracks.items[0].external_urls.spotify;

                        const spotifyEmbed = new MessageEmbed();
                        spotifyEmbed.setTitle(`Track Information for ${title}`);
                        spotifyEmbed.setThumbnail(cover);
                        spotifyEmbed.setColor(0x1DB954);
                        spotifyEmbed.setDescription(
                            `**Artist(s)**: ${artist}\n` +
                            `**Album**: ${album}\n` +
                            `**Explicit?** ${explicit}\n` +
                            `**Release Date**: ${rel_date}\n` +
                            `**Duration**: ${length}\n\n` +
                            `[Play ${title} on Spotify →](${playback_url})`)
                        spotifyEmbed.setFooter(`Information provided by the Spotify Web API.`);

                        message.channel.send(spotifyEmbed);
                    } catch {
                        message.channel.send("Sorry, I either couldn't find that, or something else happened. Perhaps try again?")
                    }
                }
            })
        }, err => {
            console.log('Something went wrong when retrieving an access token', err);
        }).catch(() => {
            return message.channel.send("Sorry, an error occurred and I cannot continue with your request. Try again later.")
        });
    }
}

module.exports = SpotifyTrackCommand;