use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DiagramTheme {
    Studio,
    Default,
    Base,
    Dark,
    Forest,
    Neutral,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DiagramFontFamily {
    JetbrainsMono,
    SystemSans,
    SystemSerif,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DiagramBackground {
    Transparent,
    White,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MermaidLook {
    Classic,
    HandDrawn,
    Neo,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum MermaidCurve {
    Basis,
    Linear,
    Rounded,
    Step,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommonDiagramConfigOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub theme: Option<DiagramTheme>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_family: Option<DiagramFontFamily>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<DiagramBackground>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MermaidConfigOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub look: Option<MermaidLook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve: Option<MermaidCurve>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiagramTypeConfigOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mermaid: Option<MermaidConfigOverrides>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DiagramConfigOverrides {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common: Option<CommonDiagramConfigOverrides>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<DiagramTypeConfigOverrides>,
}

impl DiagramConfigOverrides {
    pub fn validate(&self) -> Result<(), &'static str> {
        Ok(())
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    #[serde(default)]
    pub ollama: OllamaSettings,
    #[serde(default)]
    pub diagram_defaults: DiagramConfigOverrides,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OllamaSettings {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub model: Option<String>,
}
