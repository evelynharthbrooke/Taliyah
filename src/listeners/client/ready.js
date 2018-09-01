const { Listener } = require('discord-akairo');
const config = require('../../../config.json');
const { version } = require('../../../package.json');

class ReadyListener extends Listener {
    constructor() {
        super('ready', {
            emitter: 'client',
            event: 'ready',
            category: 'client'
        });
    }

    async exec() {
        this.client.logger.info(`Erica v${version} has connected to the Discord API and is ready to handle requests.`);
        this.client.logger.info('Setting activity.')
        this.client.user.setActivity(`Erica v${version} | ${config.prefix}help`);
        this.client.logger.info('Set the activity.')
    }
}

module.exports = ReadyListener;
