use poise::serenity_prelude::{self as serenity};
use posture_bot::service::config as ConfigService;
use posture_bot::service::timer as TimerService;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


/// Retrieves cooldown until next posture check
#[poise::command(slash_command)]
async fn cooldown(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let response = format!("This is currently WIP...");
    ctx.say(response).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    println!("Starting Posture Bot...");
    println!("Reading configuration...");

    let full_config = ConfigService::get_full_config();
    let token = full_config.discord_token();
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![cooldown()],
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
                Ok(Data {})
            })
        })
        .build();


    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}