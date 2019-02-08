/**
 * ghrepo.js -- Retrieves information on a specified GitHub repository.
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
const request = require('node-superfetch');
const moment = require('moment');
// Project level imports
const { shorten, base64 } = require("../../../util/utilities");
const config = require("../../../../config.json");

class GitHubRepoCommand extends Command {
    constructor() {
        super('ghrepo', {
            aliases: ['ghrepo', 'grepo', 'githubrepo'],
            category: 'Search',
            description: {
                content: 'Searches for a repository on GitHub.',
                usage: '<repository>'
            },
            args: [
                {
                    id: 'owner',
                    type: 'string',
                    prompt: {
                        start: `Who is the owner of the repository?`
                    }
                },
                {
                    id: 'repository',
                    type: 'string',
                    match: 'rest',
                    prompt: {
                        start: `What is the name of the repository?`
                    }
                }
            ]
        })
    }

    async exec(message, { owner, repository }) {
        const github_base = 'https://api.github.com'
        const repoUrl = github_base + `/repos/${owner}/${repository}`
        const login = `Basic ${base64(`${config.github_username}:${config.github_password}`)}`

        let { body: repo } = await request.get(repoUrl).set({ Authorization: login });

        const creationDate = moment.utc(repo.created_at).format('lll');
        const updatedDate = moment.utc(repo.updated_at).format('lll');

        let RepoEmbed = new MessageEmbed()
            .setColor(0xFFFFFF)
            .setURL(repo.html_url)
            .setThumbnail(repo.owner.avatar_url)
            .setTitle(`GitHub Repository Information for ${repo.full_name}`)
            .setDescription(repo.description ? shorten(repo.description) : `No description.`)
            .addField('❯ Owner', repo.owner.login, true)
            .addField('❯ Stars', repo.stargazers_count, true)
            .addField('❯ Forks', repo.forks, true)
            .addField('❯ Open Issues', repo.open_issues, true)
            .addField('❯ Language', repo.language || '???', true)
            .addField('❯ License', repo.license ? shorten(repo.license.spdx_id) : '???', true)
            .addField('❯ Created On', creationDate, true)
            .addField('❯ Last Updated', updatedDate, true)

        message.util.send(RepoEmbed)
    }
}

module.exports = GitHubRepoCommand;
