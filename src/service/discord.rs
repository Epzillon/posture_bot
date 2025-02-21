use poise::serenity_prelude::{Context as SerenityContext, Http as SerenityHttp, GuildId, ChannelId, UserId, GuildChannel, ChannelType};
use crate::service::config::{self as ConfigService, SystemConfigTrait, AppConfigTrait};
use crate::service::message as MessageService;

pub async fn is_voice_active(ctx: &SerenityContext, guild_id: GuildId) -> bool {
    let voice_channels = get_voice_channels(&ctx, guild_id);

    for channel in voice_channels {
        let mut active_members = channel.members(ctx).unwrap();

        // Get active members excluding the ones in the exlude list
        active_members.retain(|member| !ConfigService::get_config().ignore_list().contains(&member.user.id.get()));
        
        if active_members.len() > *ConfigService::get_config().user_threshold() {
            return true;
        }
    }

    return false;
}

pub async fn posture_check_callout(ctx: &SerenityContext, http: &SerenityHttp) {
    let channel_id = ChannelId::new(*ConfigService::get_config().callout_channel_id());
    let guild_id = GuildId::new(*ConfigService::get_full_config().guild_id());

    if is_voice_active(&ctx, guild_id).await {
        let message = MessageService::build_posture_message(&ctx, guild_id);
    
        if let Err(why) = channel_id.say(http, &message).await {
            println!("Error sending message: {why:?}");
        }
    }
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