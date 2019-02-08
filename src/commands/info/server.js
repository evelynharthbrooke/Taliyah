/**
 * server.js -- Retrieves information about the current server.
 * 
 * Copyright (c) 2019-present Kamran Mackey.
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

const { Command } = require("discord-akairo");
const { MessageEmbed } = require("discord.js");
const moment = require('moment');

// region listing for all Discord server regions.
// Makes the names easier to digest.
const regionNames = {
    "brazil": "Brazil",
    "eu-central": "Central Europe",
    "hongkong": "Hong Kong",
    "japan": "Japan",
    "russia": "Russia",
    "singapore": "Singapore",
    "southafrica": "South Africa",
    "sydney": "Sydney",
    "us-central": "US Central",
    "us-east": "US East",
    "us-south": "US South",
    "us-west": "US West",
    "eu-west": "Southern Europe"
}

class ServerCommand extends Command {
    constructor() {
        super('server', {
            aliases: ['server', 'server-info', 'guild-info', 'guild'],
            category: 'Information',
            description: {
                content: 'Retrieves detailed information on the current server, if available.',
                usage: '<blank>'
            },
        })
    }

    async exec(message) {

        const discordCreationDate = moment.utc(message.guild.createdAt).format('lll');
        const voiceChannels = message.guild.channels.filter(channel => channel.type == "voice").size
        const textChannels = message.guild.channels.filter(channel => channel.type == "text").size
        const regionName = message.guild.region ? `${regionNames[message.guild.region]}` : message.guild.region;

        // Check if the current guild is up. If it's up, return an embed
        // with detailed guild information. If it's down, return a message
        // saying that the guild isn't available.
        if (message.guild.available) {
            const embed = new MessageEmbed()

            embed.setTitle(`Information for server ${message.guild.name}`)
            embed.setThumbnail(message.guild.iconURL())
            embed.addField("❯ Name", message.guild.name, true)
            embed.addField("❯ Owner", message.guild.owner.user.tag, true)
            embed.addField("❯ Members", message.guild.memberCount, true)

            if (message.guild.verified == false) {
                embed.addField("❯ Verified?", "No", true)
            } else {
                embed.addField("❯ Verified?", "Yes", true)
            }

            embed.addField("❯ Region", regionName, true)
            embed.addField("❯ Creation Date", discordCreationDate, true)
            embed.addField("❯ Emoji Count", message.guild.emojis.size, true)
            embed.addField("❯ Presences", message.guild.presences.size, true)
            embed.addField("❯ Text Channels", textChannels, true)
            embed.addField("❯ Voice Channels", voiceChannels, true)
            embed.addField("❯ Roles", message.guild.roles.size, true)
            embed.setFooter(`The ID of the guild ${message.guild.name} is ${message.guild.id}.`)

            return message.channel.send(embed);
        } else {
            return message.channel.send("This guild isn't available. Sorry! Try again later.")
        }

    }
}

module.exports = ServerCommand;
