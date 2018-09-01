const { AkairoClient, CommandHandler, ListenerHandler } = require('discord-akairo');
const { createLogger, transports, format } = require('winston');
const path = require('path');

class EricaClient extends AkairoClient {
    constructor(config) {
        super({ ownerID: config.owner }, {
            disableEveryone: true,
        });

        this.logger = createLogger({
            format: format.combine(
                format.colorize(),
                format.timestamp({ format: 'YYYY/MM/DD HH:mm:ss' }),
				format.printf(info => `[${info.timestamp}] ${info.level}: ${info.message}`)
            ),
            transports: [new transports.Console()]
        })

        // Initialize the command handler.
        this.commandHandler = new CommandHandler(this, {
            directory: path.join(__dirname, '..', 'commands'),
            prefix: config.prefix,
            aliasReplacement: /-/g,
            commandUtil: true,
            allowMention: true,
            handleEdits: true
        });

        // Initialize the listener handler.
        this.listenerHandler = new ListenerHandler(this, {
            directory: path.join(__dirname, '..', 'listeners')
        });

        this.config = config;

        this.setup();
    };

    setup() {
        this.commandHandler.useListenerHandler(this.listenerHandler);
        
        // Set the listener handler emitters.
        this.listenerHandler.setEmitters({
            commandHandler: this.commandHandler,
            listenerHandler: this.listenerHandler
        })

        // Load all commands and listeners.
        this.commandHandler.loadAll();
        this.listenerHandler.loadAll();
    }

    async start() {
        return this.login(this.config.token);
    }
};

module.exports = EricaClient;
