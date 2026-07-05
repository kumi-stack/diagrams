mod features;
mod shell;
mod utils;

use features::{ai, export, projects::commands, settings};
use shell::commands as shell_commands;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(shell::AppState::default())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .setup(|app| {
            shell::setup(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<shell::AppState>();
                if shell::should_hide_on_close(window.label(), state.is_exiting()) {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_projects,
            commands::create_project,
            commands::list_project_tree,
            commands::read_diagram,
            commands::write_diagram,
            commands::get_project_diagram_config,
            commands::save_project_diagram_defaults,
            commands::save_diagram_overrides,
            commands::create_entry,
            commands::rename_entry,
            commands::move_entry_to_trash,
            settings::commands::get_settings,
            settings::commands::save_settings,
            settings::commands::save_global_diagram_defaults,
            ai::commands::list_ollama_models,
            ai::commands::generate_diagram,
            export::commands::inspect_diagram_png,
            export::commands::save_diagram_png,
            export::commands::copy_diagram_png,
            shell_commands::set_current_project,
            shell_commands::get_current_project,
            shell_commands::finish_quick_add,
            shell_commands::cancel_quick_add
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
