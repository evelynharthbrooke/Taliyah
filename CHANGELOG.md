# Ellie Changelog

## 2.0.0 - Unreleased (TBA)
This is a MAJOR release, moving from JavaScript to Microsoft's TypeScript language (used in 
projects like Visual Studio Code), as well as adding various new commands and new features.
### New Features
- Ellie has been rewritten in TypeScript instead of JavaScript. This should improve stability and 
speed. However, some commands were migrated from the JavaScript version, like `!pokemon`, just with 
a few changes to support TypeScript's syntax.
- Added this Changelog, for keeping track of the stuff I change.
- Added `!ban`. Bans a user from the current Discord guild.
- Added `!steamstatus`. Retrieves the current Steam server status.
- Added a listener that checks for message deletion, and logs it to a channel. Working on making this 
support per-guild channels instead of hardcoding the channel name, but that most likely will not be 
present in this release.
- Added the `convertToTitleCase` function to Utilities. Converts words or strings to (you guessed it) Title
Case. Used in `!pokemon` to convert ability names to proper Title Case instead of being hyphenated and in
Normal case.
- Added a `Constants` class. This class moves anything that would've been at the top level of a command's
code, to their own individual class, just in case something needs to be reused, easier than just porting
the code each time to a command's top level code.
### Changed
- The bot has been renamed from Erica to Ellie. Nothing major really, just a slight name change.
- Revamped the majority of the commands to use the embed description instead of embed fields. This (IMHO) 
makes things a fair bit cleaner and improves things on the mobile side, since Rich Embeds on Discord's 
mobile apps don't have multiple columns and show each field on a separate row, which doesn't look that
good.
- The `!pokemon` command now displays a Pokémon's abilities, and allows users to visit Bulbapedia to get
more information.
- Changed the logging system from `winston` to `signale`. Signale looks a lot better and works without much
configuration.
### Removed
- Removed the `!id` command. Its functionality was replaced by the `!user` command, which displays far more
information.