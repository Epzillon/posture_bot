use poise::serenity_prelude::{Context as SerenityContext, Http as SerenityHttp};
use std::sync::Arc;
use crate::service::config::{self as ConfigService, AppConfigTrait};
use crate::service::discord as DiscordService;

// Creates and returns a timer that sleeps for a configured amount and then runs the posture check callout
pub async fn register_timer(ctx: SerenityContext, http: Arc<SerenityHttp>) {
    let timer_dur = *ConfigService::get_config().timer();
    let cleanup_timer_dur = *ConfigService::get_config().cleanup_timer();

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(timer_dur)).await; // Set timer from .env
        let posture_opt = DiscordService::posture_check_callout(&ctx, &http).await;

        // Register auto-deletion of message if we sent it
        if posture_opt.is_some() {
            match posture_opt {
                Some(posture_res) => {
                    match posture_res {
                        Ok(msg_ref) => {
                            // Register Timer
                            tokio::time::sleep(tokio::time::Duration::from_secs(cleanup_timer_dur)).await;
                            DiscordService::delete_message(&http, msg_ref).await;
                        },
                        Err(why) => {
                            println!("Failed to send posture check message. Reason: {}", why);
                        },
                    }
                },
                None => {
                    return;
                },
            }
        }
    }
}