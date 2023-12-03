// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio;
mod window;
mod mp3;
mod openai;
mod postprocess;
mod recording_proc;
mod config;
mod screencapture;
mod data_repo;

use anyhow::anyhow;
use crate::data_repo::MdFile;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_files() -> Vec<MdFile> {
    return data_repo::load_files();
}

fn main() -> anyhow::Result<()> {
    let config = config::load_config()?;
    let openai_api_token = config.openai_api_token.ok_or(
        anyhow!("Missing OpenAI API token in the configuration file: {:?}",
        config::get_config_path()?)
    )?;

    // #1 Check if the platform is supported
    let supported = screencapture::is_supported();
    if !supported {
        return Err(anyhow!("❌ Platform not supported"));
    } else {
        println!("✅ Platform supported");
    }

    // #2 Check if we have permission to capture the screen
    let has_permission = screencapture::has_permission();
    if !has_permission {
        return Err(anyhow!("❌ Permission not granted"));
    } else {
        println!("✅ Permission granted");
    }

    std::thread::spawn(move || {
        recording_proc::start_recording_process(openai_api_token, config.target_device)
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, load_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
