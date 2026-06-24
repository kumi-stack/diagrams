use super::{
    error::SettingsError,
    model::{AppSettings, DiagramConfigOverrides},
    service::SettingsService,
};
use tauri::{AppHandle, Manager};

pub(crate) fn service(app: &AppHandle) -> Result<SettingsService, SettingsError> {
    let home = app
        .path()
        .home_dir()
        .map_err(|error| SettingsError::io("resolve home directory", error))?;
    SettingsService::new(home.join(".arch-diagrams"))
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Result<AppSettings, SettingsError> {
    service(&app)?.get()
}

#[tauri::command]
pub fn save_settings(app: AppHandle, settings: AppSettings) -> Result<AppSettings, SettingsError> {
    service(&app)?.save(&settings)?;
    Ok(settings)
}

#[tauri::command]
pub fn save_global_diagram_defaults(
    app: AppHandle,
    defaults: DiagramConfigOverrides,
) -> Result<AppSettings, SettingsError> {
    service(&app)?.save_diagram_defaults(defaults)
}
