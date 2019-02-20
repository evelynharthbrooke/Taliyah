/**
 * report.js -- The report command.
 * 
 * Allows users to report other users.
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
const ReportSchema = require("../../assets/models/report");
const mongoose = require('mongoose');

class ReportCommand extends Command {
    constructor() {
        super('report', {
            aliases: ['report'],
            category: 'Moderation',
            clientPermissions: ['READ_MESSAGE_HISTORY', 'MANAGE_MESSAGES'],
            userPermissions: ['MANAGE_MESSAGES'],
            description: {
                content: "Allows you to report a user.",
                usage: "<user>"
            },
            args: [
                {
                    id: 'member',
                    type: "member",
                    prompt: {
                        start: "Please enter the member you'd like to report.",
                        retry: "You did not enter a user to report."
                    }
                },
                {
                    id: 'reason',
                    match: "rest",
                    prompt: {
                        start: "Why are you reporting this user?",
                        retry: "You did not enter a reason!"
                    }
                }
            ]
        })
    }

    async exec(message, { member, reason }) {

        mongoose.connect("mongodb://localhost/discord")

        const report = new ReportSchema({
            _id: mongoose.Types.ObjectId(),
            username: member.user.username,
            userID: member.id,
            reason: reason,
            guild: message.guild.name,
            channel: message.channel.name,
            channelID: message.channel.id,
            rUser: message.author.username,
            rUserID: message.author.id,
            time: message.createdAt
        });

        const reportEmbed = new MessageEmbed();
        reportEmbed.setTitle(`User ${report.username} reported.`)
        reportEmbed.setThumbnail(member.user.displayAvatarURL())
        reportEmbed.setDescription(`User reported for "${report.reason}".`)
        reportEmbed.addField("Reporter Username", report.rUser, true)
        reportEmbed.addField("Reporter ID", report.rUserID, true)
        reportEmbed.addField("Guild", report.guild, true)
        reportEmbed.addField("Channel", report.channel, true)
        reportEmbed.setFooter(`Report sent at ${report.time}. Channel ID: ${report.channelID}`)

        report.save()
            .then(message.channel.send(reportEmbed))
            .catch(err => console.log(err));
    }
}

module.exports = ReportCommand;