/**
 * ytchannel.js -- Retrieves information on a YouTube channel.
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
const { Command } = require('discord-akairo');
const countries = require('i18n-iso-countries');
const { MessageEmbed } = require('discord.js');
const moment = require('moment');
const numeral = require('numeral');
const request = require('node-superfetch');
const pluralize = require('pluralize')
const config = require('../../../../../config.json');

class YouTubeChannelCommand extends Command {
    constructor() {
        super('youtube-channel', {
            category: 'Search',
            description: {
                content: 'Retrieves information on a specified YouTube channel.',
                usage: '<channel>'
            },
            args: [
                {
                    id: 'channelName',
                    type: 'string',
                    match: 'content',
                    prompt: {
                        start: 'What is the name of the YouTube channel?'
                    }
                }
            ]
        })
    }

    async exec(message, { channelName }) {
        const yt_base = "https://www.googleapis.com/youtube/v3/"
        const channel = yt_base + "channels"
        const search = yt_base + "search"
        const login = config.youtube_key

        let { body: channel_search } = await request.get(search).query({
            part: 'snippet',
            q: channelName,
            maxResults: 1,
            type: 'channel',
            key: login
        })

        let { body: channel_info } = await request.get(channel).query({
            part: 'snippet,statistics',
            type: 'channel',
            maxResults: 1,
            id: channel_search.items[0].id.channelId,
            key: login
        })

        if (!channel_info.items.length) return msg.channel.send(`Could not find any results for ${channel_name}!`);

        const data = channel_info.items[0]
        const publishedDate = moment.utc(data.snippet.publishedAt).format('LL');
        const videoCount = numeral(data.statistics.videoCount).format('0,0').toLocaleUpperCase();
        const subsCount = numeral(data.statistics.subscriberCount).format('0a').toLocaleUpperCase();
        const viewCount = numeral(data.statistics.viewCount).format('0a').toLocaleUpperCase();

        function getCountry() {
            if (data.snippet.country != null) {
                return countries.getName(data.snippet.country, 'en').substr(0, 13);
            } else {
                return "No country available."
            }
        }

        const channelEmbed = new MessageEmbed()
        channelEmbed.setColor(0xDD2825)
        channelEmbed.setTitle(`Channel information for ${data.snippet.title}`)
        channelEmbed.setDescription(
            `${data.snippet.description}\n\n` +
            `**Country**: ${getCountry()}\n` +
            `**Total Subscribers**: ${pluralize('subscriber', subsCount, true)}\n` +
            `**Total Videos**: ${pluralize('video', videoCount, true)}\n` +
            `**Total Views**: ${pluralize('view', viewCount, true)}\n` +
            `**Creation Date**: ${publishedDate}`
        )
        channelEmbed.setThumbnail(data.snippet.thumbnails.medium ? data.snippet.thumbnails.medium.url : null)

        return message.util.send(channelEmbed);
    }
}

module.exports = YouTubeChannelCommand;
