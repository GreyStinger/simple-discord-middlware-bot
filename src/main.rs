use std::collections::{ HashMap, HashSet };
use std::env;
use std::sync::Arc;

use dotenv::dotenv;

use event_handler::Handler;
use serenity::all::{ Message, UserId };

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

use songbird::{ SerenityInit, Songbird };

mod commands;
mod hooks;
mod voice_handler;
mod event_handler;

use commands::owner::{ SLOW_MODE_COMMAND, LATENCY_COMMAND };

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

pub struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

#[group]
#[owners_only]
#[summary = "Commands for server owners"]
#[only_in(guilds)]
#[commands(slow_mode, latency)]
struct Owner;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token_env_key = "TOKEN";
    let songbird = Songbird::serenity();

    let builtin_token: Option<&'static str> =
        include_optional::include_str_optional!("../token.txt");

    let token: String = match env::var(token_env_key) {
        Ok(val) => {
            println!("{}: {}", token_env_key, val);
            val
        }
        Err(_e) => {
            match builtin_token {
                Some(token) => {
                    println!(
                        "Couldn't interpret {}: {} - Using compile time token instead.",
                        token_env_key,
                        _e
                    );
                    token.to_string()
                }
                None => panic!("Expected a token in the environment or at compile time"),
            }
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
        .group(&OWNER_GROUP);

    framework.configure(Configuration::new().prefix("!").on_mention(Some(bot_id)).owners(owners));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .register_songbird_with(songbird)
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
