use std::cmp::Reverse;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use anyhow::anyhow;
use chrono::Local;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

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


#[derive(Debug, Serialize, Deserialize)]
pub struct MdFile {
    pub filename: String,
    pub content: String,
}

impl MdFile {
    fn new(path: &PathBuf) -> anyhow::Result<MdFile> {
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        let mut file = fs::File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(MdFile {
            filename,
            content,
        })
    }
}

pub fn load_files() -> Vec<MdFile> {
    log::info!("Loading files...");

    let data_dir = match get_data_dir() {
        Ok(d) => { d }
        Err(err) => {
            println!("Cannot get data directory: {}", err);
            return Vec::new();
        }
    };
    let mut results = Vec::new();

    for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "md" {
            let path = entry.into_path();
            match MdFile::new(&path) {
                Ok(mdfile) => {
                    results.push(mdfile);
                }
                Err(err) => {
                    println!("Cannot load mdfile: {:?}: {}", path, err)
                }
            };
        }
    }
    results.sort_by_key(|res| {
       Reverse(res.filename.to_string())
    });
    results
}

pub fn get_unprocessed_wave_files() -> Vec<PathBuf> {
    log::info!("Loading wave files...");

    let data_dir = match get_data_dir() {
        Ok(d) => { d }
        Err(err) => {
            println!("Cannot get data directory: {}", err);
            return Vec::new();
        }
    };
    let mut wave_files = Vec::new();

    for entry in WalkDir::new(data_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "wav" {
            let path = entry.into_path();
            wave_files.push(path);
        }
    }
    wave_files.sort_by_key(|file| Reverse(file.display().to_string()));
    wave_files
}
