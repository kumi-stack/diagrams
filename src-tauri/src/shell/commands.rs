use super::{hide_window, show_window, AppState, MAIN_WINDOW, QUICK_ADD_WINDOW};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, State};

pub const QUICK_ADD_CREATED_EVENT: &str = "quick-add-created";

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickAddResult {
    pub project: String,
    pub path: String,
    pub warning: Option<String>,
}

#[tauri::command]
pub fn set_current_project(state: State<'_, AppState>, project: Option<String>) {
    state.set_current_project(project);
}

#[tauri::command]
pub fn get_current_project(state: State<'_, AppState>) -> Option<String> {
    state.current_project()
}

#[tauri::command]
pub fn finish_quick_add(app: AppHandle, result: QuickAddResult) -> Result<(), String> {
    hide_window(&app, QUICK_ADD_WINDOW);
    show_window(&app, MAIN_WINDOW);
    app.emit_to(MAIN_WINDOW, QUICK_ADD_CREATED_EVENT, result)
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn cancel_quick_add(app: AppHandle) {
    hide_window(&app, QUICK_ADD_WINDOW);
}
