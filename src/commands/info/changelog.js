/**
 * changelog.js -- Fetches the most recent commits to Erica.
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
const { shorten, base64 } = require('../../util/utilities');
const config = require('../../../config.json');
const request = require('node-superfetch');

class ChangelogCommand extends Command {
    constructor() {
        super('changelog', {
            aliases: ['changelog', 'updates', 'commits'],
            category: 'Information',
            description: {
                content: 'Replies with the bot\'s most recent commits.'
            },
            ratelimit: 2
        })
    }

    async exec(message) {
        const github_base = 'https://api.github.com'
        const commitsUrl = github_base + `/repos/${config.erica_repo_username}/${config.erica_repo_name}/commits`
        const login = `Basic ${base64(`${config.github_username}:${config.github_password}`)}`

        const githubCommitsURL = `https://github.com/${config.erica_repo_username}/${config.erica_repo_name}/commits/master`

        let {body : botCommits } = await request.get(commitsUrl).set({ Authorization: login });

        const commits = botCommits.slice(0, 10);
		const embed = new MessageEmbed()
			.setTitle(`Most recent commits on ${config.erica_repo_name}'s master branch`)
			.setColor(0x7289DA)
			.setURL(githubCommitsURL)
			.setDescription(commits.map(commit => {
				const sha = `[\`${commit.sha.slice(0, 7)}\`](${commit.html_url})`;
				return `${sha} ${shorten(commit.commit.message.split('\n')[0], 50)} - ${commit.author.login}`;
			}).join('\n'));
		return message.channel.send(embed);
    }
}

module.exports = ChangelogCommand;
