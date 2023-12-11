use screencapturekit::sc_shareable_content::SCShareableContent;
use serde::{Deserialize, Serialize};
use crate::config::load_config;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowPattern {
    pub(crate) bundle_id: String,
    pub(crate) window_title: String,
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
