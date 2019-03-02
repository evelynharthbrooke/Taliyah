/**
 * user.js -- Retrieves information about a user.
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
const { trimArray } = require('../../util/utilities');
const moment = require('moment');

const activityChoices = {
    PLAYING: 'Playing',
    STREAMING: 'Streaming',
    WATCHING: 'Watching',
    LISTENING: 'Listening to'
};

class UserCommand extends Command {
    constructor() {
        super('user', {
            aliases: ['user', 'member', 'user-info', 'member-info'],
            category: 'Information',
            description: {
                content: 'Retrieves detailed information on a user, if available.',
                usage: '<blank> or <user>'
            },
            args: [
                {
                    id: 'user',
                    match: 'content',
                    type: 'user'
                }
            ]
        })
    }

    async exec(message, { user }) {
        if (!user) user = message.author;

        const discordCreationDate = moment.utc(user.createdAt).format('lll');

        const userEmbed = new MessageEmbed()
            .setThumbnail(user.displayAvatarURL())
            .addField('❯ ID', user.id, true)
            .addField('❯ Creation Date', discordCreationDate, true)
            .addField('❯ Account Type', user.bot ? 'Bot' : 'Human', true);

        if (message.channel.type === 'text') {
            const member = await message.guild.members.fetch(user.id);
            const serverJoinDate = moment.utc(member.joinedAt).format('lll');
            const roles = member.roles.filter(r => r.id !== message.guild.defaultRole.id)
                .sort((x, y) => x.position - y.position)
                .map(r => r.name);

            if (user.id === message.author.id) {
                userEmbed.setTitle(`Information on you, ${user.tag}.`);
            } else if (user.id === this.client.user.id) {
                userEmbed.setTitle('Information on me!');
            } else {
                userEmbed.setTitle(`Information on user ${user.tag}.`);
            }

            userEmbed.setColor(member.displayHexColor);
            userEmbed.setDescription(member.presence.activity
                ? `${activityChoices[member.presence.activity.type]} **${member.presence.activity.name}**.`
                : '');
            userEmbed.addField('❯ Server Join Date', serverJoinDate, true);
            userEmbed.addField('❯ Nickname', member.nickname || 'No nickname set.', true);
            userEmbed.addField('❯ Hoist Role', member.roles.hoist ? member.roles.hoist.name : 'None', true)
            userEmbed.addField(`❯ Roles (${roles.length})`, roles.length ? trimArray(roles, 1).join(', ')
                : 'No roles found.', true);
        }

        if (!message.guild) {
            userEmbed.setFooter("Cannot retrieve server-only user info, displaying basic user info instead.");
        }

        return message.channel.send(userEmbed);
    }
}

module.exports = UserCommand;
