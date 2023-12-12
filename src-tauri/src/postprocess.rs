use std::fs;
use std::process::Command;
use crate::{mp3, whisper};
use anyhow::{anyhow, Result};
use crate::summarizer::Summarizer;

pub struct PostProcessor {
    summarizer: Box<dyn Summarizer>,
}

impl PostProcessor {
    pub fn new(summarizer: Box<dyn Summarizer>) -> Box<PostProcessor> {
        Box::new(PostProcessor { summarizer })
    }

    pub fn postprocess(&self, mic_wav_file: String, language: &str, whisper_model: &str) -> Result<()>{
        let wav_file = merge_audio_files(mic_wav_file.clone())?;

        // convert to MP3
        let mp3_file = wav_file.replace(".wav", ".mp3");
        if let Err(e) = mp3::convert_to_mp3(&wav_file, &mp3_file) {
            return Err(anyhow!("Cannot convert to mp3({} to {}): {:?}", wav_file, mp3_file, e))
        }

        // convert to VTT
        let vtt_file = wav_file.replace(".wav", ".vtt");
        log::info!("Convert {} to {}", mp3_file, vtt_file);
        // バージョンとモデルは変更可能にしたい
        match whisper::run_whisper("v1.5.1", whisper_model,  language, &wav_file, &vtt_file) {
            Ok(_) => {
                log::info!("Wrote transcript to {}", vtt_file);
            }
            Err(e) => {
                return Err(anyhow!("Cannot transcribe from wave file: {:?}, {:?}", wav_file, e))
            }
        }

        // Summarize VTT
        let summary_file = wav_file.clone().replace(".wav", ".md");
        self.summarize(vtt_file.as_str(), summary_file.as_str())?;

        // cleanup files
        file_remove(wav_file.as_str())?;
        file_remove(mic_wav_file.clone().as_str())?;
        let raw_files = glob::glob(&mic_wav_file.replace(".mic.wav", "*.raw"))?;
        for x in raw_files {
            let y = x.unwrap();
            file_remove(y.to_str().unwrap())?;
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
        log::info!("Requesting summarization: {}", vtt_file);

        let summary = self.summarizer.summarize(vtt_content.as_str())
            .map_err(|err| { anyhow!("Cannot process {:?}: {:?}", vtt_file, err)})?;

        if let Err(e) = fs::write(summary_file, summary) {
            return Err(anyhow!("Cannot write to file({}): {:?}",
                    summary_file, e))
        }

        log::info!("Finished summarization: {}", vtt_file);

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

fn merge_audio_files(mic_wav_file: String) -> anyhow::Result<String> {
    let output_wave_file = mic_wav_file.replace(".mic.wav", ".wav");

    // merge raw files to 1 wav file
    let screen_tmp = tempfile::Builder::new()
        .suffix(".wav")
        .rand_bytes(5)
        .tempfile()
        .unwrap();
    {
        let raw_files = glob::glob(&mic_wav_file.replace(".mic.wav", "*.raw"))?;
        log::info!("Processing raw files: {:?}", raw_files);
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
        }
        command.arg(screen_tmp.path().to_str().unwrap());
        command.arg("norm");
        log::info!("Merge & normalize raw file: {:?}", command);
        let output = command.output()?;
        if !output.status.success() {
            log::error!("Cannot run sox: {:?}: {}", command, String::from_utf8_lossy(&output.stderr));
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
