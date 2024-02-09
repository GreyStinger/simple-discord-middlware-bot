use serenity::async_trait;
use serenity::builder::{ CreateInteractionResponseMessage, CreateInteractionResponse };

use serenity::all::{ Interaction, InteractionResponseFlags, Ready };
use serenity::client::{ Context, EventHandler };

use crate::commands::{ self };

pub struct Handler;

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

            let content = match command.data.name.as_str() {
                "ping" => Some(commands::user::ping::run(&command.data.options())),
                "id" => {
                    let result = commands::user::id::run(
                        &ctx,
                        guild_id,
                        &command.data.options()
                    ).await;
                    Some(result)
                }
                "join_channel" => {
                    let result = commands::user::join_channel::run(
                        &ctx,
                        guild_id,
                        &command.data.options()
                    ).await;
                    Some(result)
                }
                "leave_channel" => {
                    let result = commands::user::leave_channel::run(
                        &ctx,
                        guild_id,
                        &command.data.options()
                    ).await;

                    Some(result)
                }
                "create_meeting" =>
                    Some(commands::user::create_meeting::run(&command.data.options())),
                // "help" => {
                //     // .unwrap_or("Failed to fetch help".to_owned())
                // }
                "get_mem_usage" => Some(commands::user::get_mem_usage::run()),
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

        for guild_id in guilds_cache {
            let _commands = guild_id.set_commands(
                &ctx.http,
                vec![
                    commands::user::ping::register(),
                    commands::user::id::register(),
                    commands::user::create_meeting::register(),
                    commands::user::join_channel::register(),
                    commands::user::leave_channel::register(),
                    commands::user::get_mem_usage::register()
                ]
            ).await;
        }
    }
}
