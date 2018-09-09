/**
 * id.js -- The id command; retrieves the current user's Discord ID.
 * 
 * Copyright (c) 2018-present Kamran Mackey.
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

class IdCommand extends Command {
    constructor() {
        super('id', {
            aliases: ['id'],
            category: 'Info',
            description: {
                content: 'Retrieves your Discord user ID.'
            },
            args: [
                {
                    id: 'user',
                    match: 'content',
                    type: 'user'
                }
            ]
        });
    };

    async exec(message, { user }) {
        if (!user) {
            message.util.send(`Hi ${message.author}, your Discord ID is \`${message.author.id}\`.`);
        } else {
            message.channel.send(`${user.username}'s user ID is ${user.id}.`)
        }
    };
};

module.exports = IdCommand;