use poise::serenity_prelude::{Context as SerenityContext, GuildId, MessageBuilder};
use rand::Rng;
use crate::service::config as ConfigService;
use crate::service::discord as DiscordService;

fn get_random_posture_phrase() -> String {
    let config = ConfigService::get_config();
    let phrases = config.message_phrases();

    let mut rng = rand::rng();
    let max = rng.random_range(1..phrases.len()) - 1;

    phrases[max].clone()
}

pub fn build_posture_message(ctx: &SerenityContext, guild_id: GuildId) -> String {
    let mut msg_builder = MessageBuilder::new();

    for user_id in DiscordService::get_all_active_voice_users(&ctx, guild_id) {
        msg_builder.mention(&user_id);
        msg_builder.push(" ");
    }

    msg_builder.push("\n\n");
    msg_builder.push(get_random_posture_phrase());

    msg_builder.build()
}