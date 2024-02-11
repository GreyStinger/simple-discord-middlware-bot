use serenity::{
    framework::standard::{ macros::command, CommandResult },
    client::Context,
    all::Message,
};

use crate::ShardManagerContainer;

pub mod slow_mode {
    use serenity::{
        all::{ Channel, ChannelId, CommandOptionType, ResolvedOption, ResolvedValue },
        builder::{ CreateCommand, CreateCommandOption, EditChannel },
        client::Context,
    };

    pub fn register() -> CreateCommand {
        CreateCommand::new("slow_mode")
            .description("Set Channel to slow mode")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Integer,
                    "Rate Limit",
                    "How long the delay should be between messages in seconds (0 is Off)"
                ).required(true)
            )
    }

    pub async fn run(
        ctx: &Context,
        channel_id: &ChannelId,
        options: &[ResolvedOption<'_>]
    ) -> String {
        // Extract the delay value directly from the options
        let delay = options
            .iter()
            .find_map(|option| {
                if option.name == "Rate Limit" {
                    if let ResolvedValue::Integer(rate) = option.value { Some(rate) } else { None }
                } else {
                    None
                }
            })
            .unwrap_or_default();

        // Fetch the channel and handle the error inline
        let channel = match channel_id.to_channel(&ctx.http).await {
            Ok(channel) => channel,
            Err(_) => {
                println!("Somehow failed to fetch channel with id: {}", channel_id);
                return "Failed to find channel that you requested to put into slow mode.".to_owned();
            }
        };

        // Handle the channel type and edit operation inline
        match channel {
            Channel::Guild(mut channel) => {
                let builder = EditChannel::new().rate_limit_per_user(delay as u16);
                match channel.edit(&ctx.http, builder).await {
                    Ok(_) =>
                        format!("Set {} to rate limit with {} second delay", channel.name(), delay),
                    Err(err) => format!("Failed to set Slow mode with reason: {:?}", err),
                }
            }
            _ =>
                "Failed to set channel to Slow Mode. Please make sure to run this in a Servers Channel".to_owned(),
        }
    }
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
