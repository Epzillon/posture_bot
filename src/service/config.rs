/// Config Service to handle configuration of the bot.
/// 
/// # Description
/// Lots of structs and traits going on here so here's a summary:
/// 
/// The config.json contains two parts, the system configuration and the app configuration.
/// 
/// The System Configuration consists of necessary configuration for the bot to start and function properly.
/// The App Configuration consists of general purpose configuration which determines how the bot runs,
/// stuff such as message frequency (timer), user threshold, etc.
/// 
/// ## Structs
/// The SystemConfig struct defines the structure of data within the "system_config", while the AppConfig
/// struct defines the structure of the data within the "app_config".
/// 
/// The FullConfig struct represents the ENTIRE config.json, including the "system_config" while the
/// Config struct omits the "system_config" portion
/// 
/// ## Traits
/// Since this is a project for learning Rust I attempted to reduce code duplication and made two traits
/// for the structs that will primarily be used in the fucntion (This idea came directly from previous
/// OOP implementations based on inheritance). Actually making something like this required some
/// macro rule definitions for easily implementing the traits for both structs.
/// 
/// impl_system_config_trait implements the SystemConfigTrait and defines where to get the relevant system config values.
/// impl_app_config_trait implements the AppConfigTrait and defines where to get the relevant app config values.
/// 
/// These macros are used on the Config and FullConfig structs to automatically add the functionality of getting the relevant value for each key.
/// 
/// # Notes
/// Does this ruin the purpose of deserializing the values with serde_json? Yes.
/// Did this reduce the overall amount of code? No.
/// Did this reduce code duplication? Kinda? Now we deal with implementing functions for each key instead, but that could have some advantages later...
/// Did I learn something? Yes!
/// Was it worth it? Ehhh, I guess

use std::fs::{self, File};
use std::io::Write;
use std::error::Error;
use poise::serenity_prelude::User;
use serde::{Deserialize, Serialize};

macro_rules! impl_system_config_trait {
    ($struct_name: ident) => {
        impl SystemConfigTrait for $struct_name {
            fn discord_token(&self) -> &String {
                &self.system_config.discord_token
            }
            fn guild_id(&self) -> &u64 {
                &self.system_config.guild_id
            }
        }
    };
}

macro_rules! impl_app_config_trait {
    ($struct_name: ident) => {
        impl AppConfigTrait for $struct_name {
            fn callout_channel_id(&self) -> &u64 {
                &self.app_config.callout_channel_id
            }
            fn message_phrases(&self) -> &Vec<String> {
                &self.app_config.message_phrases
            }
            fn timer(&self) -> &u64 {
                &self.app_config.timer
            }
            fn user_threshold(&self) -> &usize {
                &self.app_config.user_threshold
            }
            fn ignore_list(&self) -> &Vec<u64> {
                &self.app_config.ignore_list
            }
            fn ignore_list_add(&mut self, user_id: u64) {
                self.app_config.ignore_list.push(user_id);
            }
            fn ignore_list_remove(&mut self, user_id: u64) {
                self.app_config.ignore_list.retain(|&user| user != user_id);
            }
        }
    };
}

pub trait SystemConfigTrait {
    fn discord_token(&self) -> &String;
    fn guild_id(&self) -> &u64;
}

pub trait AppConfigTrait {
    fn callout_channel_id(&self) -> &u64;
    fn message_phrases(&self) -> &Vec<String>;
    fn timer(&self) -> &u64;
    fn user_threshold(&self) -> &usize;
    fn ignore_list(&self) -> &Vec<u64>;
    fn ignore_list_add(&mut self, user_id: u64);
    fn ignore_list_remove(&mut self, user_id: u64);
}

/// The system configuration structure from config.js
#[derive(Debug, Serialize, Deserialize)]
struct SystemConfig {
    /// The Discord Bot Token
    discord_token: String,
    /// The current Discord Server Guild ID
    guild_id: u64,
}

/// The application configuration structure from config.js
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    /// Channel ID of channel to send posture check message in
    callout_channel_id: u64,
    /// Phrases used in the message
    message_phrases: Vec<String>,
    /// Timer in seconds between sent messages
    timer: u64,
    /// Minimum threshold of active users to exceed before sending message
    user_threshold: usize,
    /// List of users being ignored by the bot
    ignore_list: Vec<u64>,
}

/// Retrieves and deserializes the entire config.json file.
///
/// Use discouraged outside of bot setup.
#[derive(Debug, Serialize, Deserialize)]
pub struct FullConfig {
    system_config: SystemConfig,
    app_config: AppConfig,
}

/// Retrieves and deserializes the general application configurations of the config.json file.
#[derive(Debug, Deserialize)]
pub struct Config {
    app_config: AppConfig
}

/// Retrieves and deserializes the general application configurations of the config.json file.
#[derive(Debug, Deserialize)]
pub struct SysConfig {
    system_config: SystemConfig
}

// Implement default trait functionality
impl_system_config_trait!(FullConfig);
impl_app_config_trait!(FullConfig);
impl_system_config_trait!(SysConfig);
impl_app_config_trait!(Config);

/// Toggles ignore status for a user, returns true if the user is now ignored or false if the user is no longer being ignored
pub fn ignore_user(user: User) -> bool {
    let user_id = user.id.get();
    let mut config = get_full_config();

    if is_ignored(&user) {
        config.ignore_list_remove(user_id);

        let _ = update_app_config(config);

        return false;
    } else {
        config.ignore_list_add(user_id);

        let _ = update_app_config(config);

        return true;
    }
}

/// Checks whether a specified user is in the list of ignored users
pub fn is_ignored(user: &User) -> bool {
    let user_id = user.id.get();

    if get_full_config().ignore_list().contains(&user_id) {
        return true;
    }

    false
}

/// Retrieves the entire current configuration.
/// 
/// Use discouraged, see get_config() and get_sys_config().
pub fn get_full_config() -> FullConfig {
    let config_str = fs::read_to_string("./config.json").expect("Unable to read config file.");

    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
}

/// Retrieves the current configuration
pub fn get_config() -> Config {
    let config_str = fs::read_to_string("./config.json").expect("Unable to read config file.");

    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
}

/// Retrieves the current system configuration
pub fn get_sys_config() -> SysConfig {
    let config_str = fs::read_to_string("./config.json").expect("Unable to read config file.");

    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
}

/// Updates the config.json and returns the new config or fucking explodes or something idk
/// 
/// TODO: Make sure this does not explode when multiple people are calling, aka is thread safe
fn update_app_config(full_config: FullConfig) -> Result<FullConfig, Box<dyn Error>> {
    let json_data = serde_json::to_string_pretty(&full_config)?;
    
    let mut file = File::create("config.json")?;
    file.write_all(json_data.as_bytes())?;

    println!("Successfully updated config!");
    Ok(full_config)
}