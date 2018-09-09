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
                    type: 'number'
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
            await this.client.logger.log('info', `Purged ${pluralize('message', msgCount, true)} in ${message.channel.name}.`)
            return null;
        } catch (err) {
            console.log(err);
            this.client.logger.log('info', 'User could not delete messages older than two weeks.')
            message.channel.send('Sorry, I cannot delete messages older than two weeks!')
        }
    }
}

module.exports = PurgeCommand;
