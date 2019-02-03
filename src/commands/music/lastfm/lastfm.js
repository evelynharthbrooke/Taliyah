/**
 * lastfm.js -- Last.fm commands used to retrieve users'
 * Last.fm data.
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

const { Command } = require("discord-akairo");
const { MessageEmbed } = require("discord.js");
const request = require("node-superfetch");
const moment = require("moment");
const numeral = require("numeral");
const config = require("../../../../config.json");

class LastFMRecentCommand extends Command {
    constructor() {
        super('lastfm', {
            aliases: ["lfm", "fm", "lastfm"],
            category: "Music",
            description: {
                content: "Retrieves yours or another users' Last.FM state. Mode can be basic, or embed.",
                usage: "<username>, optional: <mode>"
            },
            args: [
                {
                    id: "user",
                    type: "string",
                    prompt: {
                        start: "Enter the name of the last.fm user you'd like to search for."
                    }
                },
                {
                    id: 'mode',
                    type: 'string',
                    match: "rest",
                    default: "embed",
                    prompt: {
                        optional: true,
                        start: "Which mode would you like? You can pick between basic, and embed."
                    }
                }
            ]
        })
    }

    async exec(message, { mode, user }) {
        // Endpoint for the last.fm API.
        const lastfm_base = 'https://ws.audioscrobbler.com/2.0/?method='
        // API methods
        const rt_method = 'user.getRecentTracks'
        const ui_method = 'user.getInfo'
        // Recent Tracks query.
        const song_query = `&user=${user}&api_key=${config.lastfm_key}&limit=5&format=json`
        const song_rq_url = `${lastfm_base}${rt_method}${song_query}`
        const { body: lfm_rt } = await request.get(song_rq_url)
        // User Information query.
        const user_query = `&user=${user}&api_key=${config.lastfm_key}&format=json`
        const user_rq_url = `${lastfm_base}${ui_method}${user_query}`
        const { body: lfm_ui } = await request.get(user_rq_url)
        // Scrobble information
        const lfm_total = numeral(lfm_rt.recenttracks["@attr"].total).format('0.0a')
        const track = lfm_rt.recenttracks.track[0]
        const lfm_user = lfm_rt.recenttracks["@attr"].user
        const lfm_song = track.name
        const lfm_album = track.album["#text"]
        const lfm_artist = track.artist["#text"]
        // User Information
        const lfm_user_url = lfm_ui.user.url
        const lfm_country = lfm_ui.user.country
        const lfm_sub = lfm_ui.user.subscriber
        const lfm_registered = moment.unix(lfm_ui.user.registered.unixtime).format('lll');

        if (mode == "basic") {
            if (!track.hasOwnProperty("@attr")) {
                return message.channel.send(`${lfm_user} last listened to ${lfm_song} on ${lfm_album} by ${lfm_artist}.`)
            } else {
                return message.channel.send(`${lfm_user} is listening to ${lfm_song} on ${lfm_album} by ${lfm_artist}.`)
            }
        } else if (mode == "embed") {
            const lfm_embed = new MessageEmbed()

            lfm_embed.setTitle(`Last.fm information for user ${lfm_user}`)
            lfm_embed.setURL(lfm_user_url)

            if (track["image"][3]["#text"] == "") {
                this.client.logger.log("No immage attached to track, omitting from embed.")
            } else {
                lfm_embed.setThumbnail(lfm_rt.recenttracks.track[0]["image"][3]["#text"])
            }

            function getSubStatus() {
                if (lfm_sub == 0) {
                    return "No"
                } else {
                    return "Yes"
                }
            }

            if (track.hasOwnProperty("@attr")) {
                lfm_embed.setDescription(`${lfm_user} is currently listening to ${lfm_song} on ${lfm_album} by ${lfm_artist}.` +
                                         "\n\n" + `[View ${track.name} on last.fm →](${track.url})`)
            } else {
                lfm_embed.setDescription(`${lfm_user} last listened to ${lfm_song} on ${lfm_album} by ${lfm_artist}` +
                                         "\n\n" + `[View ${track.name} on last.fm →](${track.url})`)
            }

            lfm_embed.addField("❯ Total Scrobbles", lfm_total, true)
            lfm_embed.addField("❯ Country", lfm_country, true)
            lfm_embed.addField("❯ Subscriber?", getSubStatus(), true)
            lfm_embed.addField("❯ Registered On", lfm_registered, true)

            return message.channel.send(lfm_embed);
        }
    }
}

module.exports = LastFMRecentCommand;