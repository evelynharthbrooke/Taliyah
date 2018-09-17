# Erica, a bot for Discord

This is the official GitHub repository for Erica, a bot for Discord written using JavaScript and Node.js. This is my first full blown
attempt at creating a proper Discord bot, and if any issues occur, please file them on either GitLab or GitHub and I'll make sure that I
take a look at them as soon as I can.

Erica uses the [Discord.js](https://github.com/discordjs/discord.js) library for basic interaction with the Discord API, as well as the
fantastic and well-made Discord Akairo library, for easy integration of commands, listeners and other stuff. You can check out said library
by visitng the link [here](https://github.com/1Computer1/discord-akairo).

Erica focuses on having a strong command toolbelt, while also being intuitive to use. One example is that Erica uses the prompting feature
of the Akairo library, which intuitively asks users to enter their query in certain commands, in case they send said command without applying 
any arguments. At some point, I also plan on making Erica support music so people can use it as a music bot as well, if they so desire.

## Installation

### Prerequisites

Alright, before we can get Erica up and running, we'll need to make sure Node.js is installed, which we can do by running the following
command (or file) for your respective platform. Node is basically the only thing we need to really install. Everything else can be
done through `npm` or `yarn`, but for these instructions, I'll be using `npm`.

#### Windows

To install Node.js on Windows, you can head on over to the Node.js website ([located here](https://nodejs.org)) and download the latest
version available in the **Current** channel. You can also use the nightly builds if you so choose, but I should note that nightly builds
can be unstable at times, and as such I currently recommend against using said builds, but if you still decide to use them, all I can say
is that you _use them at your own risk_, and report any issues you experience with the bot while using the nightly builds to me via the
respective issue trackers and I'll take a look.

#### macOS

First, let's get Homebrew installed if it isn't already, so we can easily install the Node Version Manager (nvm), a really nice utility
to manage and install multiple Node.js versions! **Note**: You may need Xcode and Xcode's command-line tools for certain Homebrew features
but I am not 100% sure on this. However, Xcode's comandline tools do include Apple's version of git (v2.13.6 at the time of writing, and
I'm not sure if git is already included in macOS without the dev tools but meh), so I recommend installing them anyways.

```bash
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
```

Now, let's update Homebrew's formulae, so we're all up-to-date and won't be installing any outdated versions of any packages.

```bash
brew update
```

Then, let's install the Node Version Manager.

```bash
brew install nvm && mkdir ~/.nvm
# add the below part to your current shell profile!
export NVM_DIR="$HOME/.nvm"
. "#{opt_prefix}/nvm.sh"
```

Finally, we can install the latest version of Node.js!

```bash
nvm install node # this installs the latest Node.js version
```

#### Linux

Installing Node.js on Linux is pretty easy, just use your respective package manager included in your distribution to install
Node.js. For instance, on Arch Linux, you can run the following command and you'll be done!:

```bash
sudo pacman -S nodejs npm # This should install the latest available version of Node.js, as well as npm.
```

On distributions such as Ubuntu, Debian, Linux Mint, and other `apt`-based distributions, you can run the following commands to
install the latest current version of Node.js, as well as npm. (these commands are borrowed from the Node.js Linux installation
page on the Node.js website, which will be listed below).

```bash
curl -sL https://deb.nodesource.com/setup_10.x | sudo -E bash -
sudo apt-get install -y nodejs
sudo apt-get install -y build-essential # required if you want to install native Node.js modules via npm!
```

For other distributions, such as Gentoo, openSUSE, and others, you can visit the "Installing node.js via package manager" page
on the Node.js website, which you can visit [here](https://nodejs.org/en/download/package-manager/).

### Installing

Now, we can actually download Erica and set her up. This step 100% requires Git, as that is how we will be downloading her.

```bash
git clone https://github.com/KamranMackey/Erica
```

If you'd like to use GitLab for the cloning process instead of GitHub, you can do that too. Just use the following command instead 
to clone from Erica's GitLab mirror.

```bash
git clone https://gitlab.com/KamranMackey/Erica
```

Alright, now let's `cd` into the download directory where we downloaded Erica to.

```bash
cd Erica
```

Now we can install Erica's dependencies. On Windows, you will need to install the `windows-build-tools` package using npm, as Windows
does not natively include build tools like Linux does. For macOS, just install Xcode and the commandline tools.

```bash
npm install
```

Just be patient while this process completes. It may take a while depending on how fast or slow your system is.

### Configuring

Now we can set up Erica. You will need to go to the developers site for Discord, and create a new application. You can do this by
going [here](https://discordapp.com/developers/applications/), logging in, and selecting "Create an application" on the main page,
and filling in the neccessary information. Once you have successfully created an application, click on your application's card. Now,
we'll have to create a "Bot user" for the application. You can do this by selecting "Bot" on the left hand column, under OAuth2, and
clicking "Add Bot". This will add a bot user to your application.

Now, for the fun part! Let's grab the bot's token. You can do this by clicking the "Click to reveal token" button underneath the Username
field on the bot page. Copy the token given to you. Now, in the bot's root directory, rename `config.sample.json` to `config.json`, and
open the file. Paste the token into the token field. While you have the file open, you may want to take this opportunity to enter your
Discord user ID in the "owner" field so you can use any owner-only commands that have been added, as well as any API keys and usernames
and passwords you'd like. I should note though that there is currently no error catching implemented in any commands right now, so if
you forget to add API keys or usernames/passwords, you will encounter an error when trying to run the respective commands, so that's why
I strongly suggest doing so.

Now, we are pretty much done. Onto the final step!

### Running

We've finally reached the final step! To run the bot, run the following command.

```bash
npm start
```

This will start the bot. You will see a bunch of messages letting you know the bot is loading. Also: It is also possible to run Erica
under PM2, so you are entirely free to use that if you wish.
