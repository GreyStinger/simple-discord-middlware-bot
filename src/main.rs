use std::collections::{ HashMap, HashSet };
use std::env;
use std::sync::Arc;

#[allow(unused_imports)]
use db::DatabasePool;
#[allow(unused_imports)]
use diesel::r2d2::{ ConnectionManager, Pool };
#[allow(unused_imports)]
use diesel::PgConnection;
use dotenv::dotenv;

use event_handler::Handler;

use serenity::all::UserId;
use serenity::gateway::ShardManager;
use serenity::http::Http;
use serenity::prelude::*;
use serenity::framework::standard::macros::group;
use serenity::framework::standard::{ StandardFramework, Configuration };

use songbird::{ SerenityInit, Songbird };

mod commands;
mod hooks;
mod voice_handler;
mod event_handler;
mod db;

use commands::owner::LATENCY_COMMAND;

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
#[commands(latency)]
struct Owner;

fn get_token_from_env_or_file() -> Result<String, std::env::VarError> {
    let token_env_key = "TOKEN";
    match env::var(token_env_key) {
        Ok(val) => Ok(val),
        Err(_e) => {
            let builtin_token: Option<&'static str> =
                include_optional::include_str_optional!("../token.txt");
            builtin_token.map(|token| token.to_string()).ok_or(_e)
        }
    }
}

async fn get_application_info(http: &Http) -> Result<(HashSet<UserId>, UserId), serenity::Error> {
    let info = http.get_current_application_info().await?;
    let mut owners = HashSet::new();
    if let Some(team) = info.team {
        owners.insert(team.owner_user_id);
    } else if let Some(owner) = &info.owner {
        owners.insert(owner.id);
    }
    let bot_id = http.get_current_user().await?.id;
    Ok((owners, bot_id))
}

fn setup_framework(bot_id: UserId, owners: HashSet<UserId>) -> StandardFramework {
    let framework = StandardFramework::new().before(hooks::before).group(&OWNER_GROUP);

    framework.configure(Configuration::new().prefix("!").on_mention(Some(bot_id)).owners(owners));

    framework
}

async fn setup_client(
    token: String,
    framework: StandardFramework
) -> Result<Client, serenity::Error> {
    let songbird = Songbird::serenity();
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    Client::builder(token, intents)
        .event_handler(Handler::new())
        .register_songbird_with(songbird)
        .framework(framework)
        .type_map_insert::<CommandCounter>(HashMap::default()).await
}

async fn setup_database(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let (db_url, db_user, db_pass) = (
        env::var("DATABASE_URL").unwrap_or_default(),
        env::var("DATABASE_USER").unwrap_or_default(),
        urlencoding::encode(&env::var("DATABASE_PASSWORD").unwrap_or_default()).to_string(),
    );
    let database_url = format!("postgres://{}:{}@{}", db_user, db_pass, db_url);
    let db_manager = ConnectionManager::<PgConnection>::new(database_url);
    let db_pool = Pool::builder()
        .build(db_manager)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    data.insert::<DatabasePool>(db_pool);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let token = get_token_from_env_or_file()?;
    let http = Http::new(&token);
    let (owners, bot_id) = get_application_info(&http).await?;
    let framework = setup_framework(bot_id, owners);
    let mut client = setup_client(token, framework).await?;
    setup_database(&client).await?;
    client.start().await.map_err(|why| why.into())
}
