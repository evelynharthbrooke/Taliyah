/**
 * pokemon.js -- Retrieves information on a specified Pokémon.
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

 const { Command } = require('discord-akairo');
 const { MessageEmbed } = require('discord.js');
 const Pokedex = require(`pokedex-promise-v2`)

 class PokemonLookupCommand extends Command {
    constructor() {
        super('pokemon', {
            aliases: ["pokemon", "plookup", "pmon", "poke"],
            category: 'Search',
            description: { 
                content: 'Retrieves information on the specified Pokémon.',
                usage: "<pokemon>"
            },
            args: [
                {
                    id: "pdex_entry",
                    match: "content",
                    prompt: {
                        start: "Which Pokémon would you like to get information on?"
                    }
                }
            ]
        })
    };

    async exec(message, { pdex_entry }) {
        const endpoint = new Pokedex();
        const pdex_lc = pdex_entry.toLowerCase();
        const poke_ep = `api/v2/pokemon/${pdex_lc}`;
        const poke_species_ep = `api/v2/pokemon-species/${pdex_lc}`;
        
        endpoint.resource([poke_ep, poke_species_ep]).then(res => {
            const pokeEmbed = new MessageEmbed();
            const name_lc = res[0].name
            const name = name_lc.replace(/^\w/, c => c.toUpperCase());
            const height = "__**Height**__: " + res[0].height / 10 + "m";
            const weight = "__**Weight**__: " + res[0].weight / 10 + "kg";
            const poke_url = `https://www.pokemon.com/us/pokemon/${name}`;

            let flavortext = "placeholder flavor text";
            
            if (res[1].flavor_text_entries[1].language.name == "en") {
                flavortext = res[1].flavor_text_entries[1].flavor_text;
            } else {
                flavortext = res[1].flavor_text_entries[2].flavor_text;
            }

            let pokeid = JSON.stringify(res[1].id);
            
            if (pokeid.length === 2) {
                pokeid = `0${res[1].id}`;
            } else if (pokeid.length === 1) {
                pokeid = `00${res[1].id}`;
            } else {
                pokeid = res[1].id;
            };

            const poke_thumb = `https://assets.pokemon.com/assets/cms2/img/pokedex/full/${pokeid}.png`

            let type = "placeholder type";

            if (typeof res[0].types[1] != 'undefined') {
                type = `__**Types**__: ${res[0].types[1].type.name.replace(/^\w/, c => c.toUpperCase())} ` +
                        `and ${res[0].types[0].type.name.replace(/^\w/, c => c.toUpperCase())}`;
            } else {
                type = `__**Type**__: ${res[0].types[0].type.name.replace(/^\w/, c => c.toUpperCase())}`;
            };

            pokeEmbed.setTitle(`Pokédex Information on ${name}`)
            pokeEmbed.setDescription(`${type}\n${height}\n${weight}\n\n${flavortext}`);
            pokeEmbed.setURL(poke_url);
            pokeEmbed.setThumbnail(poke_thumb)
            pokeEmbed.setFooter(`Pokédex entry #${pokeid} | Information provided by PokéAPI.`);

            return message.channel.send(pokeEmbed)
        }).catch(err => {
            console.log(err);
            return message.channel.send("Sorry! Either you entered an incorrect Pokémon name, or " + 
                                        "something else happened. Maybe try again?");
        })
    }
 }

 module.exports = PokemonLookupCommand;
