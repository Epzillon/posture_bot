use poise::serenity_prelude::{Context as SerenityContext, GuildId, MessageBuilder};
use rand::Rng;
use crate::service::config::{self as ConfigService, AppConfigTrait};
use crate::service::discord as DiscordService;

/// Retrieves one random phrase of the message phrases defined in the config
fn get_random_posture_phrase() -> String {
    // Get all configured phrases
    let config = ConfigService::get_config();
    let phrases = config.message_phrases();

    // Get random index
    let mut rng = rand::rng();
    let rng_idx = rng.random_range(1..phrases.len()) - 1;

    // Return phrase at random index
    phrases[rng_idx].clone()
}

/// Retrieves all non-ignored active users and builds a string with all mentioned users and a random posture phrase.
pub fn build_posture_message(ctx: &SerenityContext, guild_id: GuildId) -> String {
    let mut msg_builder = MessageBuilder::new();

    // Mention members
    for member in DiscordService::get_all_active_voice_members(&ctx, guild_id) {
        if !ConfigService::get_config().ignore_list().contains(&member.user.id.get()) {
            msg_builder.mention(&member);
            msg_builder.push(" ");
        }
    }

    // Add posture phrase
    msg_builder.push("\n\n");
    msg_builder.push(get_random_posture_phrase());

    // Build and return final message
    msg_builder.build()
}