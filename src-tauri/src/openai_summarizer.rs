use anyhow::anyhow;
use regex::Regex;
use tiktoken_rs::{cl100k_base, CoreBPE};
use crate::openai;
use crate::openai::OpenAICustomizedClient;
use crate::summarizer::Summarizer;

pub struct OpenAISummarizer {
    openai: OpenAICustomizedClient,
    pub timecode_regex: Regex,
}

impl OpenAISummarizer {
    pub fn new (openai_api_key: &str) -> anyhow::Result<OpenAISummarizer> {
        let openai = openai::OpenAICustomizedClient::new(openai_api_key)?;
        let timecode_regex = Regex::new(r"^\d{2}:\d{2}:\d{2}\.\d{3} --> \d{2}:\d{2}:\d{2}\.\d{3}$")?;
        Ok(OpenAISummarizer {
            openai,
            timecode_regex,
        })
    }
}

impl Summarizer for OpenAISummarizer {
    fn summarize(&self, src: &str) -> anyhow::Result<String> {
        let bpe = cl100k_base()?;

        let chunks = self.split(bpe, src, 20000);
        let mut buffer = String::new();
        for chunk in chunks {
            match self.do_summarize(chunk.as_str()) {
                Ok(result) => {
                    buffer += &*(result + "\n\n");
                }
                Err(err) => {
                    log::error!("Cannot summarized by openai: {:?}", err);
                }
            }
        }

        Ok(buffer)
    }
}

impl OpenAISummarizer {
    fn split(&self, bpe: CoreBPE, src: &str, chunk_size: usize) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_token_count = 0;

        for line in src.lines() {
            if line.is_empty() {
                continue;
            }
            if self.timecode_regex.is_match(line) {
                continue;
            }

            let tokens = bpe.encode_with_special_tokens(line);
            if current_token_count + tokens.len() > chunk_size {
                chunks.push(current_chunk);
                current_chunk = String::new();
                current_token_count = 0;
            }

            current_chunk.push_str(line);
            current_chunk.push('\n');
            current_token_count += tokens.len();
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }


    fn do_summarize(&self, content: &str) -> anyhow::Result<String> {
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
            ".trim().to_string(),
                // If the content doesn't contain any meaningful discussion, just output `NO_CONTENT`.
            },
            openai::Message {
                role: "user".to_string(),
                content: content.to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_into_chunks() {
        let vtt_content = r#"
00:00:00.000 --> 00:00:02.000
This is the first line.

00:00:02.000 --> 00:00:04.000
This is the second line.

00:00:04.000 --> 00:00:06.000
This is the third line.
"#;

        let summarizer = OpenAISummarizer::new("test")
            .unwrap();
        let got = summarizer.split(cl100k_base().unwrap(), vtt_content, 15);
        assert_eq!(got, vec![
            "This is the first line.\nThis is the second line.\n",
            "This is the third line.\n"
        ])
    }
}
