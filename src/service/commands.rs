use crate::service::config as ConfigService;

pub struct Data {} // User data, which is stored and accessible in all command invocations
type Context<'a> = poise::Context<'a, Data, Error>;
type Error = Box<dyn std::error::Error + Send + Sync>;

/// Toggles whether the bot ignores you or not during callouts.
#[poise::command(slash_command)]
pub async fn ignore_me(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let caller = ctx.author().clone();

    let is_now_ignored = ConfigService::ignore_user(caller);

    let response;
    if is_now_ignored {
        response = format!("You will no longer be recieving posture check notifications!");
    } else {
        response = format!("You will now be recieveing posture check notifications again!");
    }
    ctx.say(response).await?;
    Ok(())
}

/// Toggles whether the bot ignores you or not during callouts.
#[poise::command(slash_command)]
pub async fn ignore_status(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let caller = ctx.author().clone();
    let response: String;

    if ConfigService::is_ignored(&caller) {
        response = format!("User {}:\n\nYou are being ignored!", caller);
    } else {
        response = format!("User {}:\n\nYou are not being ignored!", caller);
    }

    ctx.say(response).await?;
    Ok(())
}