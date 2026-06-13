mod features;
mod utils;

use features::{export, projects::commands};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![
            commands::list_projects,
            commands::create_project,
            commands::list_project_tree,
            commands::read_diagram,
            commands::write_diagram,
            commands::create_entry,
            commands::rename_entry,
            commands::delete_entry,
            export::commands::inspect_diagram_png,
            export::commands::save_diagram_png,
            export::commands::copy_diagram_png
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
