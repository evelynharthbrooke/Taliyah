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

        let {body : botCommits } = await request.get(commitsUrl).set({ Authorization: login });

        const commits = botCommits.slice(0, 10);
		const embed = new MessageEmbed()
			.setTitle(`[${config.erica_repo_name}:master] Most recent commits`)
			.setColor(0x7289DA)
			.setURL(`${commitsUrl}/master`)
			.setDescription(commits.map(commit => {
				const sha = `[\`${commit.sha.slice(0, 7)}\`](${commit.html_url})`;
				return `${sha} ${shorten(commit.commit.message.split('\n')[0], 50)} - ${commit.author.login}`;
			}).join('\n'));
		return message.channel.send(embed);
    }
}

module.exports = ChangelogCommand;