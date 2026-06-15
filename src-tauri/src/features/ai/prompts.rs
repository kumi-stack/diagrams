use super::error::AiError;
use serde::Deserialize;

const PROMPTS_JSON: &str = include_str!("../../../resources/prompts.json");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DiagramPrompts {
    system: String,
    user_template: String,
}

pub fn build_prompt(description: &str) -> Result<String, AiError> {
    let prompts: DiagramPrompts = serde_json::from_str(PROMPTS_JSON).map_err(|error| {
        AiError::new(
            "invalidPrompts",
            format!("The bundled prompts file is invalid: {error}"),
        )
    })?;

    Ok(format!(
        "{}\n\n{}",
        prompts.system,
        prompts
            .user_template
            .replace("{{description}}", description)
    ))
}

pub fn clean_mermaid_response(response: &str) -> Result<String, AiError> {
    let trimmed = response.trim();
    let without_fence = if trimmed.starts_with("```") && trimmed.ends_with("```") {
        let body = trimmed
            .strip_prefix("```")
            .and_then(|value| value.strip_suffix("```"))
            .unwrap_or(trimmed)
            .trim();
        body.strip_prefix("mermaid")
            .or_else(|| body.strip_prefix("mmd"))
            .unwrap_or(body)
            .trim()
    } else {
        trimmed
    };

    if without_fence.is_empty() {
        return Err(AiError::new(
            "emptyResponse",
            "Ollama returned an empty diagram.",
        ));
    }

    Ok(without_fence.to_owned())
}
