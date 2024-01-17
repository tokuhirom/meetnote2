// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod mic_audio;
mod window;
mod mp3;
mod openai;
mod postprocess;
mod recording_proc;
mod config;
mod screencapture;
mod data_repo;
mod postprocess_resumer;
mod whisper_cpp;
mod webvtt;
mod screen_audio;
mod lindera_tokenizer;
pub mod tokenizer;
mod summarizer;
mod openai_summarizer;
mod tf_idf_summarizer;
mod transcriber;
mod openai_transcriber;

use std::fs::File;
use anyhow::anyhow;
use simplelog::ColorChoice;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder, SystemTray, SystemTrayMenu, Manager};
use crate::config::MeetNoteConfig;
use crate::data_repo::MdFile;
use crate::webvtt::Caption;
use crate::window::WindowInfo;

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
    mic_audio::get_input_devices()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_file(filename: String) -> Result<(), String> {
    data_repo::delete_file(&filename)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn save_file(filename: String, content: String) -> Result<(), String> {
    data_repo::save_file(&filename, &content)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn load_webvtt(filename: String) -> Result<Vec<Caption>, String> {
    data_repo::load_webvtt(&filename)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn read_data_tag_mp3(filename: String) -> Result<String, String> {
    data_repo::read_data_tag_mp3(&filename)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn regenerate_summary(filename: String) -> Result<(), String> {
    data_repo::regenerate_summary(&filename)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_windows() -> Vec<WindowInfo> {
    window::get_windows()
}

#[tauri::command]
fn is_recording() -> bool {
    recording_proc::is_recording()
}

fn main() -> anyhow::Result<()> {
    let config = simplelog::ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("Cannot get timezone")
        .build();

    simplelog::CombinedLogger::init(vec![
        simplelog::TermLogger::new(
            simplelog::LevelFilter::Info,
            config.clone(),
            simplelog::TerminalMode::Mixed,
            ColorChoice::Auto
        ),
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            config,
            File::create(data_repo::get_data_dir().unwrap().join("meetnote2.log"))?
        ),
    ])?;

    let config = match config::load_config() {
        Ok(c) => { c }
        Err(err) => {
            // TODO: show dialog?
            log::error!("Cannot load configuration: {:?}", err);
            config::default_config()
        }
    };

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
        recording_proc::start_recording_process(config)
    });
    std::thread::spawn(move || {
        postprocess_resumer::resume_postprocess().unwrap();
    });

    let misc_menu = Submenu::new("Misc", Menu::new()
        .add_item(CustomMenuItem::new("configuration", "Configuration")
            .accelerator("Command+,")));
    let file_menu = Submenu::new("File", Menu::new()
        .add_item(CustomMenuItem::new("exit", "Exit")));
    let window_menu = Submenu::new("Window", Menu::new()
        .add_item(CustomMenuItem::new("window_close", "Close")
            .accelerator("Command+w")));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_submenu(file_menu)
        .add_submenu(window_menu)
        .add_submenu(misc_menu);

    let tray_menu = SystemTrayMenu::new();
    let tray = SystemTray::new()
        .with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .menu(menu)
        .setup(|app| {
            let window = WindowBuilder::new(
                app,
                "main-window".to_string(),
                tauri::WindowUrl::App("index.html".into()),
            )
                .build()?;
            window.set_title("MeetNote2")?;

            Ok(())
        })
        .on_menu_event(|event| {
            match event.menu_item_id() {
                "exit" => {
                    std::process::exit(0);
                }
                "configuration" => {
                    log::info!("Got configuration event");
                    if let Err(err) = WindowBuilder::new(
                        &event.window().app_handle(),
                        "config-window".to_string(),
                        tauri::WindowUrl::App("configuration.html".into()),
                    )
                        .build() {
                        log::error!("Cannot open configuration window: {:?}", err);
                    };
                }
                "window_close" => {
                    log::info!("Closing window: '{:?}'", event.window().title());
                    if let Err(err) = event.window().close() {
                        log::error!("Cannot close window: {:?}", err)
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            load_files, delete_file, save_file,
            get_input_devices,
            load_config, save_config,
            load_webvtt,
            read_data_tag_mp3,
            regenerate_summary,
            get_windows,
            is_recording,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
