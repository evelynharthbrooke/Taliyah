/**
 * github.js -- Main file for handling GitHub retrieval commands.
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

class GitHubCommand extends Command {
    constructor() {
        super('github', {
            aliases: ['github', 'gh'],
            category: 'Search',
            description: {
                content: stripIndents`
                    Allows users to retrieve GitHub information, like
                    users and repositories.

                    __**Methods and Usage**__:
                    **user**: \`<username>\`
                    **repo**: \`<repository name>\`
                `,
                usage: '<method> <argument>',
            },
            args: [
                {
                    id: 'method',
                    type: ['user', 'repo']
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
            user: this.handler.modules.get('github-user'),
            repo: this.handler.modules.get('github-repo')
        }[method];

        return this.handler.handleDirectCommand(message, args, subcommand, true);
    }
}

module.exports = GitHubCommand;
