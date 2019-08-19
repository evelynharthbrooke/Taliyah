/**
 * Constants.ts -- Constants made available for easy access anywhere
 * in the bot's source code.
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

export default {
  STEAM_LOGO: 'https://upload.wikimedia.org/wikipedia/commons/thumb/8/83/Steam_icon_logo.svg/250px-Steam_icon_logo.svg.png',
  DATE_FORMAT: 'MMM Do, YYYY [@] h:mm a',

  DEFAULT_VOLUME: 1, // Default voice volume. Unused for now.

  STEAM_STATUS_CODES: {
    1: 'Online',
    2: 'Offline',
  } as { [key: string]: string },

  ACTIVITY_NAMES: {
    PLAYING: 'playing',
    WATCHING: 'watching',
    STREAMING: 'streaming',
    LISTENING: 'listening to',
  } as { [key: string]: string },

  GUILD_TIERS: {
    0: 'No Current Tier',
    1: 'Level 1',
    2: 'Level 2',
    3: 'Level 3',
  } as { [key: string]: string},

  GUILD_VERIFICATION_LEVELS: {
    0: 'None - Unrestricted',
    1: 'Low - Must have a verified email',
    2: 'Medium - Registered on Discord for 5+ minutes',
    3: '(╯°□°）╯︵ ┻━┻ - In the server for 10+ minutes',
    4: '┻━┻ ﾐヽ(ಠ益ಠ)ノ彡┻━┻) - Must have a verified phone number',
  } as { [key: string]: string },

  GUILD_EXPLICIT_FILTER: {
    0: 'Disabled',
    1: 'No role',
    2: 'Everyone',
  } as { [key: string]: string },

  GUILD_REGIONS: {
    brazil: 'Brazil',
    hongkong: 'Hong Kong',
    japan: 'Japan',
    russia: 'Russia',
    singapore: 'Singapore',
    southafrica: 'South Africa',
    sydney: 'Sydney',
    'us-central': 'US Central',
    'us-east': 'US East',
    'us-south': 'US South',
    'us-west': 'US West',
    'eu-central': 'Central Europe',
    'eu-west': 'Southern Europe',
  } as { [key: string]: string },

  PERMISSIONS: {
    ADMINISTRATOR: 'Administrator',
    VIEW_AUDIT_LOG: 'View Audit Log',
    MANAGE_GUILD: 'Manage Guild',
    MANAGE_ROLES: 'Manage Roles',
    KICK_MEMBERS: 'Kick Members',
    BAN_MEMBERS: 'Ban Members',
    CREATE_INSTANT_INVITE: 'Create Instant Invites',
    CHANGE_NICKNAME: 'Change Nickname',
    MANAGE_NICKNAMES: 'Manage Nicknames',
    MANAGE_EMOJIS: 'Manage Emojis',
    MANAGE_WEBHOOKS: 'Manage Webhooks',
    VIEW_CHANNEL: 'Access to Text/Voice Channels',
    SEND_MESSAGES: 'Send Messages',
    SEND_TTS_MESSAGES: 'Send TTS Messages',
    MANAGE_MESSAGES: 'Manage Messages',
    EMBED_LINKS: 'Embed Links',
    ATTACH_FILES: 'Attach Files',
    READ_MESSAGE_HISTORY: 'See Message History',
    MENTION_EVERYONE: 'Mention Everyone',
    USE_EXTERNAL_EMOJIS: 'Use External Emojis',
    ADD_REACTIONS: 'Add Reactions',
    CONNECT: 'Connect',
    SPEAK: 'Speak',
    MUTE_MEMBERS: 'Mute Members',
    DEAFEN_MEMBERS: 'Deafen Members',
    MOVE_MEMBERS: 'Move Members',
    USE_VAD: 'Use Voice Activity',
  } as { [key: string]: string },
};
