/**
 * EricaClient.js -- The Erica client.
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
const { AkairoClient, CommandHandler, ListenerHandler } = require('discord-akairo');
const { createLogger, transports, format } = require('winston');
const path = require('path');
const { version } = require('../../package.json');

class EricaClient extends AkairoClient {
    constructor(config) {
        super({ ownerID: config.owner }, {
            disableEveryone: true,
        });

        // Setup a logger using the Winston library so that 
        // way we can do, well, logging related tasks such 
        // as logging messages and for debugging anything 
        // that needs to be debugged.
        this.logger = createLogger({
            format: format.combine(
                format.colorize(),
                format.timestamp({ format: 'MMM D YYYY HH:mm:ss' }),
				format.printf(info => `[${info.timestamp}] ${info.level}: ${info.message}`)
            ),
            transports: [new transports.Console()]
        })

        // Initialize the command handler.
        this.logger.log('info', 'Initializing the command handler.');
        this.commandHandler = new CommandHandler(this, {
            directory: path.join(__dirname, '..', 'commands'),
            prefix: config.prefix,
            aliasReplacement: /-/g,
            commandUtil: true,
            allowMention: true,
            handleEdits: true
        });

        // Initialize the listener handler.
        this.logger.log('info', 'Initializing the listener handler.');
        this.listenerHandler = new ListenerHandler(this, {
            directory: path.join(__dirname, '..', 'listeners')
        });

        this.config = config;

        // Run the setup function to setup the command handler, the
        // listener handler, as well as load all commands and listeners.
        this.setup();
    };

    setup() {
        this.commandHandler.useListenerHandler(this.listenerHandler);
        
        // Set the listener handler emitters.
        this.logger.log('info', 'Setting up emitters.')
        this.listenerHandler.setEmitters({
            commandHandler: this.commandHandler,
            listenerHandler: this.listenerHandler
        })

        // Load all commands and listeners.
        this.logger.log('info', 'Loading handlers.')
        this.commandHandler.loadAll();
        this.listenerHandler.loadAll();
    }

    async start() {
        this.logger.log('info', `Starting up Erica v${version} and logging into the Discord API.`)

        if (process.env.pm_id) {
            this.logger.log('info', 'I am running in PM2 mode. I will remain running in the background')
            this.logger.log('info', 'and will restart/reload if a crash occurs.')
        } else {
            this.logger.log('warn', 'I am not running under PM2 :( I will not gracefully restart if a crash occurs!')
        }
        
        if (process.version.includes('nightly') || process.version.includes('canary')) {
            this.logger.log('warn', 'You are running Erica on an experimental version of Node. You may experience issues!')
        }
        
        return this.login(this.config.token);
    }
};

module.exports = EricaClient;
