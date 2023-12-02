mod audio;
mod window;
mod mp3;
mod openai;
mod postprocess;
mod recording_proc;
mod config;

use anyhow::__private::kind::TraitKind;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder, TrayIconEvent,
};

use winit::event_loop::{ControlFlow, EventLoopBuilder};


fn main() -> anyhow::Result<()> {
    let config = config::load_config()?;
    let openai_api_token = config.openai_api_token.ok_or(
        anyhow!("Missing OpenAI API token in the configuration file: {:?}",
        config::get_config_path()?)
    )?;

    // #1 Check if the platform is supported
    let supported = MeetNote2::is_supported();
    if !supported {
        return Err(anyhow!("❌ Platform not supported"));
    } else {
        println!("✅ Platform supported");
    }

    // #2 Check if we have permission to capture the screen
    let has_permission = MeetNote2::has_permission();
    if !has_permission {
        return Err(anyhow!("❌ Permission not granted"));
    } else {
        println!("✅ Permission granted");
    }

    std::thread::spawn(move || {
        recording_proc::start_recording_process(openai_api_token, config.target_device)
    });

    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/icon.png");
    let icon = load_icon(std::path::Path::new(path));

    let event_loop = EventLoopBuilder::new()
        .build().unwrap();

    let menu = Menu::with_items(&[
        &MenuItem::new("hello", true, Option::None)
    ]).unwrap();
    let _tray_icon = Some(
        TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("winit - awesome windowing lib")
            .with_icon(icon)
            .build()
            .unwrap(),
    );

    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();

    event_loop.run(move |_event, event_loop| {
        event_loop.set_control_flow(ControlFlow::Poll);

        if let Ok(event) = tray_channel.try_recv() {
            println!("{event:?}");
        }
        if let Ok(event) = menu_channel.try_recv() {
            println!("{event:?}");
        }
    }).expect("Cannot start event loop!");
    Ok(())
}


fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
