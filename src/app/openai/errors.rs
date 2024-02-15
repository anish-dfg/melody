use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum OpenAIError {
    #[error("unable to retrieve chat completion. error: {0}")]
    CreateChat(String),
    #[error("unable to serialize response. error: {0}")]
    Serialize(String),
}
