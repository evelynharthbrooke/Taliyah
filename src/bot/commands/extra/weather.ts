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
import { Util } from '../../utils/Util';
import moment from 'moment';
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

    if (!location) {
      return message.channel.send('You didn\'t enter a location name! Please enter one and ' +
        'then try again.');
    }

    const locationCoordinates = await Util.getCoordinates(location);
    const darkSkyApiUrl = 'https://api.darksky.net/forecast';
    const darkSkyApiKey = this.client.config.darksky.key;
    const { body: weather } = await request.get(
      `${darkSkyApiUrl}/${darkSkyApiKey}/` +
      `${locationCoordinates.lat},${locationCoordinates.long}?${stringify({
        units: 'si',
      })}`,
    );

    /** Initialize the weather embed. */
    const darkSkyEmbed = new MessageEmbed();
    /** Weather Conditions */
    const darkSkySummary = weather.daily.summary;
    const darkSkyCondition = weather.daily.data[0].summary;
    const darkSkyWindSpeed = weather.currently.windSpeed;
    const darkSkyPresure = Math.round(weather.currently.pressure);
    const darkSkyDewPoint = weather.currently.dewPoint;
    const darkSkyHumidity = Math.round(weather.currently.humidity * 100);
    const darkSkySunrise = weather.daily.data[0].sunriseTime * 1000;
    const darkSkySunset = weather.daily.data[0].sunsetTime * 1000;
    const darkSkyTemp = Math.round(weather.currently.temperature);
    const darkSkyTempFahren = Math.round(Util.fahrenify(weather.currently.temperature));
    const darkSkyTodayHigh = Math.round(weather.daily.data[0].temperatureHigh);
    const darkSkyTodayHighFahren = Math.round(Util.fahrenify(darkSkyTodayHigh));
    const darkSkyTodayLow = Math.round(weather.daily.data[0].temperatureLow);
    const darkSkyTodayLowFahren = Math.round(Util.fahrenify(darkSkyTodayLow));

    darkSkyEmbed.setTitle(`Weather information for ${locationCoordinates.address}`);
    darkSkyEmbed.setColor(0x8cbed6);

    /** Set the embed description. Contains all the weather information. */
    darkSkyEmbed.setDescription(
      `${darkSkySummary}\n\n` +
      `**Condition**: ${darkSkyCondition}\n` +
      `**Currently**: ${darkSkyTemp} °C | ${darkSkyTempFahren} °F\n` +
      `**Today's High**: ${darkSkyTodayHigh} °C | ${darkSkyTodayHighFahren} °F\n` +
      `**Today's Low**: ${darkSkyTodayLow} °C | ${darkSkyTodayLowFahren} °F\n` +
      `**Wind Speed**: ${darkSkyWindSpeed} km/h\n` +
      `**Pressure**: ${darkSkyPresure} hPa\n` +
      `**Dew Point**: ${darkSkyDewPoint} °C\n` +
      `**Humidity**: ${darkSkyHumidity}%\n` +
      `**Sunrise**: ${moment(darkSkySunrise).format('h:mm a')}\n` +
      `**Sunset**: ${moment(darkSkySunset).format('h:mm a')}\n`,
    );

    /** Set the embed footer. */
    darkSkyEmbed.setFooter('Powered by the Dark Sky API.');

    /** Send the weather embed. */
    message.channel.send(darkSkyEmbed);

  }
}
