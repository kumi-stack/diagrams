use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SettingsError {
    pub code: &'static str,
    pub message: String,
}

impl SettingsError {
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn io(action: &str, error: impl std::fmt::Display) -> Self {
        Self::new("io", format!("Failed to {action}: {error}"))
    }
}
