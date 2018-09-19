/**
 * about.js -- The about command; sends the user useful information
 * about the Erica bot for Discord.
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
const { Command } = require('discord-akairo');
const { MessageEmbed } = require('discord.js');
const humanize = require('humanize-duration')
const { base64 } = require('../../util/utilities');
const request = require('node-superfetch')
const pluralize = require('pluralize');
const config = require("../../../config.json")
const { version } = require('../../../package.json');

class AboutCommand extends Command {
    constructor() {
        super('about', {
            aliases: ['about', 'info'],
            category: 'Utilities',
            description: { content: 'Retrieves various information and statistics about Erica.'},
        })
    }

    async exec(message) {
        // Set up a request to the GitHub API. This allows us to properly retrieve
        // total commits, as well as the latest commit. This uses GitHub authentication
        // to lower the risk of being rate-limited. The authentication details are sent
        // using base64 encoding instead of plain text for improved security so that way
        // the details aren't able to be intercepted, even though HTTPS does a good job
        // of preventing that anyways.
        const { body: commits } = await request
            .get('https://api.github.com/repos/KamranMackey/Erica/commits')
            .set({
                Authorization: `Basic ${base64(`${config.github_username}:${config.github_password}`)}`
            });
        
        function getNodeVersion() {
            if (process.version.includes('nightly')) {
                return process.version.substr(0, 7).concat(' ' + '(nightly)'); // for nightly builds
            } else if (process.version.includes('canary')) {
                return process.version.substr(0, 7).concat(' ' + '(canary)'); // for canary builds
            } else if (process.version.includes('rc')) {
                return process.version.substr(0, 7).concat(' ' + '(rc)'); // for rc builds
            } else {
                return process.version.substr(0, 7).concat(' ' + '(stable)'); // for stable builds
            }
        }

        // Filter the channel count to remove channel categories, as they aren't
        // neccessary while retrieving the number of channels.
        const channelCount = this.client.channels.filter(channel => channel.type != "category").size

        const info = new MessageEmbed()
        .setColor(0x00AE86)
        .setTitle(`About ${this.client.user.username}`)
        .setThumbnail(this.client.user.displayAvatarURL({ format: 'png', size: 1024 }))
        .setDescription(`Information about ${this.client.user.username}, such as the latest commit, its uptime, etc. You ` +
                        `can visit her source code on GitHub [here](https://github.com/KamranMackey/Erica).`)
        .addField("❯ Latest Commit", `[\`${commits[0].sha.substr(0, 7)}\`](${commits[0].html_url})`, true)
        .addField('❯ Uptime', humanize(this.client.uptime, { largest: 1, round: true }), true)
        .addField('❯ Servers', `${this.client.guilds.size} server${this.client.guilds.size > 1 ? 's' : ''}`, true)
        .addField('❯ Users', pluralize('user', this.client.guilds.map(g => g.memberCount).reduce((f, l) => f + l), true), true)
        .addField('❯ Channels', pluralize('channel', channelCount, true), true)
        .addField('❯ Version', version, true)
        .addField('❯ Node Version', getNodeVersion(), true)
        .addField('❯ V8 Version', process.versions.v8.substr(0, 15), true)
        .addField('❯ Memory Usage', `${Math.round(process.memoryUsage().heapUsed / 1024 / 1024)} MB`, true)

        return message.util.send(info);
    }
}

module.exports = AboutCommand;
