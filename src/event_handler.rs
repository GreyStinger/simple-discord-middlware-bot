use std::collections::HashMap;
use std::sync::Arc;

use serenity::async_trait;
use serenity::builder::{
    CreateCommand,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use serenity::all::{ CommandInteraction, Interaction, InteractionResponseFlags, Ready };
use serenity::client::{ Context, EventHandler };
use serenity::framework::standard::CommandError;

use crate::commands::{ self };

#[async_trait]
pub trait Command {
    fn name(&self) -> &'static str;
    fn register(&self) -> CreateCommand;
    async fn run(
        &self,
        ctx: &Context,
        command: &CommandInteraction
    ) -> Result<String, CommandError>;
}

pub struct Handler {
    commands: HashMap<String, Arc<dyn Command + Send + Sync>>,
}

impl Handler {
    pub fn new() -> Self {
        let mut commands: HashMap<String, Arc<dyn Command + Send + Sync>> = HashMap::new();

        commands.insert("ping".to_string(), Arc::new(commands::user::Ping));

        Self { commands }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // println!("Received interaction: {interaction:#?}");

        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let guild_id = match command.guild_id {
                Some(id) => id,
                None => {
                    println!("This command must be used in a guild.");
                    return;
                }
            };

            if let Some(cmd) = self.commands.get(&command.data.name) {
                match cmd.run(&ctx, &command).await {
                    Ok(content) => {
                        let data = CreateInteractionResponseMessage::new().content(content);
                        let builder = CreateInteractionResponse::Message(
                            data.flags(InteractionResponseFlags::EPHEMERAL)
                        );
                        if let Err(why) = command.create_response(&ctx.http, builder).await {
                            println!("Cannot respond to slash command: {why}");
                        }
                        return;
                    }
                    Err(err) => {
                        println!("Cannot respond to slash command: {err}");
                    }
                }
                // if let Err(why) = {
                // }
            }

            // if let Some(command_fn) = self.commands.get(command.data.name.as_str()) {
            //     let result = command_fn(&ctx, &command).await;
            //     match result {
            //         Ok(content) => {
            //             let data = CreateInteractionResponseMessage::new().content(content);
            //             let builder = CreateInteractionResponse::Message(
            //                 data.flags(InteractionResponseFlags::EPHEMERAL)
            //             );
            //             if let Err(why) = command.create_response(&ctx.http, builder).await {
            //                 println!("Cannot respond to slash command: {why}");
            //             }
            //         }
            //         Err(_) => {
            //             let data = CreateInteractionResponseMessage::new().content(
            //                 "not implemented :(".to_string()
            //             );
            //             let builder = CreateInteractionResponse::Message(
            //                 data.flags(InteractionResponseFlags::EPHEMERAL)
            //             );
            //             if let Err(why) = command.create_response(&ctx.http, builder).await {
            //                 println!("Cannot respond to slash command: {why}");
            //             }
            //         }
            //     }
            // }

            let content = match command.data.name.as_str() {
                // "ping" => Some(commands::user::ping::run(&ctx, &command)),
                "id" => {
                    let result = commands::user::id::run(
                        &ctx,
                        guild_id,
                        &command.data.options()
                    ).await;
                    Some(result)
                }
                "join_channel" => {
                    let result = commands::driver_control::join_channel::run(
                        &ctx,
                        guild_id,
                        &command.data.options()
                    ).await;
                    Some(result)
                }
                "leave_channel" => {
                    let result = commands::driver_control::leave_channel::run(
                        &ctx,
                        guild_id,
                        &command.data.options()
                    ).await;

                    Some(result)
                }
                "create_meeting" =>
                    Some(commands::user::create_meeting::run(&command.data.options())),
                "get_mem_usage" => Some(commands::user::get_mem_usage::run()),
                "slow_mode" =>
                    Some(
                        commands::owner::slow_mode::run(
                            &ctx,
                            &command.channel_id,
                            &command.data.options()
                        ).await
                    ),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(
                    data.flags(InteractionResponseFlags::EPHEMERAL)
                );
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guilds_cache = ctx.cache.guilds();

        let mut commands_to_register: Vec<CreateCommand> = Vec::new();
        commands_to_register.append(&mut commands::register_user::register());
        commands_to_register.append(&mut commands::register_owner::register());
        commands_to_register.append(&mut commands::register_driver_control::register());

        for guild_id in guilds_cache {
            let _commands = guild_id.set_commands(&ctx.http, commands_to_register.clone()).await;
        }
    }
}
