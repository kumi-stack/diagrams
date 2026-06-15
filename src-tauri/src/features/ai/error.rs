use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AiError {
    pub code: &'static str,
    pub message: String,
}

impl AiError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn ollama(error: impl std::fmt::Display) -> Self {
        Self::new(
            "ollamaUnavailable",
            format!("Could not connect to Ollama at localhost:11434: {error}"),
        )
    }
}
