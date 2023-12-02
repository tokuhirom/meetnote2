mod audio;
mod window;
mod mp3;
mod openai;
mod postprocess;
mod recording_proc;

use clap::Parser;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIconBuilder, TrayIconEvent,
};

use winit::event_loop::{ControlFlow, EventLoopBuilder};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[clap(long)] // , about = "The target input device")
    target_device: Option<String>,
}

fn main() {
    let opts = Opts::parse();
    let openai_api_key = std::env::var("OPENAI_API_KEY")
        .expect("Expected environment variable: OPENAI_API_KEY");

    // #1 Check if the platform is supported
    let supported = MeetNote2::is_supported();
    if !supported {
        println!("❌ Platform not supported");
        // TODO: use GUI dialog?
        return;
    } else {
        println!("✅ Platform supported");
    }

    // #2 Check if we have permission to capture the screen
    let has_permission = MeetNote2::has_permission();
    if !has_permission {
        println!("❌ Permission not granted");
        return;
    } else {
        println!("✅ Permission granted");
    }

    std::thread::spawn(move || {
        recording_proc::start_recording_process(openai_api_key, opts.target_device)
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
