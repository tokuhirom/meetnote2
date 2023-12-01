mod audio;
mod window;
mod mp3;
mod openai;
mod postprocess;
mod recording_proc;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[clap(long)] // , about = "The target input device")
    target_device: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    let openai_api_key = std::env::var("OPENAI_API_KEY")
        .expect("Expected environment variable: OPENAI_API_KEY");

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

    recording_proc::start_recording_process(openai_api_key, opts.target_device)
}
