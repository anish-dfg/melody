use reqwest::Client;

#[derive(Clone)]
pub struct OpenAIClient {
    pub api_key: String,
    pub base_uri: String,
    pub client: Client,
}

impl OpenAIClient {
    pub fn new(api_key: &str, base_uri: &str) -> Self {
        OpenAIClient {
            api_key: api_key.to_owned(),
            base_uri: base_uri.to_string(),
            client: Client::new(),
        }
    }

    pub fn with_client(api_key: &str, base_uri: &str, client: Client) -> Self {
        OpenAIClient {
            api_key: api_key.into(),
            base_uri: base_uri.into(),
            client,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::app::{openai::OpenAIClient, util::test_util};

    #[tokio::test]
    pub async fn test_create_client() {
        test_util::init();
        let api_key =
            env::var("OPENAI_API_KEY").expect("environment variable OPENAI_API_KEY not defined");
        let _ = OpenAIClient::new(&api_key, "https://api.openai.com/v1");
    }
}
