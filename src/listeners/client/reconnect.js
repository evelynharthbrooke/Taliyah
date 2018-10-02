/**
 * reconnect.js -- The listener for the reconnect event, checks
 * if the bot has reconnected to the Discord API.
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

class ReconnectListener extends Listener {
    constructor() {
        super('reconnecting', {
            emitter: 'client',
            event: 'reconnecting',
            category: 'client'
        });
    }
    
    exec() {
        this.client.logger.log('info', 'Reconnected to the Discord API.')
    }
}

module.exports = ReconnectListener;