use std::thread::sleep;
use std::time::Duration;
use cpal::traits::DeviceTrait;
use lazy_static::lazy_static;
use crate::{mic_audio, window};
use crate::config::{load_config_or_default, MeetNoteConfig};
use crate::postprocess::PostProcessor;
use crate::screen_audio::ScreenAudioRecorder;
use std::sync::RwLock;
use crate::data_repo::DataRepo;

lazy_static! {
    static ref IS_RECORDING: RwLock<bool> = RwLock::new(false);
}

pub fn is_recording() -> bool {
    *IS_RECORDING.read().unwrap()
}

pub fn start_recording_process(config: MeetNoteConfig) {
    let mut mic_recorder: Option<mic_audio::MicAudioRecorder> = None;
    let mut screen_audio_recorder: Option<ScreenAudioRecorder> = None;
    let target_device = config.target_device;

    let datarepo = DataRepo::new()
        .expect("DataRepo::new");

    log::info!("Ready to processing...");

    loop {
        if let Some(info) = window::is_there_target_windows() {
            if !(*IS_RECORDING.read().unwrap()) {
                let input_device = mic_audio::select_input_device_by_name(&target_device);

                // TODO このへん、ちゃんと struct で状態管理したほうがいいかも。。エラー処理が冗長になりすぎ

                log::info!("Starting recording...: window={:?} input_device={:?}", info, input_device.name());

                let entry = match datarepo.new_entry() {
                    Ok(entry) => {entry}
                    Err(err) => {
                        log::error!("Cannot create entry: {:?}", err);
                        continue;
                    }
                };
                let mic_output_path = entry.mic_wav_path();
                let Some(output_file) = mic_output_path.as_path().to_str() else {
                    log::error!("Cannot get wave output file name");
                    continue;
                };

                match mic_audio::MicAudioRecorder::new(output_file, &input_device) {
                    Ok(mut recorder) => {
                        recorder.start_recording();
                        mic_recorder = Some(recorder)
                    }
                    Err(err) => {
                        log::error!("Cannot start mic recording: {:?}", err);
                        sleep(Duration::from_secs(1));
                        continue;
                    }
                };
                screen_audio_recorder = Some(ScreenAudioRecorder::new(output_file.replace(".mic.wav", ""))
                    .unwrap());
                if let Err(err) = screen_audio_recorder.as_mut().unwrap().start_recording() {
                    log::error!("cannot start recording: {:?}", err);
                }
                *(IS_RECORDING.write().unwrap()) = true;
            }
        } else if *IS_RECORDING.read().unwrap() {
            // Window disappears, stop recording
            *(IS_RECORDING.write().unwrap()) = false;
            if let Some(recorder) = &mut mic_recorder {
                recorder.stop_recording();
            }
            let mic_wave_file = mic_recorder.as_ref()
                .expect("Expected wave_file to be Some; recording should stop when window disappears.")
                .output_file
                .clone();  // Clone the file path for the new thread
            mic_recorder.take();  // Release the recorder if necessary
            if let Err(err) = screen_audio_recorder.as_mut().unwrap()
                .stop_recording() {
                log::error!("Cannot stop audio recorder: {:?}", err)
            }

            log::info!("Stop recording...");

            // let openai_api_key_clone = openai_api_key.clone();
            std::thread::spawn(move || {
                start_postprocess(mic_wave_file);
            });
        }

        sleep(Duration::from_secs(1))
    }
}

pub fn start_postprocess(mic_wave_file: String) {
    let config = load_config_or_default();
    let summarizer = config.build_summarizer()
        .unwrap();
    let post_processor = PostProcessor::new(summarizer);

    match post_processor.postprocess(mic_wave_file.clone(), config) {
        Ok(_) => {
            log::info!("Successfully processed: {}", mic_wave_file);
        }
        Err(e) => {
            log::error!("Cannot process {}: {:?}", mic_wave_file, e)
        }
    }
}
