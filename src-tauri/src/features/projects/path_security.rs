use super::{error::ProjectError, model::EntryKind};
use crate::utils::filesystem::{canonicalize, entry_type, EntryType};
use std::path::{Component, Path, PathBuf};

pub fn validate_name(name: &str, kind: EntryKind) -> Result<(), ProjectError> {
    if name.is_empty() || name.trim() != name {
        return Err(ProjectError::invalid_name(
            "Names cannot be empty or start/end with whitespace.",
        ));
    }
    if name == "." || name == ".." || name.contains('/') || name.contains('\\') {
        return Err(ProjectError::invalid_name(
            "Names cannot be '.', '..', or contain path separators.",
        ));
    }
    if name.chars().any(char::is_control) {
        return Err(ProjectError::invalid_name(
            "Names cannot contain control characters.",
        ));
    }
    if kind == EntryKind::File && !name.to_ascii_lowercase().ends_with(".mmd") {
        return Err(ProjectError::invalid_name(
            "Diagram filenames must end with .mmd.",
        ));
    }
    Ok(())
}

pub fn validate_relative_path(path: &str) -> Result<PathBuf, ProjectError> {
    if path.is_empty() {
        return Err(ProjectError::invalid_path("An entry path is required."));
    }
    let path = Path::new(path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(ProjectError::invalid_path(
            "Only normalized project-relative paths are allowed.",
        ));
    }
    Ok(path.to_path_buf())
}

pub fn ensure_parent_contained(project_root: &Path, parent: &Path) -> Result<(), ProjectError> {
    let parent =
        canonicalize(parent).map_err(|error| ProjectError::io("resolve parent folder", error))?;
    if !parent.starts_with(project_root) {
        return Err(ProjectError::invalid_path(
            "The parent folder escapes the selected project.",
        ));
    }
    Ok(())
}

pub fn reject_symlink_components(project_root: &Path, relative: &Path) -> Result<(), ProjectError> {
    let mut current = project_root.to_path_buf();
    for component in relative.components() {
        current.push(component.as_os_str());
        let current_type = entry_type(&current)
            .map_err(|error| ProjectError::io("inspect project path", error))?;
        if current_type == EntryType::Symlink {
            return Err(ProjectError::invalid_path(
                "Symbolic links are not allowed.",
            ));
        }
    }
    Ok(())
}

pub fn validate_diagram_path(path: &Path) -> Result<(), ProjectError> {
    if is_diagram_path(path) {
        Ok(())
    } else {
        Err(ProjectError::invalid_path(
            "Only .mmd diagram files can be opened.",
        ))
    }
}

pub fn is_diagram_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("mmd"))
}

pub fn relative_path(root: &Path, path: &Path) -> Result<String, ProjectError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| ProjectError::invalid_path("The entry is outside the project."))?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}
