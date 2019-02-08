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

class LastfmCommand extends Command {
    constructor() {
        super('lastfm', {
            aliases: ["lfm", "fm", "lastfm"],
            category: "Music",
            description: {
                content: "Retrieves a users' last.fm state.",
                usage: "<set> <username> or <username>"
            },
            args: [
                {
                    id: "set",
                    type: "flag"
                }
            ]
        })
    }
}



// class LastFMRecentCommand extends Command {
//     constructor() {
//         super('lastfm', {
//             aliases: ["lfm", "fm", "lastfm"],
//             category: "Music",
//             description: {
//                 content: "Retrieves yours or another users' Last.FM state.",
//                 usage: "<username>, optional: <mode>"
//             },
//             args: [
//                 {
//                     id: "rest",
//                     type: "rest"
//                 },
//             ]
//         })
//     }

//     async exec(message, { user }) {
//         // Endpoint for the last.fm API.
//         const lastfm_base = 'https://ws.audioscrobbler.com/2.0/?method='
//         // API methods
//         const recentTracksMethod = 'user.getRecentTracks'
//         const userInfoMethod = 'user.getInfo'
//         const lovedTracksMethod = 'user.getLovedTracks'
//         const libraryArtistsMethod = 'library.getArtists'
//         const query = `&user=${user}&api_key=${config.lastfm_key}&limit=5&format=json`
//         const song_rq_url = `${lastfm_base}${recentTracksMethod}${query}`
//         const { body: lfm_rt } = await request.get(song_rq_url)
//         // User Information query.
//         const user_rq_url = `${lastfm_base}${userInfoMethod}${query}`
//         const { body: lfm_ui } = await request.get(user_rq_url)
//         // Get Loved Tracks query
//         const loved_rq_url = `${lastfm_base}${lovedTracksMethod}${query}`
//         const { body: lfm_lt } = await request.get(loved_rq_url)
//         // Get Artists query
//         const artists_rq_url = `${lastfm_base}${libraryArtistsMethod}${query}`
//         const { body: lfm_la } = await request.get(artists_rq_url)
//         // Scrobble information
//         const lfm_total = numeral(lfm_rt.recenttracks["@attr"].total).format('0.0a')
//         const track = lfm_rt.recenttracks.track[0]
//         const lfm_user = lfm_rt.recenttracks["@attr"].user
//         const lfm_song = track.name
//         const lfm_album = track.album["#text"]
//         const lfm_artist = track.artist["#text"]
//         // User Information
//         const lfm_user_url = lfm_ui.user.url
//         const lfm_country = lfm_ui.user.country
//         const lfm_loved = lfm_lt.lovedtracks["@attr"].total
//         const lfm_artists = lfm_la.artists["@attr"].total
//         const lfm_sub = lfm_ui.user.subscriber
//         const lfm_registered = moment.unix(lfm_ui.user.registered.unixtime).format('ll');
//         const lfm_time_registered = moment(lfm_registered).toNow(true)

//         if (user == null) {
//             message.channel.send("Looks like you haven't entered a last.fm username!");
//             return;
//         }

//         const lfm_embed = new MessageEmbed()

//         lfm_embed.setTitle(`Last.fm information for user ${lfm_user}`)
//         lfm_embed.setURL(lfm_user_url)

//         if (track["image"][3]["#text"] == "") {
//             this.client.logger.log('info', 'No immage attached to Last.fm track, omitting from embed.')
//         } else {
//             lfm_embed.setThumbnail(lfm_rt.recenttracks.track[0]["image"][3]["#text"])
//         }

//         function getSubStatus() {
//             if (lfm_sub == 0) {
//                 return "No"
//             } else {
//                 return "Yes"
//             }
//         }

//         if (track.hasOwnProperty("@attr")) {
//             lfm_embed.setDescription(`${lfm_user} is currently listening to ${lfm_song} on ${lfm_album} by ${lfm_artist}.` +
//                 "\n\n" + `[View track ${track.name} on Last.fm →](${track.url}`)
//         } else {
//             lfm_embed.setDescription(`${lfm_user} last listened to ${lfm_song} on ${lfm_album} by ${lfm_artist}.` +
//                 "\n\n" + `[View track ${track.name} on Last.fm →](${track.url})`)
//         }

//         lfm_embed.addField("❯ Total Scrobbles", lfm_total, true)
//         lfm_embed.addField("❯ Loved Tracks", lfm_loved, true)
//         lfm_embed.addField("❯ Total Artists", lfm_artists, true)
//         lfm_embed.addField("❯ Country", lfm_country, true)
//         lfm_embed.addField("❯ Subscriber?", getSubStatus(), true)
//         lfm_embed.addField("❯ Registered On", `${lfm_registered} (${lfm_time_registered})`, true)

//         return message.channel.send(lfm_embed);
//     }
// }

// module.exports = LastFMRecentCommand;