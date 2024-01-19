use std::fs;
use anyhow::anyhow;
use crate::openai::OpenAICustomizedClient;
use crate::transcriber::Transcriber;

pub struct OpenAITranscriber {
    openai: OpenAICustomizedClient,
    language: String,
}

impl OpenAITranscriber {
    pub fn new(openai: OpenAICustomizedClient, language: String) -> OpenAITranscriber {
        OpenAITranscriber { openai, language }
    }
}

impl Transcriber for OpenAITranscriber {
    fn transcribe(&self, in_file: &str, out_file: &str) -> anyhow::Result<()> {
        match self.openai.transcript(in_file, &self.language) {
            Ok(txt) => {
                if let Err(err) = fs::write(out_file, txt) {
                    return Err(anyhow!("Cannot write result to: {:?}", err))
                }
                Ok(())
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}
