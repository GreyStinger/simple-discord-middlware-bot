use serenity::{
    framework::standard::{ macros::command, Args, CommandResult },
    client::Context,
    all::Message,
    builder::EditChannel,
};

use crate::ShardManagerContainer;

/// This is an asynchronous function that sets the slow mode rate for a channel.
///
/// # Arguments
///
/// * `ctx` - A reference to the context in which this command is being called.
/// * `msg` - A reference to the message that triggered this command.
/// * `args` - The arguments passed to this command.
///
/// # Description
///
/// This function sets the slow mode rate for a channel. The slow mode rate is the amount of time a user must wait before sending another message in the channel.
///
/// If a slow mode rate is provided as an argument, the function will attempt to set the channel's slow mode rate to this value. If the operation is successful, a success message is sent to the channel. If the operation fails, an error message is sent to the channel.
///
/// If no slow mode rate is provided as an argument, the function will retrieve the current slow mode rate from the channel and send a message to the channel with this value.
///
/// If the channel cannot be found in the cache, an error message is sent to the channel.
///
/// # Returns
///
/// This function returns a `CommandResult`. If the function executes successfully, it returns `Ok(())`. If the function encounters an error, it returns `Err(why)`, where `why` is the error that occurred.
///
/// # Examples
///
/// ```no_run
/// pub async fn slow_mode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
///     let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u16>() {
///         let builder = EditChannel::new().rate_limit_per_user(slow_mode_rate_seconds);
///         if let Err(why) = msg.channel_id.edit(&ctx.http, builder).await {
///             println!("Error setting channel's slow mode rate: {why:?}");
///
///             format!("Failed to set slow mode to `{slow_mode_rate_seconds}` seconds.")
///         } else {
///             format!("Successfully set slow mode rate to `{slow_mode_rate_seconds}` seconds.")
///         }
///     } else if let Some(channel) = msg.channel_id.to_channel_cached(&ctx.cache) {
///         let slow_mode_rate = channel.rate_limit_per_user.unwrap_or(0);
///         format!("Current slow mode rate is `{slow_mode_rate}` seconds.")
///     } else {
///         "Failed to find channel in cache.".to_string()
///     };
///
///     msg.channel_id.say(&ctx.http, say_content).await?;
///
///     Ok(())
/// }
/// ```
#[command]
pub async fn slow_mode(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let say_content = if let Ok(slow_mode_rate_seconds) = args.single::<u16>() {
        let builder = EditChannel::new().rate_limit_per_user(slow_mode_rate_seconds);
        if let Err(why) = msg.channel_id.edit(&ctx.http, builder).await {
            println!("Error setting channel's slow mode rate: {why:?}");

            format!("Failed to set slow mode to `{slow_mode_rate_seconds}` seconds.")
        } else {
            format!("Successfully set slow mode rate to `{slow_mode_rate_seconds}` seconds.")
        }
    } else if let Some(channel) = msg.channel_id.to_channel_cached(&ctx.cache) {
        let slow_mode_rate = channel.rate_limit_per_user.unwrap_or(0);
        format!("Current slow mode rate is `{slow_mode_rate}` seconds.")
    } else {
        "Failed to find channel in cache.".to_string()
    };

    msg.channel_id.say(&ctx.http, say_content).await?;

    Ok(())
}

#[command]
pub async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            msg.reply(ctx, "There was a problem getting the shard manager").await?;

            return Ok(());
        }
    };

    let runners = shard_manager.runners.lock().await;

    let runner = match runners.get(&ctx.shard_id) {
        Some(runner) => runner,
        None => {
            msg.reply(ctx, "No shard found").await?;

            return Ok(());
        }
    };

    msg.reply(ctx, &format!("The shard latency is {:?}", runner.latency)).await?;

    Ok(())
}
