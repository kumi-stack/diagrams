use super::{
    error::SettingsError,
    model::{AppSettings, DiagramConfigOverrides},
};
use crate::utils::filesystem::{
    create_directory_all, path_exists, read_text_file, write_text_file,
};
use std::path::PathBuf;

pub struct SettingsService {
    root: PathBuf,
}

impl SettingsService {
    pub fn new(root: PathBuf) -> Result<Self, SettingsError> {
        create_directory_all(&root)
            .map_err(|error| SettingsError::io("create application directory", error))?;
        Ok(Self { root })
    }

    pub fn get(&self) -> Result<AppSettings, SettingsError> {
        let path = self.path();
        if !path_exists(&path).map_err(|error| SettingsError::io("inspect config.json", error))? {
            return Ok(AppSettings::default());
        }

        let content =
            read_text_file(&path).map_err(|error| SettingsError::io("read config.json", error))?;
        let settings: AppSettings = serde_json::from_str(&content).map_err(|error| {
            SettingsError::new(
                "invalidConfig",
                format!("Could not parse config.json: {error}"),
            )
        })?;
        settings
            .diagram_defaults
            .validate()
            .map_err(|message| SettingsError::new("invalidConfig", message))?;
        Ok(settings)
    }

    pub fn save(&self, settings: &AppSettings) -> Result<(), SettingsError> {
        settings
            .diagram_defaults
            .validate()
            .map_err(|message| SettingsError::new("invalidConfig", message))?;
        let content = serde_json::to_string_pretty(settings)
            .map_err(|error| SettingsError::io("serialize settings", error))?;
        write_text_file(&self.path(), &format!("{content}\n"))
            .map_err(|error| SettingsError::io("write config.json", error))
    }

    pub fn save_diagram_defaults(
        &self,
        defaults: DiagramConfigOverrides,
    ) -> Result<AppSettings, SettingsError> {
        defaults
            .validate()
            .map_err(|message| SettingsError::new("invalidConfig", message))?;
        let mut settings = self.get()?;
        settings.diagram_defaults = defaults;
        self.save(&settings)?;
        Ok(settings)
    }

    fn path(&self) -> PathBuf {
        self.root.join("config.json")
    }
}
