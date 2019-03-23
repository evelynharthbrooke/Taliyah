/**
 * debug.ts — Listens for any debug events and logs them to the console.
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

import { Listener } from 'discord-akairo';

export default class DebugListener extends Listener {
  public constructor() {
    super('debug', {
      emitter: 'client',
      event: 'debug',
      category: 'client',
    });
  }

  public exec(event: any) {
    /** Log debug events. */
    this.client.logger.debug(event);
  }
}
