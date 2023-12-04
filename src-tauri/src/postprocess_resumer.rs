use anyhow::anyhow;
use crate::data_repo;
use crate::config;
use crate::postprocess::postprocess;

pub fn resume_postprocess() -> anyhow::Result<()> {
    let wave_files = data_repo::get_unprocessed_wave_files();
    let config = config::load_config()?;
    let openai_api_token = config.openai_api_token
        .ok_or(anyhow!("Missing openai api token in configuration file"))?;
    for wave in &wave_files {
        match postprocess(&openai_api_token,
                    wave.to_str().unwrap().to_string(),
                    "ja") {
            Ok(_) => {
                log::info!("Proceeded {:?}", wave.to_str());
            }
            Err(err) => {
                log::error!("Cannot process {:?}: {:?}", wave.to_str(), err)
            }
        }
    }

    log::info!("Proceeded {} files...", wave_files.len());
    Ok(())
}