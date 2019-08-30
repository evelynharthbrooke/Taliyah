/**
 * eval.ts -- Allows the evaulation of JavaScript code. Not available
 * for use by non bot owners.
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

import { Command } from 'discord-akairo';
import { Message } from 'discord.js';

export default class EvalCommand extends Command {
  public constructor() {
    super('eval', {
      aliases: ['eval'],
      category: 'Utilities',
      ownerOnly: true,
      ratelimit: 2,
      args: [
        {
          id: 'code',
          match: 'content',
          type: 'string',
        },
      ],
    });
  }

  public async exec(message: Message) {
    return message.channel.send('Work in progress; not implemented yet.');
  }
}
