use std::env;
use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, ChannelId, MessageBuilder};
use std::sync::Arc;
use timer::Timer;

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

pub async fn register_timer(http: Arc<poise::serenity_prelude::Http>) -> Timer {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60 * 30)).await; // Set timer to 30 minutes
        posture_check_callout(&http).await;
    }
}

pub fn get_posture_callout_channel() -> ChannelId  {
    // TODO: Read initial setting from env, register changes via application command, e.g. "/set_channel #lobby"
    // FIX: Proper Error Handling - If unparsable, safely shutdown
    let channel_id = env::var("CHANNEL_ID").expect("missing CHANNEL_ID in .env").parse::<u64>().unwrap();

    ChannelId::new(channel_id)
}

pub async fn posture_check_callout(http: &poise::serenity_prelude::Http) {
    let channel_id = get_posture_callout_channel();

    let response = MessageBuilder::new()
        .push("@here") // TODO: Change this to get all users currently in voice channels
        .push("\n\nPOSTURE CHECK RIGHT NOW")
        .build();

    if let Err(why) = channel_id.say(http, &response).await {
        println!("Error sending message: {why:?}");
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Posture Bot...");
    println!("Fetching environment variables...");
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN in .env");
    let intents = serenity::GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![cooldown()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            println!("Running setup...");
            let http_clone = ctx.http.clone();
            // Get Posture Check Timer
            tokio::spawn(async move {
                register_timer(http_clone).await;
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