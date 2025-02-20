use std::{env, fs, sync::Arc};
use poise::serenity_prelude::{self as serenity, ChannelId, ChannelType, Context as SerenityContext, GuildChannel, GuildId, UserId, Http as SerenityHttp, MessageBuilder};
use rand::{seq::IndexedRandom, Rng};
use timer::Timer;
use serde::Deserialize;

struct Data {} // User data, which is stored and accessible in all command invocations
#[derive(Debug, Deserialize)]
struct Config {
    discord_token: String,
    guild_id: u64,
    callout_channel_id: u64,
    message_phrases: Vec<String>,
    timer: u64,
    user_threshold: usize,
}
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

pub async fn register_timer(ctx: SerenityContext, http: Arc<SerenityHttp>) -> Timer {
    let timer = get_config().timer;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(timer)).await; // Set timer from .env
        posture_check_callout(&ctx, &http).await;
    }
}

// TODO: Move to lib/separate crate
/* HELPERS */
pub fn get_voice_channels(ctx: &SerenityContext, guild_id: GuildId) -> Vec<GuildChannel> {
    let mut voice_channels = Vec::<GuildChannel>::new();

    if let Some(guild) = ctx.cache.guild(guild_id) {
        for channel in guild.channels.values() {
            if let ChannelType::Voice = channel.kind {
                voice_channels.push(channel.clone());
            }
        }
    }

    voice_channels
}

pub fn get_all_active_voice_users(ctx: &SerenityContext, guild_id: GuildId) -> Vec<UserId> {
    let voice_channels = get_voice_channels(ctx, guild_id);
    let mut active_users = Vec::<UserId>::new();

    for channel in voice_channels {
        for member in channel.members(ctx.cache.clone()).unwrap() {
            active_users.push(member.user.id);
        }
    }

    active_users
}

fn get_random_posture_phrase() -> String {
    let phrases = get_config().message_phrases;

    let mut rng = rand::rng();
    let max = rng.random_range(1..phrases.len()) - 1;

    phrases[max].clone()
}

pub fn build_posture_message(ctx: &SerenityContext, guild_id: GuildId) -> String {
    let mut msg_builder = MessageBuilder::new();

    for user_id in get_all_active_voice_users(&ctx, guild_id) {
        msg_builder.mention(&user_id);
        msg_builder.push(" ");
    }

    msg_builder.push("\n\n");
    msg_builder.push(get_random_posture_phrase());

    msg_builder.build()
}
/* END OF HELPERS */

pub async fn posture_check_callout(ctx: &SerenityContext, http: &SerenityHttp) {
    let channel_id = ChannelId::new(get_config().callout_channel_id);
    let guild_id = GuildId::new(get_config().guild_id);

    if is_voice_active(&ctx, guild_id).await {
        let message = build_posture_message(&ctx, guild_id);
    
        if let Err(why) = channel_id.say(http, &message).await {
            println!("Error sending message: {why:?}");
        }
    }
}

pub async fn is_voice_active(ctx: &SerenityContext, guild_id: GuildId) -> bool {
    let voice_channels = get_voice_channels(&ctx, guild_id);

    for channel in voice_channels {
        let active_members = channel.members(ctx).unwrap();
        
        if active_members.len() > get_config().user_threshold {
            return true;
        }
    }

    return false;
}

fn get_config() -> Config {
    let config_str = fs::read_to_string("./config.json").expect("Unable to read config file.");

    serde_json::from_str(&config_str).expect("JSON was not well-formatted")
}

#[tokio::main]
async fn main() {
    println!("Starting Posture Bot...");
    println!("Reading configuration...");

    let token = get_config().discord_token;
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
                register_timer(ctx_clone, http_clone).await;
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