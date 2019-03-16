/**
 * ellie.ts -- The main file. Responsible for importing the Bot Client,
 * initializing the config file, and loading the bot itself.
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

import Config from './bot/client/Config';
import EllieClient from './bot/client/EllieClient';

/** Initialize the config file. */
const config = Config.initConfigFromFile();

/**
 * Create a new instance of EllieClient with
 * the configuration.
 */
const client = new EllieClient(config, {
  ownerID: config.owner,
  disableEveryone: true,
  disabledEvents: ['TYPING_START'],
});

/** Finally, start Ellie. */
client.loginToDiscord(config.token);
