
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


    // #4 Create Options
    let options = MeetNote2::Options {
        fps: 60,
        targets,
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
    };

    // #5 Create Recorder
    let mut recorder = MeetNote2::Recorder::init(options);

    // #6 Start Capture
    recorder.start_capture();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // #7 Stop Capture
    recorder.stop_capture();
}

