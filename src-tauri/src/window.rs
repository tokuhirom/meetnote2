use screencapturekit::sc_shareable_content::SCShareableContent;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowPattern {
    pub(crate) bundle_id: String,
    pub(crate) window_title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowInfo {
    pub bundle_id: String,
    pub window_title: String,
    pub height: u32,
    pub width: u32,
    pub is_on_screen: bool,
}

pub fn get_windows() -> Vec<WindowInfo> {
    let current = SCShareableContent::current();
    let mut result = Vec::new();
    for window in current.windows {
        // When a window is displayed on a different screen, the is_active flag becomes false.
        // Therefore, you should not filter using this flag. This is because, during meetings,
        // there is a possibility that windows from applications like Zoom are being displayed
        // on a separate screen.
        // https://developer.apple.com/documentation/screencapturekit/scwindow/4110525-active?language=objc
        //
        // if !window.is_active {
        //     continue
        // }

        if let Some(title) = window.title {
            if let Some(app) = window.owning_application {
                if let Some(bundle_id) = app.bundle_identifier {
                    result.push(WindowInfo {
                        bundle_id,
                        window_title: title,
                        height: window.height,
                        width: window.width,
                        is_on_screen: window.is_on_screen,
                    });
                }
            }
        }
    }
    result.sort_by(|a, b| a.bundle_id.cmp(&b.bundle_id)
        .then_with(|| a.window_title.cmp(&b.window_title)));
    result
}

