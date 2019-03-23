/**
 * pokemon.ts -- Retrieves information on the specified Pokémon.
 *
 * Copyright (c) 2019-present Kamran Mackey.
 *
 * Ellie is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Ellie is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ellie. If not, see <https://www.gnu.org/licenses/>.
 */

import { Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';
import Pokedex from 'pokedex-promise-v2';
import { Util } from '../../../utils/Util';

export default class PokemonCommand extends Command {
  public constructor() {
    super('pokemon', {
      aliases: ['pokemon', 'poke', 'pokedex'],
      category: 'Search',
      description: {
        content: 'Retrieves information on the specified Pokémon.',
        usage: '<pokemon>',
      },
      args: [
        {
          id: 'pokemon',
          match: 'content',
        },
      ],
    });
  }

  public async exec(message: Message, { pokemon }: { pokemon: string }) {
    const POKE_ENDPOINT = new Pokedex();
    const POKEMON_LOWERCASE = pokemon.toLowerCase();
    const POKEMON_MAIN_API = `api/v2/pokemon/${POKEMON_LOWERCASE}`;
    const POKEMON_SPECIES_API = `api/v2/pokemon-species/${POKEMON_LOWERCASE}`;

    POKE_ENDPOINT.resource([POKEMON_MAIN_API, POKEMON_SPECIES_API]).then((res) => {
      const POKEMON_EMBED = new MessageEmbed();
      const POKEMON_NAME = res[0].name.replace(/^\w/, (c: string) => c.toUpperCase());
      const BULBAPEDIA_URL = `https://bulbapedia.bulbagarden.net/wiki/${POKEMON_NAME}`;
      const POKEMON_HEIGHT = '**Height**: ' + res[0].height / 10 + 'm';
      const POKEMON_WEIGHT = '**Weight**: ' + res[0].weight / 10 + 'kg';
      // This is not the best looking code in existence, but it gets the job done nicely.
      const POKEMON_ABILITIES = '**Abilities**: ' + res[0].abilities.map((ability: any) => {
        return Util.convertToTitleCase(ability.ability.name).replace('-', ' ');
      }).join(', ');

      let POKEMON_FLAVOR_TEXT: string;
      let POKEMON_TYPE: string;
      let POKEMON_ID = JSON.stringify(res[1].id);

      if (res[1].flavor_text_entries[1].language.name === 'en') {
        POKEMON_FLAVOR_TEXT = res[1].flavor_text_entries[1].flavor_text;
      } else {
        POKEMON_FLAVOR_TEXT = res[1].flavor_text_entries[2].flavor_text;
      }

      if (POKEMON_ID.length === 2) {
        POKEMON_ID = `0${res[1].id}`;
      } else if (POKEMON_ID.length === 1) {
        POKEMON_ID = `00${res[1].id}`;
      } else {
        POKEMON_ID = res[1].id;
      }

      const POKEMON_THUMBNAIL = `https://assets.pokemon.com/assets/cms2/img/pokedex/full/${POKEMON_ID}.png`;

      if (typeof res[0].types[1] !== 'undefined') {
        POKEMON_TYPE = `**Types**: ${res[0].types[1].type.name.replace(/^\w/, (c: string) => c.toUpperCase())} ` +
          `and ${res[0].types[0].type.name.replace(/^\w/, (c: string) => c.toUpperCase())}`;
      } else {
        POKEMON_TYPE = `**Type**: ${res[0].types[0].type.name.replace(/^\w/, (c: string) => c.toUpperCase())}`;
      }

      POKEMON_EMBED.setTitle(POKEMON_NAME);
      POKEMON_EMBED.setColor(0xFFCB05);
      POKEMON_EMBED.setThumbnail(POKEMON_THUMBNAIL);
      POKEMON_EMBED.setDescription(
        `${POKEMON_TYPE}\n`
        + `${POKEMON_HEIGHT}\n`
        + `${POKEMON_WEIGHT}\n`
        + `${POKEMON_ABILITIES}\n\n`
        + `${POKEMON_FLAVOR_TEXT}\n\n`
        + `More information is available on [Bulbapedia](${BULBAPEDIA_URL}).`,
      );
      POKEMON_EMBED.setFooter(`Pokédex entry ${POKEMON_ID} | Powered by PokéAPI.`);

      message.channel.send(POKEMON_EMBED);

    }).catch((err) => {
      console.log(err);
    });
  }
}
