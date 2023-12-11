use std::thread::sleep;
use std::time::Duration;
use crate::{mic_audio, data_repo, window};
use crate::postprocess::PostProcessor;
use crate::screen_audio::ScreenAudioRecorder;
use crate::tf_idf_summarizer::TFIDFSummarizer;

pub fn start_recording_process(_openai_api_key: String, target_device: Option<String>) {
    let mut is_recording = false;
    let mut mic_recorder: Option<mic_audio::MicAudioRecorder> = None;
    let mut screen_audio_recorder: Option<ScreenAudioRecorder> = None;

    let input_device = mic_audio::select_input_device_by_name(target_device);
    log::info!("Ready to processing...");

    loop {
        if window::is_there_target_windows() {
            if !is_recording {
                is_recording = true;

                let output_path = match data_repo::new_mic_wave_file_name() {
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

                mic_recorder = Some(mic_audio::MicAudioRecorder::new(output_file, &input_device));
                mic_recorder.as_mut().unwrap().start_recording();
                screen_audio_recorder = Some(ScreenAudioRecorder::new(output_file.replace(".mic.wav", ""))
                    .unwrap());
                screen_audio_recorder.as_mut().unwrap().start_recording();

                log::info!("Start recording...");
            }
        } else if is_recording {
            // Window disappears, stop recording
            is_recording = false;
            mic_recorder.as_mut().unwrap().stop_recording();
            let mic_wave_file = mic_recorder.as_ref()
                .expect("Expected wave_file to be Some; recording should stop when window disappears.")
                .output_file
                .clone();  // Clone the file path for the new thread
            mic_recorder.take();  // Release the recorder if necessary
            screen_audio_recorder.as_mut().unwrap().stop_recording();

            log::info!("Stop recording...");

            // let openai_api_key_clone = openai_api_key.clone();
            std::thread::spawn(move || {
                let post_processor = PostProcessor::new(
                    Box::new(TFIDFSummarizer::new().expect("Cannot create instance of TFIDFSummarizer"))
                );

                match post_processor.postprocess(mic_wave_file.clone(), "ja") {
                    Ok(_) => {
                        log::info!("Successfully processed: {}", mic_wave_file);
                    }
                    Err(e) => {
                        log::error!("Cannot process {}: {:?}", mic_wave_file, e)
                    }
                }
            });
        }

        sleep(Duration::from_secs(1))
    }
}
