use greys_macros::slash_command;
// use serenity::all::{ResolvedValue, CommandOptionType};

// pub mod ping {
//     use serenity::{ all::ResolvedOption, builder::CreateCommand };

//     pub fn register() -> CreateCommand {
//         CreateCommand::new("ping").description("It pings!")
//     }

//     pub fn run(_options: &[ResolvedOption]) -> String {
//         "Pong!".to_owned()
//     }
// }

// #[description("It Pings")]
#[slash_command]
#[description("It Pings")]
mod ping {
    use serenity::all::ResolvedOption;

    pub fn ping(_options: &[ResolvedOption]) -> String {
        "Pong!".to_owned()
    }
}

#[slash_command]
#[description("Creates a meeting room for and notifies the requested users of said room")]
mod create_meeting {
    use serenity::all::{ ResolvedValue, ResolvedOption };
    use serenity::all::CommandOptionType;

    pub fn run(options: &[ResolvedOption]) -> String {
        for option in options {
            if
                let ResolvedOption {
                    value: ResolvedValue::Autocomplete {
                        kind: CommandOptionType::SubCommand,
                        value: "Users",
                    },
                    name: "id",
                    ..
                } = option
            {
            }
        }

        String::from("Ran create_meeting successfully")
    }
}

pub mod join_channel {
    use serenity::all::{ Channel, ChannelType, CommandOptionType, ResolvedOption, ResolvedValue };
    use serenity::all::GuildId;
    use serenity::builder::{ CreateCommand, CreateCommandOption };
    use serenity::prelude::Context;
    use serenity::model::id::ChannelId;
    use crate::commands::voice_client_control::voice_channel::join_voice_channel;

    pub fn register() -> CreateCommand {
        CreateCommand::new("join_channel")
            .description("Simply makes the bot join a channel")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Channel,
                    "channel",
                    "The channel to join"
                ).required(true)
            )
    }

    pub async fn run(ctx: &Context, guild_id: GuildId, options: &[ResolvedOption<'_>]) -> String {
        let channel_id = options.iter().find_map(|option| {
            if let ResolvedOption { value: ResolvedValue::Channel(channel), .. } = option {
                Some(ChannelId::new(channel.id.get()))
            } else {
                None
            }
        });

        match channel_id {
            Some(channel_id) => {
                // Fetch the channel to check if it's a voice channel
                match channel_id.to_channel(ctx).await {
                    Ok(Channel::Guild(channel)) if channel.kind == ChannelType::Voice => {
                        match join_voice_channel(ctx, guild_id, channel_id).await {
                            Ok(_) => "Successfully joined voice channel".to_string(),
                            Err(e) => format!("Failed to join voice channel: {}", e),
                        }
                    }
                    _ => "Please provide a valid voice channel".to_string(),
                }
            }
            None => "Please provide a valid channel".to_string(),
        }
    }
}

#[slash_command]
#[description("Simply makes the bot leave a channel")]
mod leave_channel {
    use serenity::all::ResolvedOption;
    use serenity::all::GuildId;
    use serenity::prelude::Context;
    use crate::commands::voice_client_control::voice_channel::leave_voice_channel;

    pub async fn run(ctx: &Context, guild_id: GuildId, _options: &[ResolvedOption<'_>]) -> String {
        if let Err(e) = leave_voice_channel(ctx, guild_id).await {
            format!("Failed to leave voice channel: {}", e)
        } else {
            "Successfully left voice channel".to_string()
        }
    }
}

pub mod record_voice {
    use serenity::{ all::CommandOptionType, builder::{ CreateCommand, CreateCommandOption } };

    pub fn _register() -> CreateCommand {
        CreateCommand::new("record_voice")
            .description("Makes the bot record voice data until commanded to stop")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Boolean,
                    "record",
                    "Whether the bot should be recording"
                ).required(true)
            )
    }

    pub fn _run() -> String {
        "in development".to_owned()
    }
}

#[slash_command]
#[description("Prints out how much memory the server is using")]
mod get_mem_usage {
    use memory_stats::memory_stats;

    pub fn run() -> String {
        if let Some(usage) = memory_stats() {
            format!(
                "Current physical memory usage: {} MiB\nCurrent virtual memory usage: {} MiB",
                usage.physical_mem / 1024 / 1024,
                usage.virtual_mem / 1024 / 1024
            )
        } else {
            "Couldn't get the current memory usage :(".to_owned()
        }
    }
}
