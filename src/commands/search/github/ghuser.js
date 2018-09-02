/**
 * ghuser.js -- Retrieves information on a specified GitHub user.
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
const request = require('node-superfetch');
const { base64 } = require('../../../util/utilities');
const moment = require('moment');
const numeral = require('numeral');
const config = require('../../../../config.json');

class GitHubUserCommand extends Command {
    constructor() {
        super('ghuser', {
            aliases: ['guser', 'ghuser', 'githubuser'],
            category: 'search',
            description: { 
                content: 'Retrieve information on a GitHub user.',
                usage: '<user>'
            },
            args: [
                {
                    id: 'name',
                    match: 'content',
                    type: 'string',
                    prompt: {
                        start: message => `${message.author}, who would you like to search for?`
                    }
                }
            ]
        });
    }

    async exec(message, { name }) {
        const github_base = 'https://api.github.com'
        // use the basic authentication type as we don't need OAuth.
        const auth = `Basic ${base64(`${config.github_username}:${config.github_password}`)}`

        let {body : user} = await request.get(`${github_base}/users/${name}`).set({ Authorization: auth });

        function getBio() {
            if (user.bio != null) {
                return user.bio
            } else {
                return 'No biography set.';
            }
        }

        function getLocation() {
            if (user.location != null) {
                return user.location
            } else {
                return 'No location set.'
            }
        }

        const repoCount = numeral(user.public_repos).format('0a');
        const followingCount = numeral(user.following).format('0a');
        const followerCount = numeral(user.followers).format('0a');
        const creationDate = moment.utc(user.created_at).format('LL')

        let GhUserEmbed = new MessageEmbed()
            .setColor(0x4078c0)
            .setThumbnail(user.avatar_url)
            .setURL(user.html_url)
            .setTitle(user.name)
            .setDescription(getBio())
            .addField('❯ Location', getLocation(), true)
            .addField('❯ Repositories', repoCount, true)
            .addField('❯ Username', user.login, true)
            .addField('❯ Created On:', creationDate, true)
            .addField('❯ Following', followingCount, true)
            .addField('❯ Followers', followerCount, true)

        if (user.type === "Organization") {
            await message.util.send(`I cannot search for organizations, please use ${config.prefix}ghorg.`)
        } else {
            return message.util.send(GhUserEmbed)
        }
    }
}

module.exports = GitHubUserCommand;
