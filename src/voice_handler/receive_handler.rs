use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use songbird::events::context_data::VoiceTick;
use tokio::sync::Mutex;

use serenity::async_trait;
use songbird::Event;
use songbird::EventContext;
use songbird::EventHandler as VoiceEventHandler;

use super::wav_manager;

static SAMPLE_RATE: u32 = 48000;

#[derive(Clone)]
pub struct ReceiveHandler {
    last_voice_packet_time: Arc<Mutex<Instant>>,
    pub voice_buffer: Arc<Mutex<Vec<i16>>>,
}

impl ReceiveHandler {
    pub fn new() -> Self {
        let new_voice_buffer: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));

        Self {
            last_voice_packet_time: Arc::new(Mutex::new(Instant::now())),
            voice_buffer: new_voice_buffer,
        }
    }

    async fn handle_voice_tick(&self, voice_tick: &VoiceTick) {
        if voice_tick.speaking.len() <= 0 {
            return;
        }
        let mut last_voice_tick_time = self.last_voice_packet_time.lock().await;

        let now = Instant::now();
        let gap_duration = now.duration_since(*last_voice_tick_time);

        let mut buffer = self.voice_buffer.lock().await;

        if gap_duration > Duration::from_millis(100) {
            let silence_samples: usize = (gap_duration.as_secs_f32() *
                (SAMPLE_RATE as f32)) as usize; // Calculate as usize
            buffer.extend(vec![0i16; silence_samples]); // Extend buffer with i16 zeros
            println!("leaving gap of {gap_duration:?}");
        }

        // Loop through each speaking user
        for (_user_id, voice_data) in &voice_tick.speaking {
            // If there is decoded voice data for the user, append it to the buffer
            if let Some(decoded_voice) = &voice_data.decoded_voice {
                buffer.extend(decoded_voice);
            }
        }

        *last_voice_tick_time = now;
    }
}

#[async_trait]
impl VoiceEventHandler for ReceiveHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        match ctx {
            EventContext::VoiceTick(packet) => {
                self.handle_voice_tick(packet).await;
                // println!("Received packet {:?}", packet);
            }
            // EventContext::RtcpPacket(_packet) => {
            //     // let mut raw_packet = packet.packet.clone();
            //     // let rtcp_decode = rtcp::packet::unmarshal(&mut raw_packet).unwrap();
            //     // println!("Ping to server: {:?}", rtcp_decode);
            //     println!("Received Rtcp packet");
            // }

            EventContext::ClientDisconnect(_event) => {
                // let buffer = self.voice_buffer.lock().await;
                // println!("current buffer: {buffer:?}");
            }
            EventContext::DriverDisconnect(_event) => {
                let buffer = self.voice_buffer.lock().await;
                wav_manager::write(buffer)
            }
            _ => {
                // We do not care about any other events in this example.
                println!("Received event {:?}", ctx);
            }
        }

        None
    }
}

pub struct ArcMutexReceiveHandler(Arc<Mutex<ReceiveHandler>>);

impl ArcMutexReceiveHandler {
    pub fn new(handler: Arc<Mutex<ReceiveHandler>>) -> Self {
        Self(handler)
    }

    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[async_trait]
impl VoiceEventHandler for ArcMutexReceiveHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let result;
        {
            let handler = self.0.lock().await;
            result = handler.act(ctx).await;
        }
        result
    }
}
