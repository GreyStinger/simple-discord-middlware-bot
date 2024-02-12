use greys_macros::slash_command;
use serenity::{
    all::CommandInteraction,
    async_trait,
    builder::{ CreateCommand, CreateCommandOption },
    client::Context,
    framework::standard::CommandError,
};
use serenity::model::application::{ CommandOptionType, ResolvedOption, ResolvedValue };
use serenity::model::id::RoleId;

use crate::event_handler::Command;

pub struct Id;

#[async_trait]
impl Command for Id {
    fn name(&self) -> &'static str {
        "id"
    }

    fn register(&self) -> CreateCommand {
        CreateCommand::new(self.name())
            .description("Get user ids")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Role,
                    "role",
                    "The role to lookup"
                ).required(true)
            )
    }

    async fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction
    ) -> Result<String, CommandError> {
        let mut response = String::new();

        let guild_id = match command.guild_id {
            Some(id) => id,
            None => {
                return Ok("This command must be used in a guild.".to_owned());
            }
        };

        for option in command.data.options() {
            if let ResolvedOption { value: ResolvedValue::Role(role), .. } = option {
                let guild_id = guild_id;
                let members = guild_id.members(&ctx.http, None, None).await?;

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
            return Err(CommandError::from("Please provide a valid role"));
        }

        Ok(response)
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
