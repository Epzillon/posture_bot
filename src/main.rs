use std::env;
use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, ChannelId, ChannelType, Context as SerenityContext, GuildChannel, GuildId, UserId, Http as SerenityHttp, MessageBuilder};
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

pub async fn register_timer(ctx: SerenityContext, http: Arc<SerenityHttp>) -> Timer {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(get_env_timer())).await; // Set timer from .env
        posture_check_callout(&ctx, &http).await;
    }
}

// TODO: Move to lib/separate crate
/* HELPERS */
pub fn get_env_timer() -> u64 {
    // TODO: Read initial setting from env, register changes via application command, e.g. "/set_channel #lobby"
    // FIX: Proper Error Handling - If unparsable, safely shutdown
    let timer = env::var("TIMER").expect("missing TIMER in .env").parse::<u64>().unwrap();

    timer
}

pub fn get_env_posture_callout_channel() -> ChannelId {
    // TODO: Read initial setting from env, register changes via application command, e.g. "/set_channel #lobby"
    // FIX: Proper Error Handling - If unparsable, safely shutdown
    let channel_id = env::var("CHANNEL_ID").expect("missing CHANNEL_ID in .env").parse::<u64>().unwrap();

    ChannelId::new(channel_id)
}

pub fn get_env_guild_id() -> GuildId {
    let guild_id = env::var("GUILD_ID").expect("missing GUILD_ID in .env").parse::<u64>().unwrap();

    GuildId::new(guild_id)
}

pub fn get_env_user_threshold() -> usize {
    let user_threshold = env::var("USER_THRESHOLD").expect("missing USER_THRESHOLD in .env").parse::<usize>().unwrap();

    user_threshold
}

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

pub fn build_posture_message(ctx: &SerenityContext, guild_id: GuildId) -> String {
    let mut msg_builder = MessageBuilder::new();

    for user_id in get_all_active_voice_users(&ctx, guild_id) {
        msg_builder.mention(&user_id);
        msg_builder.push(" ");
    }

    msg_builder.push("\n\nPOSTURE CHECK RIGHT NOW");

    msg_builder.build()
}
/* END OF HELPERS */

pub async fn posture_check_callout(ctx: &SerenityContext, http: &SerenityHttp) {
    let channel_id = get_env_posture_callout_channel();
    let guild_id = get_env_guild_id();

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
        
        if active_members.len() > get_env_user_threshold() {
            return true;
        }
    }

    return false;
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