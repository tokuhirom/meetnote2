use std::process::Command;
use anyhow::{Result, anyhow};

pub(crate) fn convert_to_mp3(wav_file: &str, mp3_file: &str) -> Result<()> {
    let output = Command::new("lame")
        .arg("--verbose")
        .arg("-v")
        .arg("--abr")
        .arg("58")
        .arg("-m")
        .arg("m")
        .arg(wav_file)
        .arg(mp3_file)
        .output()?;

    if !output.status.success() {
        return Err(anyhow!("Failed to convert to mp3: {}",
            String::from_utf8_lossy(&output.stderr)));
    }

    log::info!("Converted {} to {}", wav_file, mp3_file);
    Ok(())
}
