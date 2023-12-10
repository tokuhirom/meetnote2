use std::cmp::Reverse;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use anyhow::anyhow;
use chrono::Local;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use crate::webvtt::{Caption, parse_webvtt};

pub fn get_data_dir() -> anyhow::Result<PathBuf> {
    let app_data_dir = dirs::data_dir()
        .ok_or(anyhow!("Cannot get home directory"))?
        .join("com.github.tokuhirom.meetnote2");
    Ok(app_data_dir)
}

pub fn new_mic_wave_file_name() -> anyhow::Result<PathBuf> {
    let now = Local::now();
    let dirname = now.format("%Y%m%d").to_string();

    let data_dir = get_data_dir()?;
    let dir = data_dir.join(dirname);

    fs::create_dir_all(&dir)?;

    let filename = now.format("%Y%m%d%H%M%S.mic.wav").to_string();
    Ok(dir.join(filename))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MdFile {
    pub filename: String,
    pub content: String,
}

impl MdFile {
    fn new(path: &PathBuf, base_dir: &PathBuf) -> anyhow::Result<MdFile> {
        let filepath = path.strip_prefix(base_dir)
            .expect("Failed to get relative path")
            .to_str().unwrap().to_string();
        let mut file = fs::File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(MdFile {
            filename: filepath,
            content,
        })
    }
}

pub fn load_files() -> Vec<MdFile> {
    log::debug!("Loading files...");

    let data_dir = match get_data_dir() {
        Ok(d) => { d }
        Err(err) => {
            println!("Cannot get data directory: {}", err);
            return Vec::new();
        }
    };
    let mut results = Vec::new();

    for entry in WalkDir::new(&data_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() && entry.path().extension().unwrap_or_default() == "md" {
            let path = entry.into_path();
            match MdFile::new(&path, &data_dir) {
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

pub(crate) fn delete_file(filename: &String) -> anyhow::Result<()> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join(filename);

    let related_extensions = vec!["md", "vtt", "mp3"];
    for ext in related_extensions {
        let related_file_path = file_path.with_extension(ext);
        if related_file_path.exists() {
            log::info!("Deleting file: {}", related_file_path.to_str().unwrap());
            fs::remove_file(&related_file_path).map_err(|e| anyhow!("Failed to delete file {:?}: {}", related_file_path, e))?;
        }
    }

    Ok(())
}

pub(crate) fn save_file(filename: &String, content: &String) -> anyhow::Result<()> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join(filename);

    log::info!("Saving content to {:?}, {:?}", file_path, content);

    match fs::write(&file_path, content) {
        Ok(_) => {
            Ok(())
        }
        Err(err) => {
            Err(anyhow!("Cannot write content to {:?}: {:?}", file_path, err))
        }
    }
}

pub(crate) fn load_webvtt(filename: &String) -> anyhow::Result<Vec<Caption>> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join(filename);

    log::info!("Loading webvtt content from {:?}", file_path);
    let vtt_src = match fs::read_to_string(file_path.as_path()) {
        Ok(s) => {s}
        Err(err) => {
            return Err(anyhow!("cannot load vtt file: {:?}, {}", file_path, err))
        }
    };

    let vtt = parse_webvtt(&vtt_src);
    Ok(vtt)
}

pub(crate) fn read_data_tag_mp3(filename: &String) -> anyhow::Result<String> {
    let data_dir = get_data_dir()?;
    let file_path = data_dir.join(filename);
    let vec = fs::read(file_path)?;
    Ok(format!("data:audio/mpeg;base64,{}", base64::encode(vec)))
}
