/**
 * youtube.js -- Main file for handling YouTube retrieval commands.
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

class YouTubeCommand extends Command {
    constructor() {
        super('youtube', {
            aliases: ['youtube', 'yt'],
            category: 'Search',
            description: {
                content: stripIndents`
                    Allows users to search for YouTube content
                    such as videos and channels.

                    __**Methods and Usage**__:
                    **video**: \`<video name>\`
                    **channel**: \`<channel name>\`
                `,
                usage: '<method> <argument>',
            },
            args: [
                {
                    id: 'method',
                    type: ['video', 'channel']
                },
                {
                    id: 'args',
                    match: 'rest',
                    default: ''
                }
            ],
        })
    }

    async exec(message, { method, args }) {
        if (!method) return message.channel.send("You did not pick a method.");

        let subcommand = {
            channel: this.handler.modules.get('youtube-channel'),
            video: this.handler.modules.get('youtube-video')
        }[method];

        return this.handler.handleDirectCommand(message, args, subcommand, true);
    }
}

module.exports = YouTubeCommand;