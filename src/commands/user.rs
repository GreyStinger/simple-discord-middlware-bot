use greys_macros::slash_command;
use serenity::{
    all::CommandInteraction,
    async_trait,
    builder::CreateCommand,
    client::Context,
    framework::standard::CommandError,
};

use crate::event_handler::Command;

pub mod id {
    use serenity::all::GuildId;
    use serenity::builder::{ CreateCommand, CreateCommandOption };
    use serenity::model::application::{ CommandOptionType, ResolvedOption, ResolvedValue };
    use serenity::model::id::RoleId;
    use serenity::prelude::Context;

    pub fn register() -> CreateCommand {
        CreateCommand::new("id")
            .description("Get user ids")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Role,
                    "role",
                    "The role to lookup"
                ).required(true)
            )
    }

    pub async fn run(ctx: &Context, guild_id: GuildId, options: &[ResolvedOption<'_>]) -> String {
        let mut response = String::new();

        for option in options {
            if let ResolvedOption { value: ResolvedValue::Role(role), .. } = option {
                let guild_id = guild_id;
                let members = guild_id.members(&ctx.http, None, None).await.unwrap();

                let role_id = RoleId::new(role.id.get());
                for member in members {
                    if member.roles.contains(&role_id) {
                        response.push_str(
                            &format!("{}'s id is {}\n", member.user.tag(), member.user.id)
                        );
                    }
                }
            }
        }

        if response.is_empty() {
            return "Please provide a valid role".to_string();
        }

        response
    }
}

// #[slash_command]
// #[description("It Pings")]

pub struct Ping;

#[async_trait]
impl Command for Ping {
    fn name(&self) -> &'static str {
        "ping"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name()).description("It Pongs!")
    }

    async fn run(
        &self,
        _ctx: &Context,
        _command: &CommandInteraction
    ) -> Result<String, CommandError> {
        Ok("Pong!".to_owned())
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
