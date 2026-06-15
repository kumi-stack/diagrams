use super::{model::AppSettings, service::SettingsService};
use std::fs;
use tempfile::TempDir;

#[test]
fn returns_default_settings_without_a_config_file() {
    let temp = TempDir::new().unwrap();
    let service = SettingsService::new(temp.path().join("app")).unwrap();

    assert_eq!(service.get().unwrap(), AppSettings::default());
}

#[test]
fn saves_and_loads_selected_model() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("app");
    let service = SettingsService::new(root.clone()).unwrap();
    let settings = AppSettings {
        ollama: super::model::OllamaSettings {
            enabled: true,
            model: Some("qwen2.5-coder:7b".into()),
        },
    };

    service.save(&settings).unwrap();

    assert_eq!(service.get().unwrap(), settings);
    let content = fs::read_to_string(root.join("config.json")).unwrap();
    assert!(content.contains("\"enabled\": true"));
    assert!(content.contains("\"model\": \"qwen2.5-coder:7b\""));
}

#[test]
fn reports_invalid_json() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("app");
    let service = SettingsService::new(root.clone()).unwrap();
    fs::write(root.join("config.json"), "{ invalid").unwrap();

    let error = service.get().unwrap_err();
    assert_eq!(error.code, "invalidConfig");
    assert!(error.message.contains("config.json"));
}
