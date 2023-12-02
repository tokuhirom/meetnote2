use std::fs;
use std::path::PathBuf;
use anyhow::anyhow;
use chrono::Local;

pub fn get_data_dir() -> anyhow::Result<PathBuf> {
    let home_dir = dirs::home_dir()
        .ok_or(anyhow!("Cannot get home directory"))?;
    Ok(home_dir.join("MeetNote"))
}

pub fn new_wave_file_name() -> anyhow::Result<PathBuf> {
    let now = Local::now();
    let dirname = now.format("%Y%m%d").to_string();

    let data_dir = get_data_dir()?;
    let dir = data_dir.join(dirname);

    fs::create_dir_all(&dir)?;

    let filename = now.format("%Y%m%d%H%M%S.wav").to_string();
    Ok(dir.join(filename))
}
