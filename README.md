# Posture Bot

A simple Discord bot that screams at you to adjust your posture every now and then - I know you out here shrimping, get up!

## Description

This codebase is mostly a solo adventure to learn some Rust! The bot is written using [Poise](https://github.com/serenity-rs/poise)!

## Installation

As I do not intend for this to be a grand ambition or long-term project I do not release any binaries, you have to compile it yourself.

1. Install Rust and Cargo using [Rustup](https://rustup.rs/)
2. Clone the repositories main branch `git clone https://github.com/Epzillon/posture_bot`
3. Rename `.env_placeholder` to `.env`
4. Configure the `.env` options (see [Configuration](#configuration))
5. Compile the project `cargo build --release`
6. Run the binary `./target/release/posture_bot` or `./posture_bot` from the release folder.

## <a name="configuration"></a>Configuration

This is a list of current configuration options and their expected values:

|     Option     |  Expected Value  | Required? | Default |
|----------------|------------------|-----------|---------|
| DISCORD_TOKEN  | Your Discord Bot token acquired from the Discord Developer Portal | Yes | N/A |
| CHANNEL_ID     | The Channel ID of the channel you wish the bot to send messages in | Yes | N/A |
| GUILD_ID       | The ID of your server | Yes | N/A |
| TIMER          | Number of seconds between callouts | Yes | 1800 (30min) |
| USER_THRESHOLD | Minimum amount of users in voice channels for the bot to send messages | Yes | 1 |
