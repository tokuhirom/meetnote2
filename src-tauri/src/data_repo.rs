use std::cmp::Reverse;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use anyhow::anyhow;
use chrono::Local;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use crate::postprocess::PostProcessor;
use crate::config;
use crate::entry::Entry;

pub struct DataRepo {
    pub data_dir: PathBuf,
}

impl DataRepo {
    pub fn new() -> anyhow::Result<Self> {
        let app_data_dir = get_app_data_dir()?;
        Ok(DataRepo { data_dir: app_data_dir.join("data") })
    }

    pub fn new_entry(&self) -> anyhow::Result<Entry> {
        let now = Local::now();
        let dirname = now.format("%Y%m%d%H%M%S").to_string();

        let dir = self.data_dir.join(dirname);

        fs::create_dir_all(&dir)?;

        Ok(Entry::new(dir))
    }
}

// TODO make this private...
pub fn get_app_data_dir() -> anyhow::Result<PathBuf> {
    let app_data_dir = dirs::data_dir()
        .ok_or(anyhow!("Cannot get home directory"))?
        .join("com.github.tokuhirom.meetnote2");
    fs::create_dir_all(&app_data_dir)?;
    Ok(app_data_dir)
}

pub(crate) fn regenerate_summary(vtt_path: &String, md_path: &String) -> anyhow::Result<()> {
    log::info!("Regenerating summary from {} to {}", vtt_path, md_path);

    let config = config::load_config()?;
    let summarizer = config.build_summarizer()?;
    let post_processor = PostProcessor::new(
        summarizer
    );
    post_processor.summarize(vtt_path.as_str(), md_path)
}
