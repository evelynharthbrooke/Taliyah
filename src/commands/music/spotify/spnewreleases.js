/**
 * spnr.js -- Sub-command for Spotify that retrieves information
 * about New Releases.
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
const moment = require('moment');
const countries = require('i18n-iso-countries');
const config = require('../../../../config.json');
// Spotify stuff
const sp_node = require('spotify-web-api-node');
const spotify = new sp_node({
    clientId: config.sp_clientid,
    clientSecret: config.sp_clientsecret,
    redirectUri: 'erica://callback'
})

class SpotifyNewReleasesCommand extends Command {
    constructor() {
        super('spotify-newreleases', {
            category: 'Music',
            description: {
                content: 'Retrieves Spotify\'s Newest Releases.',
                usage: '[market]'
            },
            args: [
                {
                    id: 'market',
                },
                {
                    id: 'limit',
                    type: 'number',
                },
                {
                    id: 'offset',
                    type: 'number'
                }
            ]
        })
    }

    async exec(message, { market, limit }) {
        if (!market) {
            return message.channel.send("You didn't enter a market locale. Examples: CA, US, or SE.")
        } else if (!limit) {
            limit = 20;
            console.log("No limit entered, defaulting to 20.");
        }
        spotify.clientCredentialsGrant().then(data => {
            spotify.setAccessToken(data.body['access_token']);
            spotify.getNewReleases({ country: market, limit: limit }).then(res => {
                const newReleasesEmbed = new MessageEmbed();
                const items = res.body.albums.items.map((item) => {
                    const artist = item.artists.map(artist => {
                        return artist.name;
                    }).join(', ')

                    const name = item.name;

                    const rel_date = moment(item.release_date).format("ll")

                    return `${name} - ${artist} - ${rel_date}\n`;
                }).join('');

                function getMarketName() {
                    return countries.getName(market, 'en').substr(0, 13);
                }

                newReleasesEmbed.setTitle(`New Releases on Spotify for ${getMarketName()}`)
                newReleasesEmbed.setDescription(items);
                newReleasesEmbed.setFooter(`Powered by the Spotify Web API.`)

                if (newReleasesEmbed.length >= 2048) {
                    return message.channel.send("Sorry, the total length of the message exceeds 2048 chars. Try a smaller limit.");
                } else {
                    return message.channel.send(newReleasesEmbed);
                }
            })
        });
    }

}

module.exports = SpotifyNewReleasesCommand;