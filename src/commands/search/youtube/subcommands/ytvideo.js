/**
 * ytvideo.js -- Retrieves information on a specified YouTube video.
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
const { MessageEmbed } = require('discord.js');
const moment = require("moment");
require("moment-duration-format");
const numeral = require("numeral");
const request = require('node-superfetch');
const { shorten } = require("../../../../util/utilities");
const config = require('../../../../../config.json');

// YouTube API-related stuff
const ytApiBase = "https://www.googleapis.com/youtube/v3/";
const ytMainBase = "https://www.youtube.com";
const videoEndpoint = ytApiBase + "videos";
const searchEndpoint = ytApiBase + "search";
const apiKey = config.youtube_key;

class YouTubeVideoCommand extends Command {
    constructor() {
        super('youtube-video', {
            category: 'Search',
            description: {
                content: 'Retrieves information on a specified YouTube video.',
                usage: '<video>'
            },
            args: [
                {
                    id: 'video',
                    match: 'content',
                    type: 'string',
                    prompt: {
                        start: 'What is the name of the YouTube video?'
                    }
                }
            ]
        })
    }

    async exec(message, { video }) {
        /**
        * The video search endpoint.
        * 
        * We use this endpoint to search the API for the user's query,
        * and then we use the video ID in the API response in the video
        * information endpoint to retrieve the actual information for the
        * video. Kind of hacky and uses a tiny bit more API calls than I'd
        * reasonably like, but oh well. It works like a charm.
        */
        let { body: vidSearch } = await request.get(searchEndpoint).query({
            part: 'snippet',
            type: 'video',
            maxResults: 1,
            regionCode: 'US',
            q: video,
            key: apiKey
        })

        /**
        * The video information endpoint.
        * 
        * We use this endpoint to turn the response we get from the video
        * search endpoint into a proper video response so we can get all
        * of the video information that we need.
        */
        let { body: videoInfo } = await request.get(videoEndpoint).query({
            part: 'snippet,statistics,contentDetails',
            type: 'video',
            maxResults: 1,
            regionCode: 'US',
            id: vidSearch.items[0].id.videoId,
            key: apiKey
        })

        // Duration format, used for nicely formatting the length of the video.
        const durFormat = "d [days], h [hours], m [minutes], s [seconds]"
        // The data source for the video. Gets the first video entry in the API
        // response, and we use that for all of the video information.
        const videoSource = videoInfo.items[0]
        // The thumbnail associated with the video.
        const thumbnail = videoSource.snippet.thumbnails.maxres ? videoSource.snippet.thumbnails.maxres.url : null
        // The author (or uploader) of the video.
        const author = videoSource.snippet.channelTitle
        const authorUrl = `[${author}](${ytMainBase}/channel/${videoSource.snippet.channelId})`
        // The title of the video.
        const videoTitle = videoSource.snippet.title
        // The description of the video.
        const videoDesc = shorten(videoSource.snippet.description, 383)
        // The URL for the video.
        const videoURL = `${ytMainBase}/watch?v=${videoSource.id}`
        // The upload date (or publish date) of the video.
        const publishedDate = moment.utc(videoSource.snippet.publishedAt).format('LL');
        // The length (or duration) of the video.
        const videoDuration = videoSource.contentDetails.duration;
        const videoLength = moment.duration(videoDuration, "minutes").format(durFormat, {
            largest: 2
        });
        // The number of tags associated with the video.
        const tagCount = videoSource.snippet.tags
        // Popularity metrics such as views, likes, and dislikes.
        const viewCount = numeral(videoSource.statistics.viewCount).format('0,0');
        const likeCount = numeral(videoSource.statistics.likeCount).format('0,0');
        const dislikeCount = numeral(videoSource.statistics.dislikeCount).format('0,0');
        const commentCount = numeral(videoSource.statistics.commentCount).format('0,0');

        const channelEmbed = new MessageEmbed().setColor(0xDD2825).setTitle(videoTitle)
            .setURL(videoURL)
            .setDescription(videoDesc)
            .setImage(thumbnail)
            .addField('❯ Uploaded By', authorUrl, true)
            .addField('❯ Duration', videoLength, true)
            .addField('❯ Upload Date', publishedDate, true)
            .addField('❯ Total Views', `${viewCount} view${viewCount.length > 1 ? 's' : ''}`, true)
            .addField('❯ Tags', `${tagCount.length} tag${tagCount.length > 1 ? 's' : ''}`, true)
            .addField('❯ Likes', `${likeCount} like${likeCount.length > 1 ? 's' : ''}`, true)
            .addField('❯ Dislikes', `${dislikeCount} dislike${dislikeCount.length > 1 ? 's' : ''}`, true)
            .addField('❯ Comments', `${commentCount} comment${commentCount.length > 1 ? 's' : ''}`, true)

            return message.util.send(channelEmbed);
    }
}

module.exports = YouTubeVideoCommand;
