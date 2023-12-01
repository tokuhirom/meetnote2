use reqwest::blocking::multipart;
use anyhow::Result;
use std::fs::File;
use std::io::Read;

// fn main() -> Result<()> {
//     let api_key = "YOUR OPENAI API KEY";
//     let path = "path/to/your/audio.mp3";
//     let transcription = get_transcription(api_key, path)?;
//     println!("Transcription: {}", transcription );
//     Ok(())
// }

pub(crate) fn get_transcription(api_key: &str, file_path: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();

    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let file_name = std::path::Path::new(file_path)
        .file_name()
        .ok_or(anyhow::anyhow!("Failed to get file name"))?
        .to_string_lossy()
        .into_owned();
    let part = multipart::Part::bytes(buffer)
        .file_name(file_name);

    let form = multipart::Form::new()
        .text("model", "whisper-1")
        .text("language", "ja")
        .text("response_format", "vtt")
        .part("file", part);

    let mut response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .bearer_auth(api_key)
        .multipart(form)
        .send()?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("API request failed with status {}: {}", response.status(), response.text()?));
    }

    Ok(response.text()?)
}
