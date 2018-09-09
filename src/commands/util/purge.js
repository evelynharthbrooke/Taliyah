/**
 * purge.js -- The purge command.
 * 
 * This command purges up to 100 messages from a channel in a Discord guild.
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
const pluralize = require('pluralize');
const strings = require('../../../strings.json');

class PurgeCommand extends Command {
    constructor() {
        super('purge', {
            aliases: ['purge', 'prune', 'clear'],
            category: 'Utilities',
            clientPermissions: ['READ_MESSAGE_HISTORY', 'MANAGE_MESSAGES'],
            userPermissions: ['MANAGE_MESSAGES'],
            cooldown: 15000,
            ratelimit: 3,
            description: {
                content: strings.purge_description,
                usage: strings.purge_usage
            },
            args: [
                {
                    id: 'msgCount',
                    match: 'content',
                    type: 'number',
                    prompt: {
                        'start': 'Enter the number of messages you\'d like to purge.',
                        'retry': 'You did not enter a number. Please enter a number and try again.'
                    }
                }
            ]
        })
    }

    async exec(message, { msgCount }) {
        try {
            if (msgCount > 99) {
                message.channel.send('You cannot delete more than 99 messages. Try again with a smaller number.')
            } else {
                await message.channel.bulkDelete(msgCount + 1, true)
                await message.channel.send(`Purging, please wait...`)
                    .then(message => {
                        message.edit(`Successfully purged ${pluralize('message', msgCount, true)}.`)
                        message.delete({ timeout: 15000 }); // delete purged message after 30 seconds.
                    });
                await this.client.logger.log('info', `Purged ${pluralize('message', msgCount, true)} in ` + 
                                                     `#${message.channel.name} in ${message.guild}.`)
                return null;
            }
        } catch (err) {
            this.client.logger.log('info', `Unable to delete messages!\n\n${err}`)
            message.channel.send(`Sorry, I was unable to delete any messages!\n\n\`\`\`${err}\`\`\``)
        }
    }
}

module.exports = PurgeCommand;
