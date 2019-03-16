/**
 * Config.ts -- Ellie's configuration system.
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

import * as fs from 'fs';
import * as path from 'path';
import * as toml from 'toml';

const configFile = path.join(__dirname, '..', '..', '..', 'config.toml');

type Configuration = {
  owner?: string;
  prefix?: string;
  token?: string;
  database?: string;
  lastfm?: string;
  spotify?: {
    clientID?: string,
    clientSecret?: string;
  };
  github?: string;
  darksky?: string;
  nexusmods?: string;
  youtube?: string;
  repository?: string;
};

export default class Config {
  /** The command prefix used for invoking commands. */
  readonly prefix: string;
  /** The bot's owner. Used for commands that should be owner-only. */
  readonly owner: string;
  /** The Discord API token to use, used for connecting to the Discord API. */
  readonly token: string;
  /** The database URL to use. */
  readonly database: string;
  /** The LastFM API key to use. */
  readonly lastfm: string;
  /** The Spotify client ID and client secret to use. */
  readonly spotify: { clientID: string, clientSecret: string };
  /** The GitHub username and password to use. */
  readonly github: string;
  /** The Dark Sky API key to use. */
  readonly darksky: string;
  /** The Nexus Mods API key to use. */
  readonly nexusmods: string;
  /** The YouTube API key to use. */
  readonly youtube: string;
  /** The GitHub repository hosting the bot. */
  readonly repository: string;

  public constructor(string?: string) {
    const config = string ? (toml.parse(string) as Configuration) : {};
    const spotify = config.spotify || {};
    this.owner = process.env.ELLIE_OWNER_ID || config.owner || '';
    this.prefix = process.env.ELLIE_PREFIX || config.prefix || '!';
    this.token = process.env.ELLIE_TOKEN || config.token || '';
    this.database = process.env.ELLIE_DATABASE || config.database || '';
    this.lastfm = process.env.ELLIE_LASTFM_KEY || config.lastfm || '';
    this.spotify = {
      clientID: process.env.ELLIE_SPOTIFY_CLIENT_ID || spotify.clientID || '',
      clientSecret: process.env.ELLIE_SPOTIFY_CLIENT_SECRET || spotify.clientSecret || '',
    };
    this.github = process.env.ELLIE_GITHUB_TOKEN || config.github || '';
    this.darksky = process.env.ELLIE_DARKSKY_KEY || config.darksky || '';
    this.nexusmods = process.env.ELLIE_NEXUSMODS_KEY || config.nexusmods || '';
    this.youtube = process.env.ELLIE_YOUTUBE_KEY || config.youtube || '';
    this.repository = process.env.ELLIE_REPOSITORY || config.repository || '';
  }

  /** Loads the bot's configuration from a configuration file. */
  static initConfigFromFile(path: string = configFile): Config {
    return new Config(fs.readFileSync(path, 'utf8'));
  }
}
