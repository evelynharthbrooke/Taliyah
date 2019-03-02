/**
 * spalbum.js -- Sub-command for Spotify that retrieves information
 * about a specified album.
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

class SpotifyAlbumCommand extends Command {
    constructor() {
        super('spotify-album', {
            category: 'Music',
            description: {
                content: 'Searches Spotify for a specified album.',
                usage: '[album]'
            },
            args: [
                {
                    id: 'album',
                    match: 'content',
                    type: 'string'
                }
            ]
        })
    }

    async exec(message, { album }) {

        if (!album) return message.channel.send("You didn't enter an album name. Please try again.")

        spotify.clientCredentialsGrant().then(data => {
            spotify.setAccessToken(data.body['access_token']);

            spotify.searchAlbums(album, { limit: 1, offset: 0 }).then(res => {
                const id = res.body.albums.items[0].id;

                spotify.getAlbum(id).then(res => {
                    let albumEmbed = new MessageEmbed();
                    let albumName = res.body.name;
                    let cover = res.body.images[1].url;
                    let artists = res.body.artists.map(artist => {
                        return `[${artist.name}](${artist.external_urls.spotify})`
                    }).join(', ');
                    let rel_date = moment(res.body.release_date).format('LL');;
                    let total_tracks = res.body.total_tracks;
                    let trackListing = res.body.tracks.items.map(track => {
                        let name = track.name;
                        let trackNumber = track.track_number;
                        let trackURL = track.external_urls.spotify;
                        let length = moment.duration(track.duration_ms,
                            "milliseconds").format('h[h] mm[m] s[s]');
                        let explicit = track.explicit ? "(Explicit)" : "";

                        return `**${trackNumber}.** [${name}](${trackURL}) - ${length} ${explicit}`;

                    }).join('\n');

                    let copyright;
                    if (Object.entries(res.body.copyrights).length === 0) {
                        copyright = "No copyright info is available for this album."
                    } else {
                        copyright = res.body.copyrights[0].text;
                    }

                    let title;
                    let tracksTitle;
                    if (total_tracks === 1) {
                        title = `Information on single ${albumName}`;
                        tracksTitle = "Single";
                    } else if (res.body.album_type === "compilation") {
                        title = `Information on compilation ${albumName}`;
                        tracksTitle = "Compilation";
                    } else if (total_tracks <= 6) {
                        title = `Information on EP ${albumName}`;
                        tracksTitle = "EP"
                    } else {
                        title = `Information on album ${albumName}`;
                        tracksTitle = "Album";
                    }

                    albumEmbed.setTitle(title);
                    albumEmbed.setURL(res.body.external_urls.spotify);
                    albumEmbed.setThumbnail(cover);
                    albumEmbed.setColor(0x1DB954);
                    albumEmbed.setDescription(
                        `**Artist(s)**: ${artists}\n` +
                        `**Release Date**: ${rel_date}\n` +
                        `**Track Count**: ${total_tracks}\n\n` +
                        `**Tracks in ${tracksTitle}**:\n` +
                        `${trackListing}`
                    );
                    albumEmbed.setFooter(`${copyright} | Information provided by the Spotify Web API.`);

                    if (albumEmbed.length >= 2048) {
                        return message.channel.send(
                            "Sorry! Looks like this album is too big to display. " + 
                            "Please try a different album.")
                    } else {
                        message.channel.send(albumEmbed);
                    }
                }).catch(error => {
                    console.log(error);
                    return message.channel.send(`Whoops! Looks like something happened while trying to ` +
                        `fetch album details for \`${album}\`. Please try again later!`)
                })
            }).catch(error => {
                console.log(error);
                return message.channel.send(`No results found for query \`${album}\`. ` +
                    `Please try a different search term.`);
            })
        })
    }
}

module.exports = SpotifyAlbumCommand;
