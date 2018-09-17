/**
 * ready.js -- The listener for the ready event, checks to
 * make sure the bot is ready to accept commands and other
 * input, and also sets the bot's activity.
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
        this.client.logger.info(`Erica v${version} has successfully logged into Discord and is ready to handle command requests.`);
        this.client.user.setActivity(`Erica v${version} | ${config.prefix}help`);
    }
}

module.exports = ReadyListener;
