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
const { version } = require('../../../package.json');
const activities = require('../../assets/json/activities.json');

class ReadyListener extends Listener {
    constructor() {
        super('ready', {
            emitter: 'client',
            event: 'ready',
            category: 'client'
        });
    }

    async exec() {
        this.client.logger.log('info', `Erica v${version} has successfully logged into Discord and is ready to handle command requests.`);

        // set the base activity that way so an activity has been set before
        // our activity rotation executes.
        const baseActivity = activities[Math.floor(Math.random() * activities.length)];
        this.client.logger.log('info', 'Setting the base activity.')
        this.client.user.setActivity(baseActivity.text, { type: baseActivity.type })

        this.client.setInterval(() => {
            this.client.logger.log('info', `Rotating between activities.`)
            const activity = activities[Math.floor(Math.random() * activities.length)];
            this.client.user.setActivity(activity.text, { type: activity.type })
            this.client.logger.log('info', `Changed the activity! New activity is "${activity.type} ${activity.text}".`)
        }, 120000)
    }
}

module.exports = ReadyListener;
