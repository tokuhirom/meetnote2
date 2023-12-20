use crate::config::{load_config_or_default};
use crate::data_repo;

use crate::postprocess::PostProcessor;
use crate::tf_idf_summarizer::TFIDFSummarizer;

pub fn resume_postprocess() -> anyhow::Result<()> {
    let wave_files = data_repo::get_unprocessed_wave_files();
    // let config = config::load_config()?;
    // let openai_api_token = config.openai_api_token
    //     .ok_or(anyhow!("Missing openai api token in configuration file"))?;
    // let summarizer = OpenAISummarizer::new(openai_api_key)?;
    let post_processor = PostProcessor::new(
        Box::new(TFIDFSummarizer::new()?)
    );
    let config = load_config_or_default();
    for wave in &wave_files {
        match post_processor.postprocess(
            wave.to_str().unwrap().to_string(),
            config.clone()
        ) {
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