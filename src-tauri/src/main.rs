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
mod entry;

use std::fs::File;
use std::path::PathBuf;
use std::ptr::null;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use anyhow::anyhow;
use simplelog::ColorChoice;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowBuilder, SystemTray, SystemTrayMenu, Manager, AboutMetadata};
use crate::config::MeetNoteConfig;
use crate::entry::Entry;
use crate::postprocess::PostProcessStatus;
use crate::window::WindowInfo;

pub struct MyState {
    pub recording_tx: Sender<String>,
    pub postprocess_tx: Sender<Entry>,
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
fn regenerate_summary(vtt_path: String, md_path: String) -> Result<(), String> {
    data_repo::regenerate_summary(&vtt_path, &md_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_windows() -> Vec<WindowInfo> {
    window::get_windows()
}

#[tauri::command]
fn start_postprocess(dir: String, state: tauri::State<MyState>) -> Result<(), String> {
    let entry = Entry::new(PathBuf::from(dir));
    state.postprocess_tx.send(entry)
        .map_err(|err| format!("Cannot start postprocess: {:?}", err))
}

#[tauri::command]
fn call_recording_process(command: String, state: tauri::State<MyState>) -> Result<(), String> {
    state.recording_tx.send(command)
        .map_err(|err| format!("Cannot send message: {:?}", err))
}

#[tauri::command]
fn postprocess_status() -> PostProcessStatus {
    postprocess::postprocess_status()
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
            File::create(data_repo::get_app_data_dir().unwrap().join("meetnote2.log"))?
        ),
    ])?;

    let _config = match config::load_config() {
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

    let misc_menu = Submenu::new("Misc", Menu::new()
        .add_item(CustomMenuItem::new("configuration", "Configuration")
            .accelerator("Command+,")));
    let file_menu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("delete_entry", "Delete entry"))
            .add_item(CustomMenuItem::new("edit_summary", "Edit summary")
                .accelerator("Command+e"))
    );
    let edit_menu = Menu::new()
        .add_native_item(MenuItem::Undo)
        .add_native_item(MenuItem::Redo)
        .add_native_item(MenuItem::Separator)
        .add_native_item(MenuItem::Cut)
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Paste)
        .add_native_item(MenuItem::SelectAll);
    let window_menu = Submenu::new("Window", Menu::new()
        .add_item(CustomMenuItem::new("window_close", "Close")
            .accelerator("Command+w")));
    let menu = Menu::new()
        .add_submenu(Submenu::new(
            "MeetNote2",
            Menu::new()
                .add_native_item(MenuItem::About(
                    "MeetNote2".to_string(),
                    AboutMetadata::default(),
                ))
                .add_native_item(MenuItem::Quit)
        ))
        .add_submenu(file_menu)
        .add_submenu(Submenu::new("Edit", edit_menu))
        .add_submenu(window_menu)
        .add_submenu(misc_menu);

    let tray_menu = SystemTrayMenu::new();
    let tray = SystemTray::new()
        .with_menu(tray_menu);

    let (postprocess_tx, postprocess_rx) = mpsc::channel::<Entry>();
    thread::spawn(move || {
        postprocess::start_postprocess_thread(postprocess_rx)
    });
    let (recording_tx, recording_rx) = mpsc::channel::<String>();
    {
        let postprocess_tx = postprocess_tx.clone();
        thread::spawn(move || {
            recording_proc::start_recording_process_ex(recording_rx, postprocess_tx);
        });
    }

    tauri::Builder::default()
        .manage(MyState {
            recording_tx,
            postprocess_tx,
        })
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
                "edit_summary" => {
                    log::info!("Start editing summary");
                    if let Err(err) = event.window().emit("do_edit_summary", "DUMMY".to_string()) {
                        log::error!("Cannot emit message: {:?}", err);
                    }
                }
                "delete_entry" => {
                    log::info!("Start deleting entry");
                    if let Err(err) = event.window().emit("do_delete_entry", "DUMMY".to_string()) {
                        log::error!("Cannot emit message: {:?}", err);
                    }
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
            get_input_devices,
            load_config, save_config,
            regenerate_summary,
            get_windows,
            start_postprocess,
            call_recording_process,
            postprocess_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
