use super::{
    commands::generate_with,
    error::AiError,
    prompts::{build_prompt, clean_mermaid_response},
};
use crate::features::settings::model::{AppSettings, OllamaSettings};

fn enabled_settings() -> AppSettings {
    AppSettings {
        ollama: OllamaSettings {
            enabled: true,
            model: Some("test-model:latest".into()),
        },
        ..Default::default()
    }
}

#[test]
fn builds_prompt_from_bundled_template() {
    let prompt = build_prompt("API connected to PostgreSQL").unwrap();

    assert!(prompt.contains("API connected to PostgreSQL"));
    assert!(prompt.contains("Mermaid"));
}

#[test]
fn removes_mermaid_markdown_fences() {
    assert_eq!(
        clean_mermaid_response("```mermaid\nflowchart LR\n  A --> B\n```").unwrap(),
        "flowchart LR\n  A --> B"
    );
    assert_eq!(
        clean_mermaid_response("sequenceDiagram\n  A->>B: Hello").unwrap(),
        "sequenceDiagram\n  A->>B: Hello"
    );
}

#[test]
fn rejects_empty_response() {
    assert_eq!(
        clean_mermaid_response("  ").unwrap_err().code,
        "emptyResponse"
    );
}

#[test]
fn generates_with_selected_model_and_cleans_response() {
    let result = tauri::async_runtime::block_on(generate_with(
        enabled_settings(),
        "A calls B",
        |model, prompt| async move {
            assert_eq!(model, "test-model:latest");
            assert!(prompt.contains("A calls B"));
            Ok("```mermaid\nflowchart LR\n  A --> B\n```".into())
        },
    ))
    .unwrap();

    assert_eq!(result, "flowchart LR\n  A --> B");
}

#[test]
fn rejects_disabled_ollama_without_calling_client() {
    let error = tauri::async_runtime::block_on(generate_with(
        AppSettings::default(),
        "diagram",
        |_, _| async { panic!("client should not be called") },
    ))
    .unwrap_err();

    assert_eq!(error.code, "ollamaDisabled");
}

#[test]
fn rejects_missing_model_without_calling_client() {
    let settings = AppSettings {
        ollama: OllamaSettings {
            enabled: true,
            model: None,
        },
        ..Default::default()
    };
    let error = tauri::async_runtime::block_on(generate_with(settings, "diagram", |_, _| async {
        panic!("client should not be called")
    }))
    .unwrap_err();

    assert_eq!(error.code, "modelNotSelected");
}

#[test]
fn propagates_client_errors() {
    let error = tauri::async_runtime::block_on(generate_with(
        enabled_settings(),
        "diagram",
        |_, _| async { Err(AiError::new("testError", "Generation failed")) },
    ))
    .unwrap_err();

    assert_eq!(error.code, "testError");
}
