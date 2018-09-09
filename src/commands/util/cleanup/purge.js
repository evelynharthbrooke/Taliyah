const { Command } = require('discord-akairo');
const pluralize = require('pluralize');

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
                content: 'Purges a desired amount of messages from the current Discord guild. ' +
                         'Recommended max number of messages to delete is 99.',
                usage: '<number of messages>'
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
            await message.channel.bulkDelete(msgCount, true)
            await message.channel.send(`Successfully purged ${pluralize('message', msgCount, true)}.`)
                .then(message => {
                    message.delete({ timeout: 15000 }); // delete purged message after 30 seconds.
            });
            await this.client.logger.log('info', `Purged ${pluralize('message', msgCount, true)} in ` + 
                                                 `#${message.channel.name} in ${message.guild}.`)
            return null;
        } catch (err) {
            this.client.logger.log('info', `Unable to delete messages! ${err}`)
            message.channel.send(`Sorry, I was unable to delete any messages! ${err}`)
        }
    }
}

module.exports = PurgeCommand;
