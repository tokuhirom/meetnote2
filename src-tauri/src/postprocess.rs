use std::fs;
use std::process::Command;
use std::sync::mpsc::Receiver;
use crate::mp3;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use crate::config::{load_config_or_default, MeetNoteConfig, TranscriberType};
use crate::openai::OpenAICustomizedClient;
use crate::openai_transcriber::OpenAITranscriber;
use crate::summarizer::Summarizer;
use crate::transcriber::Transcriber;
use crate::whisper_cpp::WhisperTranscriber;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};
use crate::entry::Entry;

#[derive(Clone, Serialize, Deserialize)]
pub struct PostProcessStatus {
    basename: String,
    message: String,
}

lazy_static! {
    static ref POSTPROCEDSS_STATE : RwLock<PostProcessStatus> = RwLock::new(PostProcessStatus {
        basename: "".to_string(),
        message: "".to_string(),
    });
}

pub fn postprocess_status() -> PostProcessStatus {
    POSTPROCEDSS_STATE.read().unwrap().clone()
}

pub struct PostProcessor {
    summarizer: Box<dyn Summarizer>,
}

impl PostProcessor {
    pub fn new(summarizer: Box<dyn Summarizer>) -> Box<PostProcessor> {
        Box::new(PostProcessor { summarizer })
    }

    pub fn postprocess(&self, entry: Entry, config: MeetNoteConfig) -> Result<()> {
        let basename = entry.basename.clone();

        *POSTPROCEDSS_STATE.write().unwrap() = PostProcessStatus {
            basename: basename.to_string(),
            message: "Starting postprocess".to_string(),
        };

        let result = self.do_postprocess(entry, config);

        // clear the status
        *POSTPROCEDSS_STATE.write().unwrap() = PostProcessStatus {
            basename: "".to_string(),
            message: "".to_string(),
        };

        result
    }

    fn do_postprocess(&self, entry: Entry, config: MeetNoteConfig) -> Result<()>{
        let basename = entry.basename.clone();

        let set_post = |message: &str| {
            *POSTPROCEDSS_STATE.write().unwrap() = PostProcessStatus {
                basename: basename.to_string(),
                message: message.to_string(),
            };
        };

        set_post("Merging wave files");
        let merged_wav_file = merge_audio_files(&entry)?;

        // convert to MP3
        set_post("Convert to MP3");
        let mp3_file = entry.mp3_path_string();
        if let Err(e) = mp3::convert_to_mp3(&merged_wav_file, &mp3_file) {
            return Err(anyhow!("Cannot convert to mp3({} to {}): {:?}", merged_wav_file, mp3_file, e))
        }

        // convert to VTT
        set_post("Transcribing");
        let vtt_file = entry.webvtt_path_string();
        self.transcribe(&config, &merged_wav_file, &vtt_file)?;

        // Summarize VTT
        set_post("Summarizing");
        let summary_file = entry.md_path();
        self.summarize(vtt_file.as_str(), summary_file.as_str())?;

        // cleanup files
        set_post("Cleanup");
        self.cleanup(&entry)?;

        Ok(())
    }

    pub fn cleanup(&self, entry: &Entry) -> anyhow::Result<()> {
        file_remove(entry.merged_wav_path_string().as_str())?;
        file_remove(entry.mic_wav_path_string().as_str())?;
        for path in entry.list_raw_files()? {
            file_remove(path.unwrap().to_str().unwrap())?;
        }
        Ok(())
    }

    pub fn transcribe(&self, config: &MeetNoteConfig, wav_file: &String, vtt_file: &String) -> anyhow::Result<()> {
        log::info!("Convert {} to {}", wav_file, vtt_file);

        let transcriber: Box<dyn Transcriber> = match config.transcriber_type {
            TranscriberType::WhisperCppTranscriberType => {
                Box::new(WhisperTranscriber::new(
                    "v1.5.1".to_string(), config.whisper_model.to_string(), config.language.to_string(),
                ))
            }
            TranscriberType::OpenAITranscriberType => {
                let token = match &config.openai_api_token {
                    Some(token) => { token }
                    None => {
                        return Err(anyhow!("OpenAI transcriber requires OpenAI token. But it's missing."));
                    }
                };
                let openai = match OpenAICustomizedClient::new(
                    token.as_str()
                ) {
                    Ok(openai) => { openai }
                    Err(err) => {
                        return Err(anyhow!("Cannot create openai client: {:?}", err))
                    }
                };

                Box::new(OpenAITranscriber::new(
                    openai,
                    config.language.to_string(),
                ))
            }
        };
        match transcriber.transcribe(wav_file, vtt_file) {
            Ok(_) => {
                log::info!("Wrote transcript to \"{}\"", vtt_file);
            }
            Err(e) => {
                return Err(anyhow!("Cannot transcribe from wave file: {:?}, {:?}", wav_file, e))
            }
        }

        Ok(())
    }

    pub fn summarize(&self, vtt_file: &str, summary_file: &str) -> anyhow::Result<()> {
        let vtt_result = fs::read_to_string(vtt_file);
        let Ok(vtt_content) = vtt_result else {
            return Err(anyhow!("Cannot read VTT file({}): {:?}",
                vtt_file,
                vtt_result
            ))
        };
        log::info!("Requesting summarization: \"{}\"", vtt_file);

        let summary = self.summarizer.summarize(vtt_content.as_str())
            .map_err(|err| { anyhow!("Cannot postprocess summarization process {:?}: {:?}", vtt_file, err)})?;

        if let Err(e) = fs::write(summary_file, summary) {
            return Err(anyhow!("Cannot write to file({}): {:?}",
                    summary_file, e))
        }

        log::info!("Finished summarization: \"{}\" to \"{}\"", vtt_file, summary_file);

        Ok(())
    }
}


fn file_remove(filename: &str) -> anyhow::Result<()> {
    match fs::remove_file(filename) {
        Ok(_) => {
            log::info!("Removed {:?}", filename);
            Ok(())
        }
        Err(err) => {
            Err(anyhow!("Cannot remove {:?}: {:?}", filename, err))
        }
    }
}

fn merge_audio_files(entry: &Entry) -> anyhow::Result<String> {
    let mic_wav_file = entry.mic_wav_path_string();
    let output_wave_file = entry.merged_wav_path_string();

    // merge raw files to 1 wav file
    let screen_tmp = tempfile::Builder::new()
        .suffix(".wav")
        .rand_bytes(5)
        .tempfile()
        .unwrap();
    {
        let raw_files = entry.list_raw_files()?;
        // log::info!("Processing raw files: {:?}", raw_files);

        let mut path_count = 0;

        let mut command = Command::new("sox");
        for x in raw_files {
            // TODO more flexible format support...
            command
                .arg("-t").arg("raw")
                .arg("-r").arg("48000")
                .arg("-e").arg("floating-point")
                .arg("-b").arg("32")
                .arg("-c").arg("1")
                .arg("--endian").arg("little");
            command.arg(x.unwrap().to_str().unwrap());
            path_count += 1;
        }
        if path_count == 0 {
            return Err(anyhow!("Missing raw files for {:?}", mic_wav_file))
        }

        command.arg(screen_tmp.path().to_str().unwrap());
        command.arg("norm");
        log::info!("Merge & normalize raw file: {:?}", command);
        let output = command.output()?;
        if !output.status.success() {
            return Err(anyhow!("Cannot run sox: {:?}: {}", command, String::from_utf8_lossy(&output.stderr)));
        }
    }

    // normalize mic.wav file.
    let mic_wav_tmp = tempfile::Builder::new()
        .suffix(".wav")
        .rand_bytes(5)
        .tempfile()
        .unwrap();
    {
        let mut command = Command::new("sox");
        command
            .arg(mic_wav_file)
            .arg(mic_wav_tmp.path().to_str().unwrap())
            .arg("norm");
        log::info!("normalize mic wave file: {:?}", command);
        let output = command.output()?;
        if !output.status.success() {
            log::error!("Cannot run sox: {:?}: {}", command, String::from_utf8_lossy(&output.stderr));
        }
    }

    // mix wave files
    let mut command = Command::new("sox");
    command
        .arg("-m") // mix
        .arg(mic_wav_tmp.path().to_str().unwrap())
        .arg(screen_tmp.path().to_str().unwrap())
        .arg(output_wave_file.clone())
        ;
    log::info!("Merge audio files: {:?}", command);
    let output = command
        .output()?;
    if !output.status.success() {
        log::error!("Cannot run sox: {:?}: {}", command, String::from_utf8_lossy(&output.stderr));
    }

    Ok(output_wave_file)
}

pub fn start_postprocess_thread(rx: Receiver<Entry>) {
    loop {
        match rx.recv() {
            Ok(entry) => {
                let config = load_config_or_default();
                let summarizer = config.build_summarizer()
                    .unwrap();
                let post_processor = PostProcessor::new(summarizer);

                let path = entry.dir.to_str().unwrap().to_string();
                match post_processor.postprocess(entry, config) {
                    Ok(_) => {
                        log::info!("Successfully processed: {}", path);
                    }
                    Err(e) => {
                        log::error!("Cannot process {}: {:?}", path, e)
                    }
                }
            }
            Err(err) => {
                log::error!("Cannot receive message: {:?}", err);
            }
        }
    }
}
