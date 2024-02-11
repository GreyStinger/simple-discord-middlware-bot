use std::{ env, fs, io::BufWriter };

use hound::WavWriter;
use tokio::sync::MutexGuard;

pub fn write(buffer: MutexGuard<'_, Vec<i16>>) {
    let default_sample_rate = 48000;
    let sample_rate: u32 = env
        ::var("SAMPLE_RATE")
        .unwrap_or_else(|_| default_sample_rate.to_string())
        .parse()
        .unwrap_or(default_sample_rate);

    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let now = chrono::prelude::Utc::now();
    let dir = "recordings";
    fs::create_dir_all(dir).expect("Failed to create directory");

    let filename = format!("{}/recording_{}.wav", dir, now.format("%Y%m%d%H%M%S"));
    let file = fs::File::create(&filename).expect("Failed to create file");
    let mut writer = WavWriter::new(BufWriter::new(file), spec).expect(
        "Failed to create WavWriter"
    );

    let mut sample_count = 0;
    for &sample in &*buffer {
        if let Err(err) = writer.write_sample(sample) {
            eprintln!("Encountered error: {:?}", err);
        }
        sample_count += 1;
    }

    // If the number of samples is not a multiple of the number of channels, pad with zeros
    while sample_count % (spec.channels as usize) != 0 {
        writer.write_sample(0).expect("Failed to write padding sample");
        sample_count += 1;
    }

    if let Err(err) = writer.finalize() {
        eprintln!("Failed to finalize WavWriter: {:?}", err);
    } else {
        println!("Saved buffer to {}", &filename);
    }
}
