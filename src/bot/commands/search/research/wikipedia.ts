/**
 * wikipedia.ts -- Retrieves information about a specified Wikipedia
 * article.
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

import * as request from 'superagent';

import { Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';
import { Util } from '../../../util/Utilities';

export default class WikipediaCommand extends Command {
  public constructor() {
    super('wikipedia', {
      aliases: ['wikipedia', 'w', 'wiki'],
      category: 'Search',
      description: {
        content: 'Allows you to a lookup a Wikipedia article.',
        usage: '<lang> <query>',
      },
      args: [
        {
          id: 'lang',
        },
        {
          id: 'query',
          match: 'rest',
        },
      ],
    });
  }

  public async exec(message: Message, { lang, query }: { lang: string; query: string }) {
    if (!lang || !query) {
      return message.channel.send('You either didn\t enter a language shortcode, didn\t' +
        'enter an article name to search for, or both. Please try again.',
      );
    }

    try {
      const W_URL = `https://${lang}.wikipedia.org/`;
      const W_API = W_URL + '/w/api.php';
      const W_LOGO = 'https://i.imgur.com/Z7NJBK2.png';
      const W_REQUEST = await request.get(W_API).query({
        action: 'query',
        prop: 'extracts|pageimages|info',
        inprop: 'url',
        format: 'json',
        titles: query,
        exintro: '',
        explaintext: '',
        pithumbsize: 150,
        redirects: '',
        formatversion: 2,
      });

      const W_ARTICLE = W_REQUEST.body.query.pages[0];

      if (W_ARTICLE.missing) {
        return message.channel.send(`The article with the name of ${query} could not be found. ` +
          'Please try a different article name.',
        );
      }

      function restrictedPages() {
        return W_ARTICLE.title.includes('Main Page')
          || W_ARTICLE.title.includes('Special:')
          || query.includes('Main Page')
          || query.includes('Special:');
      }

      if (restrictedPages()) {
        return message.channel.send('I am unable to display the Main Page or pages labeled under ' +
          '`Special:`. Please try a different article/page.');
      }

      const ARTICLE_NAME = W_ARTICLE.title;
      const ARTICLE_DESC = W_ARTICLE.extract.replace(/\n/g, '\n\n');
      const ARTICLE_URL = W_ARTICLE.fullurl;

      const W_EMBED = new MessageEmbed();
      W_EMBED.setTitle(ARTICLE_NAME);
      W_EMBED.setURL(ARTICLE_URL);
      W_EMBED.setAuthor('Wikipedia', W_LOGO, W_URL);
      W_EMBED.setColor(0xc7c8ca);
      W_EMBED.setDescription(Util.shorten(ARTICLE_DESC, 1985));
      W_EMBED.setFooter(`Page content for ${ARTICLE_NAME} is licensed under CC-BY-SA 3.0.`);

      message.channel.send(W_EMBED);
    } catch (err) {
      console.log(err);
    }
  }
}
