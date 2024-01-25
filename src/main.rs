use std::collections::{ HashMap, HashSet };
use std::env;
use std::sync::Arc;

use dotenv::dotenv;

use serenity::all::{ Message, UserId, Interaction, Ready };
use serenity::async_trait;
use serenity::builder::{ CreateInteractionResponseMessage, CreateInteractionResponse };
use serenity::gateway::ShardManager;
use serenity::http::Http;
use serenity::prelude::*;
use serenity::framework::standard::macros::{ group, help };
use serenity::framework::standard::{
    StandardFramework,
    Configuration,
    Args,
    HelpOptions,
    CommandGroup,
    CommandResult,
    help_commands,
};

mod commands;
mod hooks;

use commands::owner::{ SLOW_MODE_COMMAND, LATENCY_COMMAND };

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

#[help]
// This replaces the information that a user can pass a command-name as argument to gain specific
// information about it.
#[individual_command_tip = "Hello! こんにちは！Hola! Bonjour! 您好! 안녕하세요~\n\n\
If you want more information about a specific command, just pass the command as argument."]
// Some arguments require a `{}` in order to replace it with contextual information.
// In this case our `{}` refers to a command's name.
#[command_not_found_text = "Could not find: `{}`."]
// Define the maximum Levenshtein-distance between a searched command-name and commands. If the
// distance is lower than or equal the set distance, it will be displayed as a suggestion.
// Setting the distance to 0 will disable suggestions.
#[max_levenshtein_distance(3)]
// When you use sub-groups, Serenity will use the `indention_prefix` to indicate how deeply an item
// is indented. The default value is "-", it will be changed to "+".
#[indention_prefix = "+"]
// On another note, you can set up the help-menu-filter-behavior.
// Here are all possible settings shown on all possible options.
// First case is if a user lacks permissions for a command, we can hide the command.
#[lacking_permissions = "Hide"]
// If the user is nothing but lacking a certain role, we just display it.
#[lacking_role = "Nothing"]
// The last `enum`-variant is `Strike`, which ~~strikes~~ a command.
#[wrong_channel = "Strike"]
// Serenity will automatically analyze and generate a hint/tip explaining the possible cases of
// ~~strikethrough-commands~~, but only if `strikethrough_commands_tip_in_{dm, guild}` aren't
// specified. If you pass in a value, it will be displayed instead.
async fn cs_help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}

#[group]
#[owners_only]
#[summary = "Commands for server owners"]
#[only_in(guilds)]
#[commands(slow_mode, latency)]
struct Owner;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            // println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                // "ping" => Some(commands::user::ping::run(&command.data.options())),
                "id" => Some(commands::admin::id::run(&command.data.options())),
                // "attachmentinput" => Some(commands::attachmentinput::run(&command.data.options())),
                // "modal" => {
                //     commands::modal::run(&ctx, &command).await.unwrap();
                //     None
                // }
                "create_meeting" => Some(commands::user::create_meeting::run(&command.data.options())),
                "help" => {
                    // let command_data = command.data.clone();
                    // for option in &command_data.options {
                    //     match option {
                    //         CommandDataOption { name, value, .. } => {
                    //             if name == "message" {
                    //                 println!("The sent message is: {value:?}");
                    //             }
                    //         }
                    //     }
                    // }
                    // cs_help(&ctx, &command, args, help_options, groups, owners);
                    Some("Still implementing help function".to_string())
                }
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
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
            let commands = guild_id.set_commands(
                &ctx.http,
                vec![
                    // commands::user::ping::register(),
                    commands::admin::id::register(),
                    commands::help::register(),
                    commands::user::create_meeting::register(),
                ]
            ).await;

            println!("Slash commands: {commands:#?} set for guild_id: {guild_id}");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let key = "TOKEN";

    let token: String = match env::var(key) {
        Ok(val) => {
            println!("{}: {}", key, val);
            val
        }
        Err(_e) => {
            println!("Couldn't interpret {}: {}", key, _e);
            panic!("Expected a token in the environment");
        }
    };

    let http = Http::new(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();

            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else if let Some(owner) = &info.owner {
                owners.insert(owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .before(hooks::before)
        .help(&CS_HELP)
        // .group(&GENERAL_GROUP)
        .group(&OWNER_GROUP);

    framework.configure(Configuration::new().prefix("!").on_mention(Some(bot_id)).owners(owners));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default()).await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
