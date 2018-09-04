/**
 * utilities.js -- Utility functions for Erica.
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
class Util {
    /**
     * Duration method
     * 
     * Used to get the duration in a human-readable format.
     * 
     * @param ms total time in milliseconds
     */
    static duration(ms) {
      const sec = Math.floor((ms / 1000) % 60).toString();
      const min = Math.floor((ms / (1000 * 60)) % 60).toString();
      const hrs = Math.floor(ms / (1000 * 60 * 60)).toString();
      return `${hrs.padStart(2, '0')}:${min.padStart(2, '0')}:${sec.padStart(2, '0')}`;
    }

    /**
     * Shorten method
     * 
     * Shortens text to be of a certain length.
     * 
     * @param text The text to shorten.
     * @param length The max length of the text.
     */
    static shorten(text, length = 2000) {
      return text.length > length ? `${text.substr(0, length - 3)}...` : text;
    }

    /**
     * Base64 method
     * 
     * Converts strings to base64.
     * 
     * @param text The speciifed text to work with.
     * @param mode The specified mode to work in.
     */
    static base64(text, mode = 'encode') {
      if (mode === 'encode') return Buffer.from(text).toString('base64');
      if (mode === 'decode') return Buffer.from(text, 'base64').toString('utf8') || null;
      throw new TypeError(`${mode} is not a supported base64 mode.`);
  }
};

module.exports = Util;
