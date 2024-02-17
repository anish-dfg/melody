use std::collections::HashMap;
use std::error::Error;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use super::errors::OpenAIError;
use super::OpenAIClient;

use super::usage::Usage;

#[derive(Debug, Serialize, Deserialize)]
pub enum ChatRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "function")]
    Function,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    role: ChatRole,
    content: String,
    name: Option<String>,
    function_call: Option<Value>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatFunction {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Value,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatOptions {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub functions: Option<Vec<ChatFunction>>,
    pub function_call: Option<Value>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<u32>,
    pub stream: Option<bool>,
    pub stop: Option<[String; 4]>,
    pub max_tokens: u64,
    pub presence_penalty: Option<i8>,
    pub frequency_penalty: Option<i8>,
    pub logit_bias: Option<HashMap<String, i8>>,
    pub user: Option<String>,
}

impl ChatOptions {
    pub fn default(model: &str, messages: Vec<ChatMessage>, max_tokens: u64) -> Self {
        Self {
            model: model.to_owned(),
            messages,
            functions: None,
            function_call: None,
            temperature: Some(1.0),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            stop: None,
            max_tokens,
            presence_penalty: Some(0),
            frequency_penalty: Some(0),
            logit_bias: None,
            user: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponseMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatResponseChoice {
    pub index: u64,
    pub message: ChatResponseMessage,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletion {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChatResponseChoice>,
    pub usage: Usage,
}

impl OpenAIClient {
    pub async fn get_chat_completion(
        &self,
        opts: &ChatOptions,
    ) -> Result<ChatCompletion, OpenAIError> {
        let uri = self.base_uri.clone() + "/chat/completions";
        let api_key = &self.api_key;
        let res = self
            .client
            .post(&uri)
            .header("Authorization", format!("Bearer {api_key}"))
            .json(&opts)
            .send()
            .await
            .map_err(|e| {
                log::error!("{}", e.to_string());
                OpenAIError::CreateChat(e.to_string())
            })?;
        let completion = res.json().await.map_err(|e| {
            log::error!("{}", e.to_string());
            OpenAIError::Serialize(e.to_string())
        })?;
        Ok(completion)
    }
}

#[cfg(test)]
mod tests {

    use crate::app::util::test_util;

    use super::*;
    use std::env;

    #[tokio::test]
    pub async fn test_chat_completion() {
        test_util::init();
        let api_key = env::var("OPENAI_API_KEY").expect("error loading API key");
        let client = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
        let x = ChatMessage {
            role: ChatRole::System,
            name: None,
            content: "you are a helpful assistant".to_owned(),
            function_call: None,
        };

        println!("{:#?}", x);
        let completion = client
            .get_chat_completion(&ChatOptions::default(
                "gpt-3.5-turbo",
                vec![ChatMessage {
                    role: ChatRole::System,
                    name: None,
                    content: "complete the lyric: scar tissue that i wish you saw...".to_owned(),
                    function_call: None,
                }],
                20,
            ))
            .await
            .expect("error fetching chat completion");
        println!("{:#?}", completion);
    }
}
