/**
 * wikipedia.js -- The Wikipedia command.
 * 
 * This command retrieves information about a Wikipedia article.
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
const { shorten } = require('../../../util/utilities');
const request = require('node-superfetch');

class WikipediaCommand extends Command {
    constructor() {
        super('wikipedia', {
            aliases: ['wikipedia', 'wiki'],
            category: 'Search',
            description: {
                content: 'Retrieves information about a given article from Wikipedia.',
                usage: '<article>'
            },
            args: [
                {
                    id: 'articleName',
                    type: 'string',
                    match: 'content'
                }
            ]
        });
    };

    async exec(message, { articleName }) {
        const wikipediaAPI = 'https://en.wikipedia.org/w/api.php';

        try {
            const { body : articleInfo } = await request.get(wikipediaAPI).query({
                action: 'query',
                prop: 'extracts|pageimages|info',
                inprop: 'url',
                format: 'json',
                titles: articleName,
                exintro: '',
                explaintext: '',
                pithumbsize: 150,
                redirects: '',
                formatversion: 2
            })

            const wikipediaLogo = 'https://i.imgur.com/Z7NJBK2.png'
            const article = articleInfo.query.pages[0];
            
            if (article.missing) {
                return message.util.send('Couldn\'t find any results! Try again with a different query.');
            }

            if (article.title.includes('Main Page') || articleName.includes('Main Page')) {
                message.util.send('Looks like you tried searching for the Main Page. Sorry, ' +
                                  'but I cannot bring up the main page. Try a different query.')
            } else if (articleName.includes('Special:')) {
                message.util.send("Sorry, searching for Special: pages is not allowed by the " +
                                  "Wikipedia API. Please try a different query.")
            } else {
                const embed = new MessageEmbed()
                    .setColor(0xFFFFFF)
                    .setTitle(article.title)
                    .setAuthor('Wikipedia', wikipediaLogo, 'https://www.wikipedia.org/')
                    .setThumbnail(article.thumbnail ? article.thumbnail.source : null)
                    .setURL(article.fullurl)
                    .setFooter(`Page content for ${article.title} is licensed under the CC-BY-SA 3.0 License.`)
                    .setDescription(shorten(article.extract.replace(/\n/g, '\n\n')));
                
                return message.util.send(embed)
            }
        } catch(err) {
            console.log(err)
            return message.util.send('Sorry, but an error has occurred! Please try again later!')
        }
    };
};

module.exports = WikipediaCommand;
