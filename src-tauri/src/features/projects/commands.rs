use super::{
    error::ProjectError,
    model::{CreateEntryRequest, EntryKind, Project, RenameEntryRequest, TreeNode},
    service::ProjectService,
};
use tauri::{AppHandle, Manager};

fn service(app: &AppHandle) -> Result<ProjectService, ProjectError> {
    let home = app
        .path()
        .home_dir()
        .map_err(|error| ProjectError::io("resolve home directory", error))?;
    ProjectService::new(home.join(".arch-diagrams"))
}

#[tauri::command]
pub fn list_projects(app: AppHandle) -> Result<Vec<Project>, ProjectError> {
    service(&app)?.list_projects()
}

#[tauri::command]
pub fn create_project(app: AppHandle, name: String) -> Result<Project, ProjectError> {
    service(&app)?.create_project(&name)
}

#[tauri::command]
pub fn list_project_tree(app: AppHandle, project: String) -> Result<Vec<TreeNode>, ProjectError> {
    service(&app)?.list_project_tree(&project)
}

#[tauri::command]
pub fn read_diagram(app: AppHandle, project: String, path: String) -> Result<String, ProjectError> {
    service(&app)?.read_diagram(&project, &path)
}

#[tauri::command]
pub fn write_diagram(
    app: AppHandle,
    project: String,
    path: String,
    content: String,
) -> Result<(), ProjectError> {
    service(&app)?.write_diagram(&project, &path, &content)
}

#[tauri::command]
pub fn create_entry(
    app: AppHandle,
    project: String,
    request: CreateEntryRequest,
) -> Result<TreeNode, ProjectError> {
    service(&app)?.create_entry(&project, request)
}

#[tauri::command]
pub fn rename_entry(
    app: AppHandle,
    project: String,
    request: RenameEntryRequest,
) -> Result<String, ProjectError> {
    service(&app)?.rename_entry(&project, request)
}

#[tauri::command]
pub fn delete_entry(
    app: AppHandle,
    project: String,
    path: String,
    kind: EntryKind,
) -> Result<(), ProjectError> {
    service(&app)?.delete_entry(&project, &path, kind)
}
