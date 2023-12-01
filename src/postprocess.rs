use openai::get_transcription;
use crate::{mp3, openai};
use std::fs::File;
use std::io::prelude::*;
use anyhow::Result;


pub fn postprocess(openai_api_key: &str, wav_file: String) -> Result<()>{
    // convert to MP3
    let mp3_file = wav_file.replace(".wav", ".mp3");
    if let Err(e) = mp3::convert_to_mp3(&wav_file, &mp3_file) {
        eprintln!("Failed to convert to mp3: {}", e);
        return Err(e)
    }

    // convert to VTT
    let vtt_file = wav_file.replace(".wav", ".vtt");
    match get_transcription(openai_api_key, &*mp3_file) {
        Ok(content) => {
            println!("Got transcript: {}", content);
            println!("Write transcript to {}", vtt_file);
            let mut file = File::create(vtt_file)?;
            file.write_all(content.as_bytes())?;
        }
        Err(e) => {
            return Err(e)
        }
    }

    Ok(())
}
