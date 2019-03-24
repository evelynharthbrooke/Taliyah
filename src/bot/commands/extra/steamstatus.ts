/**
 * steamstatus.ts -- Retrieves the current status of Steam. Uses a
 * combination of steamgaug.es and steamstat.us to get the status.
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
import Constants from '../../utils/Constants';

export default class SteamStatusCommand extends Command {
  public constructor() {
    super('steamstatus', {
      aliases: ['steamstatus'],
      category: 'Extra',
      description: {
        content: 'Displays the current status of Steam.',
        usage: '<blank>',
      },
    });
  }

  public async exec(message: Message) {
    const STEAM_EMBED = new MessageEmbed();
    const STEAM_REQUEST = await request.get('https://steamgaug.es/api/v2');
    const STEAM_STATUS_REQUEST = await request.get('https://crowbar.steamstat.us/Barney');

    if (STEAM_STATUS_REQUEST.status === 500 || STEAM_REQUEST.status === 500) {
      return message.channel.send(
        'Looks like either steamstat.us or steamgaug.es is down! ' +
        'I cannot get the current Steam status. Please try again later.');
    }

    const STEAM_COMMUNITY = Constants.STEAM_STATUS_CODES[STEAM_REQUEST.body.SteamCommunity.online];
    const STEAM_COMMUNITY_TIME = STEAM_REQUEST.body.SteamCommunity.time;
    const STEAM_STORE = Constants.STEAM_STATUS_CODES[STEAM_REQUEST.body.SteamStore.online];
    const STEAM_STORE_TIME = STEAM_REQUEST.body.SteamStore.time;
    const STEAM_USER_API = Constants.STEAM_STATUS_CODES[STEAM_REQUEST.body.ISteamUser.online];
    const STEAM_USER_API_TIME = STEAM_REQUEST.body.ISteamUser.time;
    const STEAM_USERS_ONLINE = STEAM_STATUS_REQUEST.body.services['online'].title;
    const STEAM_CMS = STEAM_STATUS_REQUEST.body.services['cms'].title;
    const STEAM_CMS_WS = STEAM_STATUS_REQUEST.body.services['cms-ws'].title;
    const STEAM_DB = STEAM_STATUS_REQUEST.body.services['database'].title;
    const STEAM_DOTA_2 = STEAM_STATUS_REQUEST.body.services['dota2'].title;
    const STEAM_TF2 = STEAM_STATUS_REQUEST.body.services['tf2'].title;
    const STEAM_CSGO_SESSIONS = STEAM_STATUS_REQUEST.body.services['csgo_sessions'].title;
    const STEAM_CSGO_INVENTORIES = STEAM_STATUS_REQUEST.body.services['csgo_community'].title;
    const STEAM_CSGO_MM_SCHEDULER = STEAM_STATUS_REQUEST.body.services['csgo_mm_scheduler'].title;

    console.log(STEAM_STATUS_REQUEST.body);

    STEAM_EMBED.setTitle('Steam Status');
    STEAM_EMBED.setThumbnail(Constants.STEAM_LOGO);
    STEAM_EMBED.setURL('https://store.steampowered.com');
    STEAM_EMBED.setColor(0x66c0f4);
    STEAM_EMBED.setDescription(
      '**__Steam Services__**:\n' +
      `**Users Online on Steam**: ${STEAM_USERS_ONLINE}\n` +
      `**Steam Community**: ${STEAM_COMMUNITY} (${STEAM_COMMUNITY_TIME}ms)\n` +
      `**Steam Store**: ${STEAM_STORE} (${STEAM_STORE_TIME}ms)\n` +
      `**Steam Web API**: ${STEAM_USER_API} (${STEAM_USER_API_TIME}ms)\n` +
      `**Steam Connection Managers**: ${STEAM_CMS}\n` +
      `**Steam Connection Managers (WS)**: ${STEAM_CMS_WS}\n\n` +
      '**__Game APIs__**:\n' +
      `**Dota 2 API**: ${STEAM_DOTA_2}\n` +
      `**Team Fortress 2 API**: ${STEAM_TF2}\n\n` +
      '**__Counter-Strike: Global Offensive__**:\n' +
      `**Sessions Logon**: ${STEAM_CSGO_SESSIONS}\n` +
      `**Player Inventories**: ${STEAM_CSGO_INVENTORIES}\n` +
      `**Matchmaking Scheduler**: ${STEAM_CSGO_MM_SCHEDULER}\n\n` +
      '**__Other Services__**:\n' +
      `**[Steam Database](https://steamdb.info)**: ${STEAM_DB}`,
    );
    STEAM_EMBED.setFooter('Powered by steamgaug.es & steamstat.us');
    STEAM_EMBED.setTimestamp();

    return message.channel.send(STEAM_EMBED);
  }
}
