# Posture Bot

A simple Discord bot that screams at you to adjust your posture every now and then - I know you out here shrimping, get up!

## Description

This codebase is mostly a solo adventure to learn some Rust! The bot is written using [Poise](https://github.com/serenity-rs/poise)!

## Installation

As I do not intend for this to be a grand ambition or long-term project I do not release any binaries, you have to compile it yourself.

1. Install Rust and Cargo using [Rustup](https://rustup.rs/)
2. Clone the repository's main branch `git clone https://github.com/Epzillon/posture_bot`
3. Rename `.env_placeholder` to `.env`
4. Rename `config_placeholder.json` to `config.json`
5. Configure the `.env` options (see [.env Values](#configuration_env))
6. Configure the `config.json` options (see [config.json Values](#configuration_json)).
7. Compile the project `cargo build --release`
8. Run the binary `./target/release/posture_bot` or `./posture_bot` from the release folder.

## <a name="configuration"></a>Configuration

This is a list of current configuration options and their expected values:

### <a name="configuration_env"></a>.env Values

These are the configuration values for the .env file.

|    Option      |           Expected Value          | Required? |   Default    | Comment |
|----------------|-----------------------------------|-----------|--------------|---------|
| ENVIRONMENT    | "PRODUCTION", "STAGING", or "DEV" | Yes       | "PRODUCTION" | Defines the environment of the application. Simply defines which config file (PRODUCTION: config.json, STAGING: config.staging.json, DEV: config.dev.json) to use. |
| DISCORD_TOKEN  | Your Discord Bot Token            | Yes       | N/A          | The Discord Bot Token acquired from the Discord Developer portal |

### <a name="configuration_json"></a>config.json Values

These are the configuration values for the config.json file(s).
Note: config.json, config.staging.json and config.dev are all valid and which one is being used is defined in the .env ENVIRONMENT variable.

|        Option          |  Expected Value  | Required? | Default | Comment |
|------------------------|------------------|-----------|---------|---------|
| guild_id               | The ID of your server | Yes | N/A | |
| callout_channel_id     | The Channel ID of the channel you wish the bot to send messages in | Yes | N/A | |
| message_phrases        | The phrases you want the bot to use | Yes | ["TIME TO STOP SHRIMPING!", "POSTURE CHECK RIGHT NOW!", "SHOW ME THAT BACK OF YOURS, NOW!", "GAMERS. RISE UP!"] |
| timer                  | Number of seconds between callouts | Yes | 1800 (30min) | |
| cleanup_timer          | Number of seconds after a callout to remove the message | Yes | 300 (5min) | |
| user_threshold         | Minimum amount of users in voice channels for the bot to send messages | Yes | 1 | |
| ignore_list            | List of user IDs to skip mentioning | No(?) | [] | Value still needs to exist but the list can be empty |

## <a name="commands"></a>Commands

These are the application commands available in the bot:

|    Command     | Parameters |                         Description                        |
|----------------|------------|------------------------------------------------------------|
| /ignore_me     | None       | Toggles ignore status for the calling user                 |
| /ignore_status | None       | Tells the calling user whether their current ignore status |