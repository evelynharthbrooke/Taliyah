/**
 * EllieClient.ts -- The Ellie client. Extends from AkairoClient.
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

import * as path from 'path';

import { AkairoClient, AkairoOptions, CommandHandler, ListenerHandler } from 'discord-akairo';

import { ClientOptions } from 'discord.js';
import Config from './Config';
import { Sequelize } from 'sequelize';
import { Signale } from 'signale';
import Spotify from 'spotify-web-api-node';
import { version } from '../../../package.json';

declare module 'discord-akairo' {
  interface AkairoClient {
    logger: Signale;
    database: Sequelize;
    config: Config;
    spotify: Spotify;
  }
}

export default class EllieClient extends AkairoClient {

  /** Initialize the logging system. */
  public logger = new Signale();

  /** Initialize the Configuration. */
  public config = new Config();

  public constructor(config: Config, options: AkairoOptions & ClientOptions) {
    super(options);

    /** Configure Signale. */
    this.logger.config({
      displayDate: true,
      displayTimestamp: true,
    });

    /** Bind this.config to the bot's configuration. */
    this.config = config;

    /** Setup the bot's database. */
    this.database = new Sequelize({
      username: config.database.user,
      password: config.database.password,
      host: config.database.host,
      dialect: config.database.dialect,
      database: config.database.name,
      port: config.database.port,
      logging: config.database.logging,
    });

    /** Initialize a new instance of the Spotify Web API client. */
    this.spotify = new Spotify({
      clientId: config.spotify.clientID,
      clientSecret: config.spotify.clientSecret,
      redirectUri: 'ellie://callback',
    });
  }

  /** Setup the bot's CommandHandler. */
  public commandHandler = new CommandHandler(this, {
    directory: path.join(__dirname, '..', 'commands'),
    prefix: this.config.prefix,
    aliasReplacement: /-/g,
    allowMention: true,
    handleEdits: true,
    commandUtil: true,
    commandUtilLifetime: 3e5,
  });

  /** Setup the bot's ListenerHandler. */
  public listenerHandler = new ListenerHandler(this, {
    directory: path.join(__dirname, '..', 'listeners'),
  });

  /** Do basic setup like loading modules and emitters. */
  public async setup() {
    this.commandHandler.useListenerHandler(this.listenerHandler);
    this.logger.info('Loading emitters.');
    this.listenerHandler.setEmitters({
      commandHandler: this.commandHandler,
      listenerHandler: this.listenerHandler,
    });

    try {
      this.logger.info('Loading modules.');
      this.commandHandler.loadAll();
      this.listenerHandler.loadAll();
      const modules = this.commandHandler.modules.size + this.listenerHandler.modules.size;
      this.logger.info(`Successfully loaded ${modules} modules.`);
    } catch (err) {
      this.logger.error(err);
      this.logger.warn('Some modules failed to load.');
    }
  }

  public async start() {
    this.logger.info(`Starting up Ellie v${version} and logging into the Discord API.`);

    if (process.version.includes('nightly') || process.version.includes('canary')) {
      this.logger.warn('You are running Ellie on an unstable version of Node. You may experience stability issues.');
      this.logger.warn('It is strongly recommended that you run a stable version of Node.');
    }

    if (process.env.pm_id) {
      this.logger.info('You are running Ellie with PM2. She will remain running in the background.');
      this.logger.info('She will also automatically restart upon crashing or disconnecting.');
    } else {
      this.logger.warn('You are not running Ellie with PM2. This is not recommended! Please consider switching to PM2.');
      this.logger.warn('It will allow Ellie to gracefully restart if she crashes or disconnects.');
    }
  }

  public async loginToDiscord(token: string) {
    await this.start();
    await this.setup();
    return this.login(token);
  }
}
