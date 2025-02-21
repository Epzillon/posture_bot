use poise::serenity_prelude::{self as serenity};
use posture_bot::service::config::{self as ConfigService, SystemConfigTrait};
use posture_bot::service::timer as TimerService;
use posture_bot::service::commands as CommandsService;

#[tokio::main]
async fn main() {
    println!("Starting Posture Bot...");
    println!("Reading configuration...");

    let full_config = ConfigService::get_full_config();
    let token = full_config.discord_token();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![CommandsService::ignore_me(), CommandsService::ignore_status()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            println!("Setting up bot...");
            let http_clone = ctx.http.clone();
            let ctx_clone = ctx.clone();
            // Get Posture Check Timer
            tokio::spawn(async move {
                TimerService::register_timer(ctx_clone, http_clone).await;
            });

            println!("Posture Bot is up and running!");
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(CommandsService::Data {})
            })
        })
        .build();


    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}