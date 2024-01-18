use anyhow::anyhow;
use crate::openai;
use crate::openai::OpenAICustomizedClient;
use crate::summarizer::Summarizer;

pub struct OpenAISummarizer {
    openai: OpenAICustomizedClient,
}

impl OpenAISummarizer {
    pub fn new (openai_api_key: &str) -> anyhow::Result<OpenAISummarizer> {
        let openai = openai::OpenAICustomizedClient::new(openai_api_key)?;
        Ok(OpenAISummarizer {
            openai
        })
    }
}

impl Summarizer for OpenAISummarizer {
    fn summarize(&self, webvtt: &str) -> anyhow::Result<String> {
        let chat_messages = vec![
            openai::Message {
                role: "system".to_string(),
                content: "
                Please summarize the main discussions and conclusions of this
                meeting and organize the result in Markdown format. Specifically,
                present the title as a section header on the first line, followed
                by the content in bullet point format. The purpose is to make
                the content easily comprehensible for later review.
                Output text must be in Japanese.
                If the content doesn't contain any meaningful discussion, just output `NO_CONTENT`.
            ".trim().to_string(),
            },
            openai::Message {
                role: "user".to_string(),
                content: webvtt.to_string(),
            }
        ];

        // TODO cleanup webvtt before post
        // TODO split into multiple parts if it's too large
        self.openai.chat_completion(&openai::ChatCompletionRequest {
            model: "gpt-4-32k".to_string(),
            messages: chat_messages,
        }).map(|resp| {
            resp.choices[0].message.content.clone()
        }).map_err(|err| {
            anyhow!("Cannot generate summary from vtt file: {}",err)
        })
    }
}
