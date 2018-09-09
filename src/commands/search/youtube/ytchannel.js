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
const config = require('../../../../config.json');

class YouTubeChannelCommand extends Command {
    constructor() {
        super('ytchannel', {
            aliases: ['ytchannel', 'ychannel'],
            category: 'Search',
            description: {
                content: 'Retrieves information on a specified YouTube channel.',
                usage: 'Enter the name of the YouTube channel.'
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

        const channelEmbed = new MessageEmbed()
            .setColor(0xDD2825)
            .setTitle(`Channel information for ${data.snippet.title}`)
            .setDescription(data.snippet.description)
            .setThumbnail(data.snippet.thumbnails.medium ? data.snippet.thumbnails.medium.url : null)
            .addField(`❯ Total Subscribers`, `${subsCount} subscriber${subsCount.length > 1 ? 's' : ''}`, true)
            .addField(`❯ Total Videos`, `${videoCount} video${videoCount.length > 1 ? 's' : ''}`, true)
            .addField(`❯ Total Views`, `${viewCount} view${viewCount.length > 1 ? 's' : ''}`, true)
            .addField(`❯ Creation Date`, `${publishedDate}`, true)

        function getCountryName() {
            return countries.getName(data.snippet.country, 'en').substr(0,13);
        }
        
        if (data.snippet.country != null) {
            channelEmbed.addField(`❯ Country of Origin`,`:flag_${data.snippet.country.toLowerCase()}: ${getCountryName()}`, true)
        } else {
            this.client.logger.log('info', `Could not detect the channel's country, omitting the country field from the embed.`)
        }
        
        return message.util.send(channelEmbed);
    }
}

module.exports = YouTubeChannelCommand;
