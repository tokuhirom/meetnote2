use screencapturekit::sc_shareable_content::SCShareableContent;

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


pub fn is_there_target_windows() -> bool {
    let current = SCShareableContent::current();

    let patterns = get_window_patterns();

    for window in current.windows {
        if let Some(title) = window.title {
            if let Some(app) = window.owning_application {
                if let Some(bundle_id) = app.bundle_identifier {
                    for pattern in &patterns {
                        if pattern.bundle_id == bundle_id && pattern.window_title == title {
                            return true
                        }
                    }
                }
            }
        }
    }
    return false
}
