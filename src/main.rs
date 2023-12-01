use std::thread::sleep;
use std::time::Duration;
use chrono::Local;
use cpal::traits::{DeviceTrait, HostTrait}; // for timestamp

mod audio;
mod window;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[clap(long)] // , about = "The target input device")
    target_device: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    // #1 Check if the platform is supported
    let supported = MeetNote2::is_supported();
    if !supported {
        println!("❌ Platform not supported");
        // TODO: use GUI dialog?
        return;
    } else {
        println!("✅ Platform supported");
    }

    // #2 Check if we have permission to capture the screen
    let has_permission = MeetNote2::has_permission();
    if !has_permission {
        println!("❌ Permission not granted");
        return;
    } else {
        println!("✅ Permission granted");
    }

    let mut is_recording = false;
    let mut recorder: Option<audio::AudioRecorder> = None;

    let input_device = audio::select_input_device_by_name(opts.target_device);
    println!("\n\nReady to processing...");

    loop {
        if window::is_there_target_windows() {
            if !is_recording {
                is_recording = true;

                let timestamp = Local::now().format("%Y%m%d-%H%M%S").to_string();
                let output_file = format!("test/audio/{}.wav", timestamp);

                recorder = Some(audio::AudioRecorder::new(&output_file, &input_device));
                recorder.as_mut().unwrap().start_recording();
                println!("Start recording...");
            }
        } else {
            if is_recording {
                // Window disappears, stop recording
                is_recording = false;
                recorder.as_mut().unwrap().stop_recording();
                recorder.take();  // Release the recorder if necessary
                println!("Stop recording...");

                // TODO send file name to post-processing thread
            }
        }

        sleep(Duration::from_secs(1))
    }
}
