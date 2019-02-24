/**
 * kick.js -- The kick command.
 * 
 * Kicks a user from the current Discord guild.
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

class KickCommand extends Command {
    constructor() {
        super('kick', {
            aliases: ['kick'],
            category: 'Moderation',
            description: {
                content: "Kicks a user from the current Discord server.",
                usage: '<member>'
            },
            args: [
                {
                    id: 'member',
                    type: 'member'
                },
                {
                    id: 'reason',
                    match: 'rest',
                    type: 'string'
                }
            ]
        })
    }

    async exec(message, { member, reason }) {
        member = message.mentions.members.first();
        if (!member) {
            return message.channel.send("You didn't mention a member to kick!");
        } else if (!reason) {
            return message.channel.send("You didn't give a reason as to why you want to kick this user!");
        } else {
            await member.kick(reason).then(() => {
                message.channel.send(
                    `Successfully kicked **${member.user.tag}!\n\n` +
                    `__**Reason**__: ${reason}`
                )
            }).catch(err => {
                message.channel.send(`I was unable to kick ${member.user.tag}!`);
                // Log the error
                console.error(err);
            });
        };
    };
};

module.exports = KickCommand;