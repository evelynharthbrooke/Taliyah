/**
 * ready.ts -- The listener for the ready event, checks to
 * make sure the bot is ready to accept commands and other
 * input, and also sets the bot's activity.
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
import { version } from '../../../../package.json';

const activities = require('../../assets/activities.json');

export default class ReadyListener extends Listener {
  public constructor() {
    super('ready', {
      emitter: 'client',
      event: 'ready',
      category: 'client',
    });
  }

  public async exec() {

    this.client.logger.info(
      `Ellie v${version} has successfully logged into the Discord API as ${this.client.user!.tag} and `
      + `is connected to ${this.client.guilds.size} guild(s).`,
    );

    const mainActivity = activities[Math.floor(Math.random() * activities.length)];

    this.client.user!.setActivity(mainActivity.name, { type: mainActivity.type });
    this.client.logger.info(`Setting initial activity to ${mainActivity.type} ${mainActivity.name}.`);

    this.client.setInterval(() => {
      this.client.logger.info('Attempting to rotate between activities.');

      const activity = activities[Math.floor(Math.random() * activities.length)];

      if (activity.name === this.client.user!.presence.activity!.name) {
        this.client.logger.info('Activity is identical, leaving activity the same for now.');
      } else {
        this.client.user!.setActivity(activity.name, { type: activity.type });

        this.client.logger.info(`Successfully changed the activity! New activity is "${activity.type} ${activity.name}".`);
      }

    }, 200000); // tslint:disable-line: align

  }
}
