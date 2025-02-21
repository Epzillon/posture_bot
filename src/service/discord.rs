use poise::serenity_prelude::{Context as SerenityContext, Http as SerenityHttp, GuildId, ChannelId, Member, GuildChannel, ChannelType};
use crate::service::config::{self as ConfigService, SystemConfigTrait, AppConfigTrait};
use crate::service::message as MessageService;

/// Checks whether the voice channels contains more user than the configuration-defined threshold
pub fn is_voice_active(ctx: &SerenityContext, guild_id: GuildId) -> bool {
    let active_members = get_all_active_voice_members(ctx, guild_id);

    if active_members.len() > *ConfigService::get_config().user_threshold() {
        return true;
    }

    return false;
}

/// Prepares and sends the posture check message
pub async fn posture_check_callout(ctx: &SerenityContext, http: &SerenityHttp) {
    let channel_id = ChannelId::new(*ConfigService::get_config().callout_channel_id());
    let guild_id = GuildId::new(*ConfigService::get_full_config().guild_id());

    if is_voice_active(&ctx, guild_id) {
        let message = MessageService::build_posture_message(&ctx, guild_id);
    
        if let Err(why) = channel_id.say(http, &message).await {
            println!("Error sending message: {why:?}");
        }
    }
}

/// Retrieves a list of all non-ignored members currently in voice chat
pub fn get_all_active_voice_members(ctx: &SerenityContext, guild_id: GuildId) -> Vec<Member> {
    let voice_channels = get_voice_channels(ctx, guild_id);
    let mut active_users = Vec::<Member>::new();
    let config = ConfigService::get_config();
    let ignore_list = config.ignore_list();

    for channel in voice_channels {
        for member in channel.members(ctx.cache.clone()).unwrap() {
            if !ignore_list.contains(&member.user.id.get()) {
                active_users.push(member);
            }
        }
    }

    active_users
}

/// Retrieves a list of all voice channels in the current guild
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