/**
 * weather.ts -- Gets the forecast for a specified location.
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
import { Utilities } from '../../util/Utilities';
import { stringify } from 'querystring';

export default class WeatherCommand extends Command {
  public constructor() {
    super('weather', {
      aliases: ['weather', 'forecast'],
      category: 'Extra',
      description: {
        content: 'Displays the forecast for a specified location.',
        usage: '<location>',
      },
      args: [
        {
          id: 'location',
          type: 'string',
          match: 'rest',
        },
      ],
    });
  }

  public async exec(message: Message, { location }: { location: string }) {
    const coordinates = await Utilities.getCoordinates(location);
    const darkSkyApiUrl = 'https://api.darksky.net/forecast';
    const darkSkyApiKey = this.client.config.darksky.key;
    const darkSkyRequest = await request.get(
      `${darkSkyApiUrl}/${darkSkyApiKey}/${coordinates.lat},${coordinates.long}?${stringify({
        units: 'si',
      })}`,
    );

    const darkSkyEmbed = new MessageEmbed();
    const darkSkySummary = darkSkyRequest.body.daily.summary;
    const darkSkyWindSpeed = darkSkyRequest.body.currently.windSpeed;
    const darkSkyCondition = darkSkyRequest.body.daily.data[0].summary;

    darkSkyEmbed.setTitle(`Weather information for ${coordinates.address}`);
    darkSkyEmbed.setDescription(
      `${darkSkySummary}\n\n` +
      `**Wind Speed**: ${darkSkyWindSpeed} km/h\n` +
      `**Condition**: ${darkSkyCondition}`);
    darkSkyEmbed.setFooter('Powered by the Dark Sky API.');

    console.log(darkSkyRequest.body.daily);

    message.channel.send(darkSkyEmbed);

  }
}
