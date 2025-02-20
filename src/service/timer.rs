use poise::serenity_prelude::{Context as SerenityContext, Http as SerenityHttp};
use timer::Timer;
use std::sync::Arc;
use crate::service::config as ConfigService;
use crate::service::discord as DiscordService;

pub async fn register_timer(ctx: SerenityContext, http: Arc<SerenityHttp>) -> Timer {
    let timer = *ConfigService::get_config().timer();

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(timer)).await; // Set timer from .env
        DiscordService::posture_check_callout(&ctx, &http).await;
    }
}