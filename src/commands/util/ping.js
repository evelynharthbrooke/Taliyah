/**
 * ping.js -- The ping command; pings the Discord API gateway
 * and checks message latency.
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

class PingCommand extends Command {
    constructor() {
        super('ping', {
            aliases: ['ping'],
            category: 'Utilities',
            description: { content: 'Checks Erica\'s response to the Discord API ' + 
                                    'and checks the message latency.'}
        });
    }

    async exec(message) {
        const m = await message.util.send(":ping_pong: Ping!");
        const latency = Math.round(m.createdTimestamp - message.createdTimestamp);

        // indicates that the bot's health is currently perfect
        // and is less than 250ms.
        const healthPerfect = 'perfect [within 250ms]'
        // indicates that the bot's health is okay and is within
        // 500ms.
        const healthOk = 'okay [within 500ms]'
        // not as bad, but still pretty bad, means that there's an
        // issue with some part of Discord but not to the extent of
        // completely having an outage.
        const healthSlow = 'on the slow side [within 501-999ms]'
        // indicates that health is bad and that there might be an
        // outage over on Discord's side of the pond.
        const healthBad = 'looking bad, possible API outage? [greater than 1000ms]'

        const bot_health = `Current bot health is ${latency < 250 ? healthPerfect : latency < 500 ? healthOk : latency > 1000 ? healthBad : healthSlow}` 

        const PingEmbed = new MessageEmbed()
            .setColor(0x8b0000)
            .setDescription("Pong! :ping_pong:")
            .addField('Message Latency', `\`${latency}ms\``, true)
            .addField('Heartbeat', `\`${Math.round(this.client.ws.ping)}ms\``, true)
            .setFooter(bot_health)
        
        return m.edit(PingEmbed);
    }
}

module.exports = PingCommand;
