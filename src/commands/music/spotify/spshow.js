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

class SpotifyPodcastCommand extends Command {
    constructor() {
        super('spotify-show', {
            category: 'Music',
            description: {
                content: 'Searches Spotify for a specified show.',
                usage: '[playlist]'
            },
            args: [
                {
                    id: 'show',
                    match: 'content',
                    type: 'string'
                }
            ]
        })
    }

    async exec(message, { show }) {
        if (!show) return message.channel.send("You didn't enter a show name. Please try again.");

        spotify.clientCredentialsGrant().then(data => {
            spotify.setAccessToken(data.body['access_token']);

            spotify.searchShows(show, { limit: 1, offset: 0 }).then(res => {
                const id = res.body.shows.items[0].id;

                spotify.getShow(id).then(res => {
                    const podcastEmbed = new MessageEmbed();

                    let episodes = res.body.episodes.items.slice(0, 5).map(episode => {
                        return `${episode.name} • ${moment(episode.release_date).format("ll")}`
                    }).join('\n');

                    let explicit = res.body.explicit ? "Yes" : "No";

                    podcastEmbed.setTitle(`Show Information for ${res.body.name}`);
                    podcastEmbed.setThumbnail(res.body.images[0].url);
                    podcastEmbed.setColor(0x1DB954);
                    podcastEmbed.setDescription(
                        `${res.body.description}\n\n` +
                        `**Publisher**: ${res.body.publisher}\n` +
                        `**Explicit Podcast?** ${explicit}\n` +
                        `**Episodes Available**: ${res.body.episodes.total}\n\n` +
                        `**Most Recent Episodes**:\n` +
                        `${episodes}`
                    );
                    podcastEmbed.setFooter("Powered by the Spotify Web API.");

                    message.channel.send(podcastEmbed);
                }).catch(error => {
                    console.log(error);
                    return message.channel.send(`Whoops! Looks like something happened while trying to ` +
                        `fetch show details for \`${show}\`. Please try again later!`)
                });
            }).catch(error => {
                console.log(error);
                return message.channel.send(`No results found for show \`${show}\`. ` +
                    `Please try a different search term.`);
            });
        });
    };
};

module.exports = SpotifyPodcastCommand;