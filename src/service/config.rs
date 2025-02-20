use std::fs;
use serde::Deserialize;

/// Retrieves and deserializes the general purpose configurations from config.json
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Channel ID of channel to send posture check message in
    callout_channel_id: u64,
    /// Phrases used in the message
    message_phrases: Vec<String>,
    /// Timer in seconds between sent messages
    timer: u64,
    /// Minimum threshold of active users to exceed before sending message
    user_threshold: usize,
}

impl AppConfig {
    pub fn callout_channel_id(&self) -> &u64 { &self.callout_channel_id }
    pub fn message_phrases(&self) -> &Vec<String> { &self.message_phrases }
    pub fn timer(&self) -> &u64 { &self.timer }
    pub fn user_threshold(&self) -> &usize { &self.user_threshold }
}


/// Retrieves and deserializes the entire config.json file.
///
/// Use discouraged outside of bot setup.
#[derive(Debug, Deserialize)]
pub struct FullConfig {
    /// The Discord Bot Token
    discord_token: String,
    /// The Guild ID of the Discord Server
    guild_id: u64,
    callout_channel_id: u64,
    message_phrases: Vec<String>,
    timer: u64,
    user_threshold: usize,
}

impl FullConfig {
    pub fn discord_token(&self) -> &String { &self.discord_token }
    pub fn guild_id(&self) -> &u64 { &self.guild_id }
    pub fn callout_channel_id(&self) -> &u64 { &self.callout_channel_id } // Look at getting default system message channel if null
    pub fn message_phrases(&self) -> &Vec<String> { &self.message_phrases }
    pub fn timer(&self) -> &u64 { &self.timer }
    pub fn user_threshold(&self) -> &usize { &self.user_threshold }
}

/// Retrieves the entire current configuration.
/// 
/// Use discouraged, see get_config().
pub fn get_full_config() -> FullConfig {
    let config_str = fs::read_to_string("./config.json").expect("Unable to read config file.");

    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
}

/// Retrieves the current configuration
pub fn get_config() -> AppConfig {
    let config_str = fs::read_to_string("./config.json").expect("Unable to read config file.");

    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
}