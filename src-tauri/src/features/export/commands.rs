use super::{
    error::ExportError,
    model::{PngMetadata, PngOptions},
    renderer,
};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub async fn inspect_diagram_png(
    svg: String,
    options: PngOptions,
) -> Result<PngMetadata, ExportError> {
    tauri::async_runtime::spawn_blocking(move || renderer::inspect(&svg, options))
        .await
        .map_err(|error| ExportError::new("render_task_failed", error.to_string()))?
}

#[tauri::command]
pub async fn save_diagram_png(
    svg: String,
    path: PathBuf,
    options: PngOptions,
) -> Result<PngMetadata, ExportError> {
    tauri::async_runtime::spawn_blocking(move || {
        let rendered = renderer::render(&svg, options)?;
        std::fs::write(&path, &rendered.encoded).map_err(|error| {
            ExportError::new(
                "write_failed",
                format!("Could not write {}: {error}", path.display()),
            )
        })?;
        Ok(rendered.metadata)
    })
    .await
    .map_err(|error| ExportError::new("render_task_failed", error.to_string()))?
}

#[tauri::command]
pub async fn copy_diagram_png(
    app: AppHandle,
    svg: String,
    options: PngOptions,
) -> Result<PngMetadata, ExportError> {
    let rendered = tauri::async_runtime::spawn_blocking(move || renderer::render(&svg, options))
        .await
        .map_err(|error| ExportError::new("render_task_failed", error.to_string()))??;

    let image = tauri::image::Image::new_owned(
        rendered.rgba,
        rendered.metadata.width,
        rendered.metadata.height,
    );
    app.clipboard()
        .write_image(&image)
        .map_err(|error| ExportError::new("clipboard_failed", error.to_string()))?;

    Ok(rendered.metadata)
}
