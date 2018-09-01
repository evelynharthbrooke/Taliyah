const { Command } = require('discord-akairo');
const { MessageEmbed } = require('discord.js');

class PingCommand extends Command {
    constructor() {
        super('ping', {
            aliases: ['ping'],
            category: 'util',
            description: { content: 'Pings Erica.'}
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
            .addField('Heartbeat', `\`${Math.round(this.client.ping)}ms\``, true)
            .setFooter(bot_health)
        
        return m.edit(PingEmbed);
    }
}

module.exports = PingCommand;
