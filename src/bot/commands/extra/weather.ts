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

import { Message, MessageEmbed } from 'discord.js';

import { Command } from 'discord-akairo';

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
        },
      ],
    });
  }

  public async exec(message: Message, { location }: { location: string }) {
    const DARK_SKY_API_URL = 'https://api.darksky.net';
    return message.channel.send('Not implemented yet.');
  }
}
