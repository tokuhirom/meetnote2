use reqwest::blocking::{Client, ClientBuilder, multipart};
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<Message>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct ChatChoice {
    pub index: i32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    #[serde(rename = "object")]
    pub object_field: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<ChatChoice>,
    pub usage: ChatCompletionUsage,
}

pub struct OpenAICustomizedClient {
    open_ai_api_key: String,
    client: Client,
}

impl OpenAICustomizedClient {
    pub fn new(open_ai_api_key: &str) -> anyhow::Result<OpenAICustomizedClient> {
        Ok(OpenAICustomizedClient {
            open_ai_api_key: open_ai_api_key.to_string(),
            client: ClientBuilder::new()
                .timeout(Duration::from_secs(10*60))
                .build()?
        })
    }

    pub fn transcript(&self, file_path: &str, language: &str) -> Result<String> {
        let mut buffer = Vec::new();
        File::open(file_path)?.read_to_end(&mut buffer)?;

        let part = multipart::Part::bytes(buffer)
            .mime_str("audio/mpeg")?
            .file_name(file_path.to_string());

        let form = multipart::Form::new()
            .text("model", "whisper-1")
            .text("language", language.to_string())
            .text("response_format", "vtt")
            .part("file", part);

        let res = self.client.post("https://api.openai.com/v1/audio/transcriptions")
            .bearer_auth(&self.open_ai_api_key)
            .multipart(form)
            .send()?;

        if res.status() == StatusCode::OK {
            Ok(res.text()?)
        } else {
            Err(anyhow::anyhow!("Transcription failed. Status: {:?}", res.status()))
        }
    }

    pub fn chat_completion(&self, request: &ChatCompletionRequest) -> Result<ChatCompletionResponse> {
        let res = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.open_ai_api_key)
            .json(request)
            .send()?;

        if res.status() == StatusCode::OK {
            Ok(res.json()?)
        } else {
            Err(anyhow::anyhow!(
                "Chat completion failed. Status: status={:?}, headers={:?}, text={:?}",
                res.status(),
                res.headers(),
                res.text()
            ))
        }
    }
}
