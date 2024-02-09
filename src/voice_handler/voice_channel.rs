use std::sync::Arc;

use serenity::all::ChannelId;
use serenity::client::Context;
use serenity::model::id::GuildId;
use songbird::{ error::JoinError, CoreEvent };
use tokio::sync::Mutex;

use crate::voice_handler::receive_handler::{ ArcMutexReceiveHandler, ReceiveHandler };

pub async fn join_voice_channel(
    ctx: &Context,
    guild_id: GuildId,
    channel_id: ChannelId
) -> Result<(), JoinError> {
    let manager = songbird
        ::get(ctx).await
        .expect("Songbird Voice client placed in at initialization.");

    let joined_config = songbird::Config
        ::default()
        .decode_mode(songbird::driver::DecodeMode::Decode);
    manager.set_config(joined_config);

    let handler = Arc::new(Mutex::new(ReceiveHandler::new()));

    let result = match manager.join(guild_id, channel_id).await {
        Ok(_) => Ok(()),
        Err(err) => {
            println!(
                "Failed to join guild with ID: {} in the channel with ID: {}",
                guild_id,
                channel_id
            );
            Err(err)
        }
    };

    if let Some(call) = songbird::get(&ctx).await.unwrap().get(guild_id) {
        println!("Found call: {call:?}");

        let arc_mutex_handler = ArcMutexReceiveHandler::new(handler.clone());

        let _ = call
            .lock().await
            .add_global_event(
                songbird::Event::Core(CoreEvent::VoiceTick),
                arc_mutex_handler.clone()
            );
        let _ = call
            .lock().await
            .add_global_event(
                songbird::Event::Core(CoreEvent::DriverDisconnect),
                arc_mutex_handler.clone()
            );
    }

    result
}

pub async fn leave_voice_channel(ctx: &Context, guild_id: GuildId) -> Result<(), JoinError> {
    let manager = songbird
        ::get(ctx).await
        .expect("Songbird Voice client placed in at initialization.");

    let joined_config = songbird::Config
        ::default()
        .decode_mode(songbird::driver::DecodeMode::Pass);
    manager.set_config(joined_config);

    manager.remove(guild_id).await
}
