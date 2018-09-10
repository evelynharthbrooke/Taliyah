/**
 * disconnect.js -- The listener for the disconnect event, checks
 * if the bot has disconnected, and if it has, log the event.
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

class DisconnectListener extends Listener {
    constructor() {
        super('disconnect', {
            emitter: 'client',
            event: 'disconnect',
            category: 'client'
        });
    }
    
    async exec(event) {
        this.client.logger.log('warn', `Sorry, looks like I disconnected! :( (${event.code})`)
    }
}

module.exports = DisconnectListener;