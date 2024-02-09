use std::{ env, fs::File, io::BufWriter };

use hound::WavWriter;
use tokio::sync::MutexGuard;

pub fn write(buffer: MutexGuard<'_, Vec<i16>>) {
    let default_sample_rate = 48000;
    let sample_rate: u32 = env
        ::var("SAMPLE_RATE")
        .map(|val| val.parse().unwrap_or(default_sample_rate))
        .unwrap_or(default_sample_rate);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let now = chrono::prelude::Utc::now();
    let filename = format!("recording_{}.wav", now.format("%Y%m%d%H%M%S"));
    let mut writer = WavWriter::new(
        BufWriter::new(File::create(&filename).unwrap()),
        spec
    ).unwrap();

    for &sample in &*buffer {
        if let Err(err) = writer.write_sample(sample) {
            println!("Encountered error: {err:?}");
        };
    }

    writer.finalize().unwrap();

    println!("Saved buffer to {}", &filename);
}
