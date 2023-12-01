use std::fs;
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
    match openai.transcript(&mp3_file, language) {
        Ok(content) => {
            println!("Got transcript: {}", content);
            println!("Write transcript to {}", vtt_file);
            let mut file = File::create(vtt_file.clone())?;
            file.write_all(content.as_bytes())?;
        }
        Err(e) => {
            return Err(anyhow!("Cannot transcribe from mp3: {:?}", e))
        }
    }

    // Summarize VTT
    let summary_file = wav_file.replace(".wav", ".md");
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

    Ok(())
}
