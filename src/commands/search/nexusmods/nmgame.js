/**
 * nmgame.js -- Retrieves information on a specified game supported by
 * the Nexus Mods modding site.
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
const config = require("../../../../config.json");
const version = require("../../../../package.json").version;

const request = require('node-superfetch');

const NEXUS_MODS_API_URL = "https://api.nexusmods.com/v1"
const NEXUS_MODS_API_KEY = config.nm_key;

class NMGameCommand extends Command {
    constructor() {
        super('nmgame', {
            aliases: ["nmgame"],
            category: 'Search',
            description: { content: 'Retrieves various information on a specified game on Nexus Mods.' },
            args: [
                {
                    id: "game",
                    match: "content",
                    prompt: {
                        start: "What's the game you would like to search for?"
                    }
                }
            ]
        })
    };

    async exec(message, { game }) {
        var headers = {
            'accept': 'application/json',
            'apikey': NEXUS_MODS_API_KEY,
            'user-agent': `Erica Discord Bot/${version} Node/${process.version.substr(0, 6)}`
        };

        let { body: NMRequest} = await request.get(`${NEXUS_MODS_API_URL}/games/${game.replace(/\s+/g, '')}.json`).set(headers);

        const nmGameEmbed = new MessageEmbed();
        nmGameEmbed.setTitle(`Nexus Mods Information on game ${NMRequest.name}`);
        nmGameEmbed.setURL(NMRequest.nexusmods_url);
        nmGameEmbed.setDescription(`${NMRequest.name} has an available forum on Nexus Mods. ` + 
                                   `[Click here](${NMRequest.forum_url}) to access it.`)
        nmGameEmbed.addField('❯ Name', NMRequest.name, true);
        nmGameEmbed.addField("❯ Domain Alias", "`" + NMRequest.domain_name + "`", true);
        nmGameEmbed.addField('❯ Genre', NMRequest.genre, true);
        nmGameEmbed.addField('❯ Authors', NMRequest.authors, true);
        nmGameEmbed.addField('❯ Mod & File Count', `${NMRequest.mods} (${NMRequest.file_count} files)`, true);
        nmGameEmbed.addField('❯ Downloads', NMRequest.downloads, true);
        nmGameEmbed.addField('❯ Categories', NMRequest.categories.length, true);
        nmGameEmbed.addField('❯ Views', NMRequest.file_views, true);
        nmGameEmbed.addField('❯ Endorsements', NMRequest.file_endorsements, true);
        nmGameEmbed.setFooter(`Game ID: ${NMRequest.id} | Information provided via the Nexus Mods API.`);

        return message.channel.send(nmGameEmbed);

    };
};

module.exports = NMGameCommand;
