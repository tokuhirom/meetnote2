use std::thread::sleep;
use std::time::Duration;
use screencapturekit::sc_shareable_content::SCShareableContent;
use MeetNote2::get_targets;

pub struct WindowPattern {
    bundle_id: String,
    window_title: String,
}

// TOOD make this configurable
fn get_window_patterns() -> Vec<WindowPattern> {
    let mut result = Vec::new();
    result.push(
        WindowPattern {
            bundle_id: String::from("us.zoom.xos"),
            window_title: String::from("Zoom Meeting"),
        }
    );
    result.push(
        WindowPattern {
            bundle_id: String::from("us.zoom.xos"),
            window_title: String::from("zoom share toolbar window"),
        }
    );
    result.push(
        WindowPattern {
            bundle_id: String::from("us.zoom.xos"),
            window_title: String::from("zoom share statusbar window"),
        }
    );
    return result
}


fn is_there_target_windows() -> bool {
    let current = SCShareableContent::current();

    let patterns = get_window_patterns();

    for window in current.windows {
        if let Some(title) = window.title {
            if let Some(app) = window.owning_application {
                if let Some(bundle_id) = app.bundle_identifier {
                    for pattern in &patterns {
                        if (pattern.bundle_id == bundle_id && pattern.window_title == title) {
                            return true
                        }
                    }
                }
            }
        }
    }
    return false
}

fn main() {
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

    // #3 Get recording targets
    let targets = MeetNote2::get_targets();
    println!("🎯 Targets: {:?}", targets);

    while true {
        println!("{}", is_there_target_windows());

        sleep(Duration::from_secs(3))
    }


    // #4 Create Options
    let options = MeetNote2::Options {
        fps: 60,
        targets,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
    };

    // #5 Create Recorder
    let output_file = concat!(env!("CARGO_MANIFEST_DIR"), "/test/audio/recorded.wav");
    let mut recorder = MeetNote2::Recorder::init(options, output_file);

    // #6 Start Capture
    recorder.start_capture();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // #7 Stop Capture
    recorder.stop_capture();
}

