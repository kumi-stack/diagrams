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
            "inspect_diagram_png",
            "save_diagram_png",
            "copy_diagram_png",
        ]),
    ))
    .expect("failed to build Tauri application");
}
