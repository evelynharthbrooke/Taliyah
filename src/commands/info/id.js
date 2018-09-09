const { Command } = require('discord-akairo');

class IdCommand extends Command {
    constructor() {
        super('id', {
            aliases: ['id'],
            category: 'Info',
            description: {
                content: 'Retrieves your Discord user ID.'
            }
        });
    };

    async exec(message) {
        message.util.send(`Hi ${message.author}, your Discord ID is \`${message.author.id}\`.`);
    };
};

module.exports = IdCommand;