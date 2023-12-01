use crate::{mp3, openai};
use std::fs::File;
use std::io::prelude::*;
use anyhow::{anyhow, Result};


pub fn postprocess(openai_api_key: &str, wav_file: String, language: &str) -> Result<()>{
    // convert to MP3
    let mp3_file = wav_file.replace(".wav", ".mp3");
    if let Err(e) = mp3::convert_to_mp3(&wav_file, &mp3_file) {
        return Err(anyhow!("Cannot convert to mp3({}): {:?}", mp3_file,     e))
    }

    let openai = openai::OpenAICustomizedClient::new(openai_api_key);

    // convert to VTT
    let vtt_file = wav_file.replace(".wav", ".vtt");
    match openai.transcript(&*mp3_file, language) {
        Ok(content) => {
            println!("Got transcript: {}", content);
            println!("Write transcript to {}", vtt_file);
            let mut file = File::create(vtt_file)?;
            file.write_all(content.as_bytes())?;
        }
        Err(e) => {
            return Err(anyhow!("Cannot transcribe from mp3: {:?}", e))
        }
    }

    Ok(())
}
