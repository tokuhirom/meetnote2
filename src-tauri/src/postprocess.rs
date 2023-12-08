use std::fs;
use std::fs::remove_file;
use std::process::Command;
use crate::{mp3, openai, whisper};
use anyhow::{anyhow, Result};

pub fn postprocess(openai_api_key: &String, mic_wav_file: String, language: &str) -> Result<()>{
    let wav_file = merge_audio_files(mic_wav_file.clone())?;

    // convert to MP3
    let mp3_file = wav_file.replace(".wav", ".mp3");
    if let Err(e) = mp3::convert_to_mp3(&wav_file, &mp3_file) {
        return Err(anyhow!("Cannot convert to mp3({} to {}): {:?}", wav_file, mp3_file, e))
    }

    let openai = openai::OpenAICustomizedClient::new(openai_api_key)?;

    // convert to VTT
    let vtt_file = wav_file.replace(".wav", ".vtt");
    log::info!("Convert {} to {}", mp3_file, vtt_file);
    // バージョンとモデルは変更可能にしたい
    match whisper::run_whisper("v1.5.1", "small",  language, &wav_file, &vtt_file) {
        Ok(_) => {
            log::info!("Wrote transcript to {}", vtt_file);
        }
        Err(e) => {
            return Err(anyhow!("Cannot transcribe from wave file: {:?}, {:?}", wav_file, e))
        }
    }

    // Summarize VTT
    let summary_file = wav_file.clone().replace(".wav", ".md");
    let vtt_result = fs::read_to_string(vtt_file.clone());
    let Ok(vtt_content) = vtt_result else {
        return Err(anyhow!("Cannot read VTT file({}): {:?}",
            vtt_file,
            vtt_result
        ))
    };
    let chat_messages = vec![
        openai::Message {
            role: "system".to_string(),
            content: "
                Please summarize the main discussions and conclusions of this
                meeting and organize the result in Markdown format. Specifically,
                present the title as a section header on the first line, followed
                by the content in bullet point format. The purpose is to make
                the content easily comprehensible for later review.
                Output text must be in Japanese.
                If the content doesn't contain any meaningful discussion, just output `NO_CONTENT`.
            ".trim().to_string(),
        },
        openai::Message {
            role: "user".to_string(),
            content: vtt_content,
        }
    ];
    println!("Requesting summarization: {}", vtt_file);
    match  openai.chat_completion(&openai::ChatCompletionRequest {
        model: "gpt-4-32k".to_string(),
        messages: chat_messages,
    }) {
        Ok(resp) => {
            let summary =  &resp.choices[0].message.content;
            if let Err(e) = fs::write(summary_file.clone(), summary) {
                return Err(anyhow!("Cannot write to file({}): {:?}",
                    summary_file, e))
            }

        }
        Err(err) => {
            return Err(anyhow!("Cannot generate summary from vtt file({}): {:?}",
                vtt_file, err))
        }
    }

    remove_file(wav_file)?;
    remove_file(mic_wav_file.clone())?;

    let raw_files = glob::glob(&*mic_wav_file.replace(".mic.wav", "*.raw"))?;
    for x in raw_files {
        let y = x.unwrap();
        remove_file(y.to_str().unwrap())?;
    }

    Ok(())
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
        let raw_files = glob::glob(&*mic_wav_file.replace(".mic.wav", "*.raw"))?;
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
            command.arg(x.unwrap().to_str().unwrap().to_string());
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
