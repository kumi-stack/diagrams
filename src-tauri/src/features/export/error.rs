use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub struct ExportError {
    pub code: &'static str,
    pub message: String,
}

impl ExportError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for ExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ExportError {}
