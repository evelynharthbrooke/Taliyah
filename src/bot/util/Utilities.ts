/**
 * Utils.ts -- Various utility functions that ease the development of
 * Ellie.
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

import { client } from '../../ellie';
import { stringify } from 'querystring';

export class Utilities {

  /**
   * The Shorten utility function.
   *
   * This is a really basic function that allows the shortening of content
   * to be of a certain length. Useful for situations where content would
   * end up reaching a character limit, such as Discord's character limit
   * for message embeds.
   *
   * @param {string} content The content to shorten.
   * @param {number} limit The amount of characters to limit the content to.
   * @returns {string} The shortened content.
   */
  public static shorten(content: string, limit: number): string {
    return content.length > limit ? `${content.substr(0, limit - 3)}...` : content;
  }

  /**
   * The getCoordinates function.
   *
   * Gets coordinates for a chosen location.
   *
   * @param location The location to get coordinates for.
   */
  public static async getCoordinates(location: string) {
    const gmapsGeocodeUrl = 'https://maps.googleapis.com/maps/api/geocode/json';

    const gmapsRequest = await request.get(`${gmapsGeocodeUrl}?${stringify({
      address: location,
      key: client.config.google!,
    })}`);

    const coordinates = await gmapsRequest.body;

    return {
      address: coordinates.results[0].formatted_address,
      lat: coordinates.results[0].geometry.location.lat,
      long: coordinates.results[0].geometry.location.lng,
    };
  }

  /**
   * The format utility function.
   *
   * Similar to Python's implementation, allowing users to format
   * their strings with %s.
   *
   * @param string The piped string.
   * @param args The arguments to send.
   * @returns {string} The formatted String.
   */
  public static format(string: string, ...args: string[]): string {
    return args.reduce((str, val) => str.replace(/%s|%v|%d|%f|%d/, val), string);
  }

  /**
   * The convertToTitleCase utility function.
   *
   * Takes a string and converts it to be of Title Case.
   *
   * @param {string} string The string to convert to Title Case.
   * @returns {string} The string in Title Case.
   */
  public static convertToTitleCase(string: string): string {
    const wordSeparators = /([ :–—-])/;
    const str = string.toLowerCase().split(wordSeparators);

    for (let i = 0; i < str.length; i++) {
      str[i] = str[i].charAt(0).toUpperCase() + str[i].slice(1);
    }

    return str.join(' ');
  }

}
