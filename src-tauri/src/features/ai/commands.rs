use super::{
    error::AiError,
    model::OllamaModel,
    prompts::{build_prompt, clean_mermaid_response},
};
use crate::features::settings::{commands::service as settings_service, model::AppSettings};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use std::future::Future;
use tauri::AppHandle;

#[tauri::command]
pub async fn list_ollama_models() -> Result<Vec<OllamaModel>, AiError> {
    let models = Ollama::default()
        .list_local_models()
        .await
        .map_err(AiError::ollama)?;

    Ok(models
        .into_iter()
        .map(|model| OllamaModel { name: model.name })
        .collect())
}

#[tauri::command]
pub async fn generate_diagram(app: AppHandle, description: String) -> Result<String, AiError> {
    let settings = settings_service(&app)
        .map_err(|error| AiError::new(error.code, error.message))?
        .get()
        .map_err(|error| AiError::new(error.code, error.message))?;

    generate_with(settings, &description, |model, prompt| async move {
        let response = Ollama::default()
            .generate(GenerationRequest::new(model, prompt))
            .await
            .map_err(AiError::ollama)?;
        Ok(response.response)
    })
    .await
}

pub(crate) async fn generate_with<F, Fut>(
    settings: AppSettings,
    description: &str,
    generate: F,
) -> Result<String, AiError>
where
    F: FnOnce(String, String) -> Fut,
    Fut: Future<Output = Result<String, AiError>>,
{
    if !settings.ollama.enabled {
        return Err(AiError::new(
            "ollamaDisabled",
            "Ollama is disabled in settings.",
        ));
    }

    let model = settings
        .ollama
        .model
        .filter(|model| !model.trim().is_empty())
        .ok_or_else(|| AiError::new("modelNotSelected", "Select an Ollama model in settings."))?;
    let prompt = build_prompt(description.trim())?;
    let response = generate(model, prompt).await?;

    clean_mermaid_response(&response)
}
