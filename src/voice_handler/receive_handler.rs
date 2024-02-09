use std::io::BufWriter;
use std::sync::Arc;
// use std::time::Duration;
// use std::time::Instant;
use hound::WavWriter;
// use songbird::events::context_data::RtpData;
use songbird::events::context_data::VoiceTick;
// use syn::buffer;
use tokio::sync::Mutex;
use std::fs::File;

use serenity::async_trait;
use songbird::Event;
use songbird::EventContext;
use songbird::EventHandler as VoiceEventHandler;

static SAMPLE_RATE: u32 = 48000;

#[derive(Clone)]
pub struct ReceiveHandler {
    // _last_raw_packet_time: Arc<Mutex<Instant>>,
    // pub raw_buffer: Arc<Mutex<Vec<u8>>>,
    pub voice_buffer: Arc<Mutex<Vec<i16>>>,
}

impl ReceiveHandler {
    pub fn new() -> Self {
        // let new_raw_buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(Vec::new()));
        let new_voice_buffer: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));

        Self {
            // _last_raw_packet_time: Arc::new(Mutex::new(Instant::now())),
            // raw_buffer: new_raw_buffer,
            voice_buffer: new_voice_buffer,
        }
    }

    // async fn handle_rtp_packet(&self, packet: &RtpData) {
    //     let mut last_packet_time = self._last_raw_packet_time.lock().await;

    //     let now = Instant::now();
    //     let gap_duration = now.duration_since(*last_packet_time);

    //     if let Ok(rtp) = rtp_rs::RtpReader::new(&packet.packet) {
    //         println!("Sequence number {:?}", rtp.sequence_number());
    //         println!("Payload length {:?}", rtp.payload().len());
    //     }

    //     let mut buffer = self.raw_buffer.lock().await;

    //     // If the gap is more than 20ms, insert silence
    //     if gap_duration > Duration::from_millis(20) {
    //         let silence_samples = (gap_duration.as_secs_f32() * SAMPLE_RATE as f32) as usize; // Honestly I don't know how to calc this rn
    //         buffer.extend(vec![0; silence_samples]);
    //     }

    //     // Append the packet data to the buffer

    //     buffer.extend_from_slice(&packet.packet);

    //     *last_packet_time = now;
    // }

    async fn handle_voice_tick(&self, voice_tick: &VoiceTick) {
        let mut buffer = self.voice_buffer.lock().await;
    
        // Loop through each speaking user
        for (_user_id, voice_data) in &voice_tick.speaking {
            // If there is decoded voice data for the user, append it to the buffer
            if let Some(decoded_voice) = &voice_data.decoded_voice {
                buffer.extend(decoded_voice);
            }
        }
    
        // *last_packet_time = now;
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
            // EventContext::RtpPacket(packet) => {
            //     // Raw packet handler
            //     self.handle_rtp_packet(packet).await;
            //     println!("Received and stored RTP packet");
            // }
            // EventContext::RtcpPacket(_packet) => {
            //     // let mut raw_packet = packet.packet.clone();
            //     // let rtcp_decode = rtcp::packet::unmarshal(&mut raw_packet).unwrap();
            //     // println!("Ping to server: {:?}", rtcp_decode);
            //     println!("Received Rtcp packet");
            // }
            // EventContext::SpeakingStateUpdate(speaking) => {
            //     println!("User {:?} {:?} speaking", speaking.ssrc, speaking.speaking);
            // }

            EventContext::ClientDisconnect(_event) => {
                // let buffer = self.voice_buffer.lock().await;
                // println!("current buffer: {buffer:?}");
            }
            EventContext::DriverDisconnect(_event) => {
                let buffer = self.voice_buffer.lock().await;
                // println!("current buffers: {buffer:?}");

                // Save the buffer to an audio file
                let spec = hound::WavSpec {
                    channels: 2,
                    sample_rate: SAMPLE_RATE,
                    bits_per_sample: 16,
                    sample_format: hound::SampleFormat::Int
                };
                let mut writer = WavWriter::new(
                    BufWriter::new(File::create("output.wav").unwrap()),
                    spec
                ).unwrap();

                for &sample in &*buffer {
                    writer.write_sample(sample).unwrap();
                }

                writer.finalize().unwrap();

                // let buffer = self.raw_buffer.lock().await;
                // // Save the buffer to an audio file
                // let spec = hound::WavSpec {
                //     channels: 2,
                //     sample_rate: SAMPLE_RATE,
                //     bits_per_sample: 16,
                //     sample_format: hound::SampleFormat::Int
                // };
                // let mut writer = WavWriter::new(
                //     BufWriter::new(File::create("output-raw.wav").unwrap()),
                //     spec
                // ).unwrap();

                // for &sample in &*buffer {
                //     writer.write_sample(sample as i16).unwrap();
                // }

                // writer.finalize().unwrap();
                println!("Saved buffer to output.wav");
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
