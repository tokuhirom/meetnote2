use reqwest::blocking::{Client, multipart};
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatChoice {
    index: i32,
    message: Message,
    finish_reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionUsage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatCompletionResponse {
    id: String,
    #[serde(rename = "object")]
    object_field: String,
    created: u64,
    model: String,
    system_fingerprint: Option<String>,
    choices: Vec<ChatChoice>,
    usage: ChatCompletionUsage,
}

pub struct OpenAICustomizedClient {
    open_ai_api_key: String,
    client: Client,
}

impl OpenAICustomizedClient {
    pub fn new(open_ai_api_key: &str) -> OpenAICustomizedClient {
        OpenAICustomizedClient {
            open_ai_api_key: open_ai_api_key.to_string(),
            client: Client::new(),
        }
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

        let mut res = self.client.post("https://api.openai.com/v1/audio/transcriptions")
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
        let client = Client::new();
        let res = self.client.post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.open_ai_api_key)
            .json(request)
            .send()?;

        if res.status() == StatusCode::OK {
            Ok(res.json()?)
        } else {
            Err(anyhow::anyhow!("Chat completion failed. Status: {:?}", res.status()))
        }
    }
}