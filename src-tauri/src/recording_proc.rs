use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use cpal::traits::DeviceTrait;
use crate::mic_audio;
use crate::screen_audio::ScreenAudioRecorder;
use std::time::Instant;
use mic_audio::MicAudioRecorder;
use crate::entry::Entry;

#[derive(Debug)]
pub struct RecordingEvent {
    pub command: String,
    pub path: Option<String>,
}

pub struct RecordingProc {
    mic_recorder: Option<MicAudioRecorder>,
    screen_audio_recorder: Option<ScreenAudioRecorder>,
    entry: Option<Entry>,
}

impl RecordingProc {
    pub fn new() -> Self {
        RecordingProc {
            mic_recorder: None,
            screen_audio_recorder: None,
            entry: None,
        }
    }

    // 設定ファイルの再読み込みについて考えるのが面倒なので毎回読み込む。
    // ローカルファイルを読み込んで JSON を deserialize することは音声処理に比べれば誤差。
    // 富豪的プログラミングで処理する。
    fn get_target_device() -> Option<String> {
        let config = match crate::config::load_config() {
            Ok(c) => { c }
            Err(err) => {
                // TODO: show dialog?
                log::error!("Cannot load configuration: {:?}", err);
                crate::config::default_config()
            }
        };
        config.target_device
    }

    pub fn start(&mut self, path: String) -> anyhow::Result<()> {
        self.stop();

        let target_device = Self::get_target_device();
        let input_device = mic_audio::select_input_device_by_name(&target_device);

        log::info!("Starting recording...: input_device={:?} path={:?}", input_device.name(), path);

        let entry = Entry::new(PathBuf::from(path));
        let mic_wav_path = entry.mic_wav_path_string();

        // mic recording
        let mut mic_recorder = MicAudioRecorder::new(
            mic_wav_path.as_str(), &input_device
        )?;
        mic_recorder.start_recording();
        self.mic_recorder = Some(mic_recorder);

        // screen audio recorder
        let screen_audio_recorder = ScreenAudioRecorder::new(
            entry.raw_prefix_path_string()
        )?;
        if let Err(err) = screen_audio_recorder.start_recording() {
            log::error!("cannot start recording: {:?}", err);
        };
        self.screen_audio_recorder = Some(screen_audio_recorder);

        self.entry = Some(entry);

        Ok(())
    }

    pub fn stop(&mut self) -> Option<Entry> {
        if let Some(recorder) = &mut self.mic_recorder {
            recorder.stop_recording();
            self.mic_recorder.take(); // clear
        }

        // screen audio recorder
        if let Some(screen_audio_recorder) = &self.screen_audio_recorder {
            if let Err(err) = screen_audio_recorder.stop_recording() {
                log::error!("Cannot stop audio recorder: {:?}", err);
            }
            self.screen_audio_recorder.take(); // clear
        }

        self.entry.take()
    }
}

pub fn start_recording_process_ex(recording_rx: Receiver<RecordingEvent>, postprocess_tx: Sender<Entry>) {
    let mut recording_proc = RecordingProc::new();

    log::info!("Ready to processing...");

    loop {
        match recording_rx.recv() {
            Ok(event) => {
                match event.command.as_str() {
                    "START" => {
                        if let Err(err) = recording_proc.start(event.path.unwrap()) {
                            log::error!("Cannot start recording proc: {:?}", err);
                        }
                    }
                    "STOP" => {
                        let start = Instant::now();
                        if let Some(entry) = recording_proc.stop() {
                            let duration = start.elapsed();
                            // usually, under 50milli seconds.
                            log::info!("`stop` took: {:?}", duration);

                            if let Err(err) = postprocess_tx.send(entry) {
                                log::error!("Cannot start postprocess: {:?}", err);
                            }
                        }
                    }
                    _ => {
                        log::error!("Unknown message: {:?}", event);
                    }
                }
            }
            Err(err) => {
                log::error!("Cannot receive message from the channel: {:?}", err);
            }
        }
    }
}
