use std::thread::sleep;
use std::time::Duration;
use crate::{mic_audio, data_repo, postprocess, window};

pub fn start_recording_process(openai_api_key: String, target_device: Option<String>) {
    let mut is_recording = false;
    let mut wave_file: Option<String> = None;
    let mut recorder: Option<mic_audio::MicAudioRecorder> = None;

    let input_device = mic_audio::select_input_device_by_name(target_device);
    log::info!("\n\nReady to processing...");

    loop {
        if window::is_there_target_windows() {
            if !is_recording {
                is_recording = true;

                let output_path = match data_repo::new_wave_file_name() {
                    Ok(path) => {
                        path
                    }
                    Err(err) => {
                        log::error!("Cannot get new wave file name: {}", err);
                        continue;
                    }
                };
                let Some(output_file) = output_path.as_path()
                    .to_str() else {
                    log::error!("Cannot get wave output file name");
                    continue;
                };

                recorder = Some(mic_audio::MicAudioRecorder::new(output_file, &input_device));
                recorder.as_mut().unwrap().start_recording();
                wave_file = Some(output_file.to_string());

                log::info!("Start recording...");
            }
        } else if is_recording {
            // Window disappears, stop recording
            is_recording = false;
            recorder.as_mut().unwrap().stop_recording();
            recorder.take();  // Release the recorder if necessary
            log::info!("Stop recording...");

            let wav_file_clone = wave_file.as_ref()
                .expect("Expected wave_file to be Some; recording should stop when window disappears.")
                .clone();  // Clone the file path for the new thread
            let openai_api_key_clone = openai_api_key.clone();
            std::thread::spawn(move || {
                match postprocess::postprocess(&openai_api_key_clone, wav_file_clone.clone(), "ja") {
                    Ok(_) => {
                        log::info!("Successfully processed: {}", wav_file_clone);
                    }
                    Err(e) => {
                        log::error!("Cannot process {}: {:?}", wav_file_clone, e)
                    }
                }
            });
        }

        sleep(Duration::from_secs(1))
    }
}
