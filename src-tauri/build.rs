fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "list_projects",
            "create_project",
            "list_project_tree",
            "read_diagram",
            "write_diagram",
            "create_entry",
            "rename_entry",
            "delete_entry",
            "get_settings",
            "save_settings",
            "list_ollama_models",
            "generate_diagram",
            "inspect_diagram_png",
            "save_diagram_png",
            "copy_diagram_png",
            "set_current_project",
            "get_current_project",
            "finish_quick_add",
            "cancel_quick_add",
        ]),
    ))
    .expect("failed to build Tauri application");
}
