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
mod postprocess_resumer;

use std::fs::File;
use anyhow::anyhow;
use simplelog::ColorChoice;
use tauri::{CustomMenuItem, Manager, Menu, MenuItem, Submenu, WindowBuilder};
use crate::config::MeetNoteConfig;
use crate::data_repo::MdFile;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn load_files() -> Vec<MdFile> {
    data_repo::load_files()
}

#[tauri::command]
fn load_config() -> Result<MeetNoteConfig, String>{
    config::load_config()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_config(config: MeetNoteConfig) -> Result<(), String>{
    config::save_config(&config)
        .map_err(|e| e.to_string())
}


#[tauri::command]
fn get_input_devices() -> Result<Vec<String>, String> {
    audio::get_input_devices()
        .map_err(|e| e.to_string())
}

fn main() -> anyhow::Result<()> {
    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            simplelog::TerminalMode::Mixed,
            ColorChoice::Auto
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create(data_repo::get_data_dir().unwrap().join("meetnote2.log"))?
        ),
    ])?;


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
        log::info!("✅ Platform supported");
    }

    // #2 Check if we have permission to capture the screen
    let has_permission = screencapture::has_permission();
    if !has_permission {
        return Err(anyhow!("❌ Permission not granted"));
    } else {
        log::info!("✅ Permission granted");
    }

    std::thread::spawn(move || {
        recording_proc::start_recording_process(openai_api_token, config.target_device)
    });
    std::thread::spawn(move || {
        postprocess_resumer::resume_postprocess().unwrap();
    });

    let misc_menu = Submenu::new("Misc", Menu::new()
        .add_item(CustomMenuItem::new("configuration", "Configuration")
            .accelerator("Command+,"))
        .add_item(CustomMenuItem::new("window_list", "Window list...")));
    let file_menu = Submenu::new("File", Menu::new()
        .add_item(CustomMenuItem::new("exit", "Exit")));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(file_menu)
        .add_submenu(misc_menu);

    tauri::Builder::default()
        .setup(|app| {
            WindowBuilder::new(
                app,
                "main-window".to_string(),
                tauri::WindowUrl::App("index.html".into()),
            )
                .menu(menu)
                .build()?;

            Ok(())
        })
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "exit" => {
                    std::process::exit(0);
                }
                "configuration" => {
                    if let Err(err) = WindowBuilder::new(
                        &event.window().app_handle(),
                        "config-window".to_string(),
                        tauri::WindowUrl::App("configuration.html".into()),
                    )
                        .build() {
                        log::error!("Cannot open configuration window: {:?}", err);
                    };
                }
                "window_list" => {
                    // WindowBuilder::new(event.window().app_handle(), )
                    // event.window().close().unwrap();
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![greet, load_files,get_input_devices, load_config, save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
