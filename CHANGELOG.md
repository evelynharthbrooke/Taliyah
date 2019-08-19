# Ellie Changelog

## 2.0.0 — Hydrogen (Unreleased)

This is a MAJOR release, moving from JavaScript to Microsoft's TypeScript language (used in 
projects like Visual Studio Code), as well as adding various new commands and some new features.

### Major Changes

- Ellie has been rewritten in TypeScript instead of JavaScript. This should improve stability and 
speed. However, some commands were migrated from the JavaScript version, like `!pokemon`, just with 
a few changes made to them to take into account TypeScript's various syntax changes.

### New Features

#### Commands

##### Moderation

- Added `ban` command. Allows server administrators and moderators (if they have the ability) to ban a
specified user from the Discord guild.

##### Extra

- Added `steamstatus` command. Retrieves information about the current status of Steam and various services
related to Steam, such as the Steam Database, and certain Valve games like Dota 2 and Counter-Strike: Global
Offensive.
- Added `weather` command. Gets the weather forcast for a specified location.

#### Listeners

- Added a `messagedelete` listener. Checks for message deletion, and logs it to a channel. Working on
making this support per-guild channels instead of hardcoding the channel name, but that most likely will
not be present in this release.
- Added the `debug` listener. Checks for debug events and then logs them to the console. Examples of
debug events would be heartbeat ACKs and shard spawns.

### Other Features

#### Miscellaneous

- Added the `convertToTitleCase` function to Utilities. Converts words or strings to (you guessed it) Title
Case. Used in `!pokemon` to convert ability names to proper Title Case instead of being hyphenated and in
Normal case.
- Added a `Constants` class. This class moves anything that would've been at the top level of a command's
code, to their own individual class, just in case something needs to be reused, easier than just porting
the code each time to a command's top level code.

### Changes

#### Command Changes

#### General

- Revamped the majority of the commands to use the embed description instead of embed fields. This (IMHO) 
makes things a fair bit cleaner and improves things on the mobile side, since message embeds when viewed
using Discord's mobile apps don't have multiple columns and show each field on a separate row, which doesn't 
look that good. The only command that doesn't use embed descriptions throughout the entire command is `!help`, 
for technical reasons.

#### Information

- Guild:
  - Added information about a server's Nitro Boost tier.

##### Music

- Lastfm:
  - Added a list of users' most recently played tracks (up to 5). Displays when a song is
    playing or not as well.
  - Switched to using the Spotify Web API instead of Last.fm's API for retrieving album
    artwork, due to Spotify having a wider selection of music and album art.
- Spotify:
  - Re-implemented the pre-existing set of Spotify commands, with some new features and a new consistent
  appearance shared between the command set.
  - Added `artist` subcommand. Retrieves information about a specified artist.
  - Added `user` subcommand. Retrieves information about a specified user.

##### Search

- Pokémon:
  - Implemented support for viewing a Pokémon's various abilities.
  - Added a link to Bulbapedia which allows users to get more information on a Pokémon.

##### Utilities

- Help:
  - Changed the layout when getting information on a individual command to use embed descriptions. However, please
    note that this does not yet affect getting information on all commands. This may change in the future.

#### General Changes

- The bot has been renamed from Erica to Ellie. Nothing major really, just a slight name change.
- Changed the logging library from `winston` to `signale`. Signale looks a lot better and works without much
configuration.
- Minor refinements to the codebase.

### Bugfixes

- Fixed a bug where the bot would fail to build with newer versions of TypeScript due to a semantic
error involving the Guild command.
- Fixed implementation issues with the GitHub user command, by swapping from importing the graphql
library to instead requiring it via a constant.

### Removed

- Removed the `!id` command. Its functionality was replaced by the `!user` command, which displays far more
information.

## 0.1.0 — Initial Release

This was the initial release of Ellie.
