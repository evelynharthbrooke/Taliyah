# Ellie, a bot for Discord

This is the official GitHub repository for Ellie, a bot for Discord written using TypeScript and Node.js. This is my
first full blown attempt at creating a proper Discord bot, and if any issues occur, please file them on either GitLab
or GitHub and I will make sure that I take a look at them as soon as I can.

Ellie uses the [Discord.js](https://github.com/discordjs/discord.js) library for basic interaction with the Discord
API, as well as the fantastic and well-made Discord Akairo library, for easy integration of commands, listeners and
other stuff. You can check out said library by visitng the link [here](https://github.com/1Computer1/discord-akairo).

Ellie focuses on having a strong command toolbelt, while also being intuitive to use. One example is that Ellie uses
the prompting featureof the Akairo library, which intuitively asks users to enter their query in certain commands,
in case they send said command without applying any arguments. At some point, I also plan on making Ellie support
music so people can use it as a music bot as well, if they so desire.

## Installation

### Prerequisites

Alright, before we can get Ellie up and running, we'll need to make sure Node.js is installed, which we can do by
running the following command (or file) for your respective platform. Node is basically the only thing we need to
really install. Everything else can be done through `npm` or `yarn`, but for these instructions, I'll be using `npm`.

#### Windows

To install Node.js on Windows, you can head on over to the Node.js website ([located here](https://nodejs.org)) and
download the latest version available in the **Current** channel. You can also use the nightly builds if you so choose,
but I should note that nightly builds can be unstable at times, and as such I currently recommend against using said
builds, but if you still decide to use them, all I can say is that you _use them at your own risk_, and report any
issues you experience with the bot while using the nightly builds to me via the respective issue trackers and I'll
take a look.

#### macOS

First, let's get Homebrew installed if it isn't already, so we can easily install the Node Version Manager (nvm), a
really nice utility to manage and install multiple Node.js versions! 

**Note**: You may need Xcode and Xcode's tools for certain Homebrew features but I am not 100% sure on this. However, 
Xcode's comandline tools include Apple's version of git (currently at version 2.17.1). I'm not sure if git is already 
included in macOS without the dev tools, but installing the tools are pretty useful for other development purposes, 
so go ahead and install them.

```bash
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
```

Now, let's update Homebrew's formulae, so we're all up-to-date and won't be installing any outdated versions of any
packages.

```bash
brew update
```

##### With the Node Version Manager
This option allows you to install Node.js with the Node Version Manager, which is strongly recommended because `nvm`
allows you to e.g. manage multiple Node.js versions, which is ridiculously useful for testing purposes. But, you can
also install node without `nvm` and I'll describe how to do that in the next section.

```bash
brew install nvm && mkdir ~/.nvm
# add the below part to your current shell profile.
# this step is crucial if you actually want to use
# nvm.
export NVM_DIR="$HOME/.nvm"
. "#{opt_prefix}/nvm.sh"
```

Finally, we can install the latest version of Node.js.

```bash
nvm install node # this installs the latest Node.js version
```

##### Without the Node Version Manager
This option allows you to install Node.js without the Node Version Manager. If you would like to proceed with this route,
you only have to accomplish one step, which is to install Node.js using Homebrew. This will install `icu4c` which allows
full Unicode support, as well as Node.js itself. You can also install Node.js with OpenSSL 1.1 support, but its optional, 
since macOS already comes with its own SSL library, LibreSSL, found in BSD operating systems such as OpenBSD and FreeBSD. 
Anyways, babbling aside, you can install Node with the below command.

```bash
brew install node # simple, right?
```

#### Linux

Installing Node.js on Linux is pretty easy, just use your respective package manager included in your distribution 
to install Node.js. As an example, on Arch Linux, you can run the following command and you'll be done:

```bash
sudo pacman -S nodejs npm # This should install the latest available version of Node.js, as well as npm.
```

On distributions such as Ubuntu, Debian, Linux Mint, and other `apt`-based distributions, you can run the following
commands to install the latest current version of Node.js, as well as npm. (these commands are borrowed from the
Node.js Linux installation page on the Node.js website, which will be listed below).

```bash
curl -sL https://deb.nodesource.com/setup_10.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo apt-get install -y build-essential # required if you want to install native Node.js modules via npm!
```

For other distributions, such as Gentoo, openSUSE, and others, you can visit the respective page on the Node.js
website, which you can visit [here](https://nodejs.org/en/download/package-manager/).

### Installing the Bot

Now, we can actually download Ellie and set her up. This step 100% requires Git, as that is how we will 
be downloading her.

```bash
git clone https://github.com/KamranMackey/Ellie
```

If you'd like to use GitLab for the cloning process instead of GitHub, you can do that too. Just use 
the following command instead to clone from Ellie's GitLab mirror.

```bash
git clone https://gitlab.com/KamranMackey/Ellie
```

Alright, now let's `cd` into the download directory where we downloaded Ellie to.

```bash
cd Ellie
```

Now we can install Ellie's dependencies. On Windows, you will need to install the `windows-build-tools` 
package using npm, as Windows does not natively include build tools like Linux does. For macOS, just 
install Xcode and the commandline tools.

```bash
npm install
```

Just be patient while this process completes. It may take a while to complete, depending on your Internet 
speed as well as the speed of your system's SSD and/or hardrive.

### Configuring the Bot

Now we can set up Ellie. You will need to go to the developers site for Discord, and create a new application.
You can do this by going [here](https://discordapp.com/developers/applications/), logging in, and selecting
"Create an application" on the main page, and filling in the neccessary information. Once you have
successfully created an application, click on your application's card. Now, we'll have to create a
"Bot user" for the application. You can do this by selecting "Bot" on the left hand column, under
OAuth2, and clicking "Add Bot". This will add a bot user to your application.

Now, for the fun part! Let's grab the bot's token. You can do this by clicking the "Click to reveal token"
button underneath the Username field on the bot page. Copy the token given to you. Now, in the bot's root
directory, rename `config.sample.toml` to `config.toml`, and open the file. Paste the token into the token
field. While you have the file open, you may want to take this opportunity to enter your Discord user ID
in the "owner" field so you can use any owner-only commands that have been added, as well as any API keys
and usernames and passwords you'd like. I should note though that there is currently no error catching
implemented in any commands right now, so if you forget to add API keys or usernames/passwords, you will
encounter an error when trying to run the respective commands, so that's why I strongly suggest doing so.

Now, we are pretty much done. Now, onto the final step, which is actually running Ellie.

### Running the Bot

You have reached the final step of the install instructions. You're almost there. You just have to build
the bot and then start her up.

```bash
npm run build && npm start
```

Congratulations! You have (hopefully) successfully installed and set up Ellie, and you can now add the bot to
any guild you'd like! (if you have the permission to of course!)

**NOTE**: Ellie does not use PM2. When I personally start usiing PM2 for running Ellie, I will re-add instructions
on how to run Ellie using it.

### Licensing and Other Information
Ellie is licensed under the terms of the GNU General Public License, version 3.0, or any later versions that
may release in the near or far future. The license snippet is below, and the full terms can be found by looking 
at the `LICENSE` file, located in the root directory.

    Copyright © 2018-present Kamran Mackey

    Ellie is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Ellie is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Ellie. If not, see <http://www.gnu.org/licenses/>.