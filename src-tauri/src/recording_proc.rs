use std::thread::sleep;
use std::time::Duration;
use cpal::traits::DeviceTrait;
use lazy_static::lazy_static;
use crate::{mic_audio, window};
use crate::config::load_config_or_default;
use crate::postprocess::PostProcessor;
use crate::screen_audio::ScreenAudioRecorder;
use std::sync::RwLock;
use std::thread;
use mic_audio::MicAudioRecorder;
use crate::data_repo::DataRepo;
use crate::entry::Entry;

lazy_static! {
    static ref IS_RECORDING: RwLock<bool> = RwLock::new(false);
}

pub struct RecordingProc {
    mic_recorder: Option<MicAudioRecorder>,
    screen_audio_recorder: Option<ScreenAudioRecorder>,
    entry: Option<Entry>,
    data_repo: DataRepo,
}

impl RecordingProc {
    pub fn new(data_repo: DataRepo) -> Self {
        RecordingProc {
            mic_recorder: None,
            screen_audio_recorder: None,
            entry: None,
            data_repo,
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
        return config.target_device;
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.stop();

        let target_device = Self::get_target_device();
        let input_device = mic_audio::select_input_device_by_name(&target_device);

        log::info!("Starting recording...: input_device={:?}", input_device.name());

        let entry = self.data_repo.new_entry()?;
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

        // scrren audio recorder
        if let Some(screen_audio_recorder) = &self.screen_audio_recorder {
            if let Err(err) = screen_audio_recorder.stop_recording() {
                log::error!("Cannot stop audio recorder: {:?}", err);
            }
            self.screen_audio_recorder.take(); // clear
        }

        return self.entry.take();
    }
}

pub fn is_recording() -> bool {
    *IS_RECORDING.read().unwrap()
}

pub fn start_recording_process() {
    let datarepo = DataRepo::new()
        .expect("DataRepo::new");
    let mut recording_proc = RecordingProc::new(datarepo);

    log::info!("Ready to processing...");

    loop {
        if let Some(info) = window::is_there_target_windows() {
            if !(*IS_RECORDING.read().unwrap()) {
                log::info!("Starting recording: {:?}", info);

                if let Err(err) = recording_proc.start() {
                    log::error!("Cannot start recording proc: {:?}", err);
                    sleep(Duration::from_secs(1));
                }

                *(IS_RECORDING.write().unwrap()) = true;
            }
        } else if *IS_RECORDING.read().unwrap() {
            // Window disappears, stop recording
            *(IS_RECORDING.write().unwrap()) = false;

            log::info!("Stop recording...");

            if let Some(entry) = recording_proc.stop() {
                thread::spawn(move || {
                    start_postprocess(entry.mic_wav_path_string());
                });
            }
        }

        sleep(Duration::from_secs(1))
    }
}

pub fn start_postprocess(mic_wav_path: String) {
    let config = load_config_or_default();
    let summarizer = config.build_summarizer()
        .unwrap();
    let post_processor = PostProcessor::new(summarizer);

    match post_processor.postprocess(mic_wav_path.clone(), config) {
        Ok(_) => {
            log::info!("Successfully processed: {}", mic_wav_path);
        }
        Err(e) => {
            log::error!("Cannot process {}: {:?}", mic_wav_path, e)
        }
    }
}
