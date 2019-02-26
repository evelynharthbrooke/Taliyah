/**
 * spotify.js - Allows users to search Spotify's database for 
 * artists, albums, tracks, etc.
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
const { stripIndents } = require('common-tags');


class SpotifyCommand extends Command {
    constructor() {
        super('spotify', {
            aliases: ['spotify', 'sp', 'splookup', 'spot'],
            description: {
                content: stripIndents`
                    Allows you to search Spotify's musical database.

                    __**Methods and Usage**__:
                    **newreleases**: \`<market>\`
                    **track**: \`<name of track>\`
                    **playlist**: \`<playlist ID>\`
                    **album**: \`<album name>\`
                `,
                usage: '<method> <argument>',
            },
            category: 'Music',
            args: [
                {
                    id: 'method',
                    type: ['track', 'playlist', 'album', 'newreleases']
                },
                {
                    id: 'args',
                    match: 'rest',
                    default: ''
                }
            ],
        })
    }

    exec(message, { method, args }) {
        if (!method) return message.channel.send("You didn't choose a method!");

        let subcommand = {
            track: this.handler.modules.get('spotify-track'),
            playlist: this.handler.modules.get('spotify-playlist'),
            album: this.handler.modules.get('spotify-album'),
            newreleases: this.handler.modules.get('spotify-newreleases')
        }[method]

        return this.handler.handleDirectCommand(message, args, subcommand, true);
    }
}

module.exports = SpotifyCommand;
