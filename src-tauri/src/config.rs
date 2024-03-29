use std::path::PathBuf;
use std::fs::{File, rename, write};
use std::io::Read;
use std::string::ToString;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use TranscriberType::WhisperCppTranscriberType;
use crate::config::SummarizerType::{OpenAISummarizerType, TFIDFSummarizerType};
use crate::openai_summarizer::OpenAISummarizer;
use crate::summarizer::Summarizer;
use crate::tf_idf_summarizer::TFIDFSummarizer;
use crate::window::WindowPattern;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum TranscriberType {
    #[default]
    WhisperCppTranscriberType,
    OpenAITranscriberType,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum SummarizerType {
    #[default]
    TFIDFSummarizerType,
    OpenAISummarizerType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeetNoteConfig {
    // OpenAI's API token
    pub openai_api_token: Option<String>,
    // The target input device
    pub target_device: Option<String>,
    // The default model
    pub whisper_model: String,
    // Target window patterns
    pub window_patterns: Vec<WindowPattern>,
    #[serde(default = "default_transcriber_type")]
    pub transcriber_type: TranscriberType,
    #[serde(default = "default_summarizer_type")]
    pub summarizer_type: SummarizerType,
    // Target language
    #[serde(default = "default_language")]
    pub language: String,
}

fn default_language() -> String {
    "ja".to_string()
}

fn default_transcriber_type() -> TranscriberType {
    WhisperCppTranscriberType
}

fn default_summarizer_type() -> SummarizerType {
    TFIDFSummarizerType
}

impl MeetNoteConfig {
    pub fn build_summarizer(&self) -> anyhow::Result<Box<dyn Summarizer>>{
        match &self.summarizer_type {
            TFIDFSummarizerType => {
                Ok(Box::new(TFIDFSummarizer::new()?))
            }
            OpenAISummarizerType => {
                match &self.openai_api_token {
                    Some(token) => {
                        Ok(Box::new(OpenAISummarizer::new(token.as_str())?))
                    }
                    None => {
                        Err(anyhow!("You set OpenAISummarizer, but Missing openai api token in configuration file"))
                    }
                }
            }
        }
    }
}

impl Default for MeetNoteConfig {
    fn default() -> Self {
        MeetNoteConfig {
            transcriber_type: WhisperCppTranscriberType,
            summarizer_type: SummarizerType::TFIDFSummarizerType,
            openai_api_token: None,
            target_device: None,
            whisper_model: "small".to_string(),
            language: "ja".to_string(),
            window_patterns: vec![
                WindowPattern {
                    bundle_id: String::from("us.zoom.xos"),
                    window_title: String::from("Zoom Meeting"),
                },
                WindowPattern {
                    bundle_id: String::from("us.zoom.xos"),
                    window_title: String::from("zoom share toolbar window"),
                },
                WindowPattern {
                    bundle_id: String::from("us.zoom.xos"),
                    window_title: String::from("zoom share statusbar window"),
                },
            ],
        }
    }
}

pub fn get_config_path() -> anyhow::Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .ok_or(
        anyhow!("Cannot get configuration directory")
    )?;
    Ok(config_dir.join("meetnote2/config.json"))
}

pub fn default_config() -> MeetNoteConfig {
    Default::default()
}


pub fn load_config_or_default() -> MeetNoteConfig {
    match load_config() {
        Ok(c) => { c }
        Err(err) => {
            log::error!("Cannot load config: {:?}", err);
            Default::default()
        }
    }
}

pub fn load_config() -> anyhow::Result<MeetNoteConfig> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(default_config());
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

pub fn save_config(config: &MeetNoteConfig) -> anyhow::Result<()> {
    let config_path = get_config_path()?;
    let tmp_path = config_path.with_extension("tmp");

    log::info!("Saving configuration: {:?} to {:?}", config, config_path);

    // Convert the config data to JSON
    let config_data = serde_json::to_string_pretty(config)?;

    // Write the config data to the temp file
    std::fs::create_dir_all(tmp_path.parent().unwrap())?;
    write(&tmp_path, config_data.clone())?;

    // Atomically replace the old config file with the temp file
    rename(tmp_path, config_path)?;

    Ok(())
}
