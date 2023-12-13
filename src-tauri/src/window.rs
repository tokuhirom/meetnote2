
use screencapturekit::sc_shareable_content::SCShareableContent;
use serde::{Deserialize, Serialize};
use crate::config::load_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowPattern {
    pub(crate) bundle_id: String,
    pub(crate) window_title: String,
}

#[derive(Debug, Serialize, Deserialize)]
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
        if !window.is_active {
            continue
        }
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

pub fn is_there_target_windows() -> bool {
    let current = SCShareableContent::current();

    let patterns = match load_config() {
        Ok(conf) => { conf.window_patterns }
        Err(err) => {
            log::error!("Cannot load configuration: {:?}", err);
            // TODO show dialog?
            return false
        }
    };

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
    false
}
