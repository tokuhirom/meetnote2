use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Default, Serialize, Deserialize)]
pub struct MeetNoteConfig {
    // OpenAI's API token
    pub(crate) openai_api_token: Option<String>,
    // The target input device
    pub(crate) target_device: Option<String>,
}

fn config_dir() ->  Option<PathBuf> {
    env::var_os("XDG_CONFIG_HOME").and_then(dirs_sys::is_absolute_path)
        .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let config_dir = config_dir().ok_or(
        anyhow!("Cannot get configuration directory")
    )?;
    Ok(config_dir.join("meetnote2/config.json"))
}

pub fn load_config() -> anyhow::Result<MeetNoteConfig> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(MeetNoteConfig { openai_api_token: None, target_device: None });
    }

    let mut file = match File::open(&config_path) {
        Ok(file) => file,
        Err(e) => return Err(anyhow::anyhow!(
            "Failed to open config file({:?}): {}", config_path, e)),
    };

    let mut file_data = String::new();
    file.read_to_string(&mut file_data)?;

    // Parse JSON
    let config: MeetNoteConfig = serde_json::from_str(&file_data)?;

    Ok(config)
}
