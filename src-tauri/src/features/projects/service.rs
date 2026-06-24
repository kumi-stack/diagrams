use super::{
    error::ProjectError,
    model::{
        compare_names, compare_nodes, CreateEntryRequest, EntryKind, Project, ProjectDiagramConfig,
        RenameEntryRequest, TreeNode,
    },
    path_security::{
        ensure_parent_contained, is_diagram_path, reject_symlink_components, relative_path,
        validate_diagram_path, validate_name, validate_relative_path,
    },
};
use crate::features::settings::model::DiagramConfigOverrides;
use crate::utils::filesystem::{
    canonicalize, create_directory, create_directory_all, create_file, delete_directory_all,
    delete_file, entry_type, path_exists, read_directory, read_text_file, rename, write_text_file,
    EntryType,
};
use std::path::{Path, PathBuf};

const PROJECT_CONFIG_FILE: &str = ".arch-diagrams.json";

const STARTER_DIAGRAM: &str = r#"flowchart LR
  A[Client] --> B[API Gateway]
  B --> C{Authenticated?}
  C -->|Yes| D[Application]
  C -->|No| E[Sign in]
  D --> F[(Database)]
  D --> G[Message Queue]
  G --> H[Worker]"#;

pub struct ProjectService {
    root: PathBuf,
}

impl ProjectService {
    pub fn new(root: PathBuf) -> Result<Self, ProjectError> {
        create_directory_all(&root)
            .map_err(|error| ProjectError::io("create project root", error))?;
        let root_type =
            entry_type(&root).map_err(|error| ProjectError::io("inspect project root", error))?;
        if root_type != EntryType::Directory {
            return Err(ProjectError::invalid_path(
                "The project root must be a real directory.",
            ));
        }
        Ok(Self { root })
    }

    pub fn list_projects(&self) -> Result<Vec<Project>, ProjectError> {
        let mut projects = Vec::new();
        let entries =
            read_directory(&self.root).map_err(|error| ProjectError::io("list projects", error))?;
        for entry in entries {
            if entry.entry_type == EntryType::Directory {
                projects.push(Project { name: entry.name });
            }
        }
        projects.sort_by(|left, right| compare_names(&left.name, &right.name));
        Ok(projects)
    }

    pub fn create_project(&self, name: &str) -> Result<Project, ProjectError> {
        validate_name(name, EntryKind::Folder)?;
        let path = self.root.join(name);
        match create_directory(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(ProjectError::new(
                    "alreadyExists",
                    format!("A project named \"{name}\" already exists."),
                ));
            }
            Err(error) => return Err(ProjectError::io("create project", error)),
        }

        if let Err(error) = write_text_file(&path.join("diagram.mmd"), STARTER_DIAGRAM) {
            let _ = delete_directory_all(&path);
            return Err(ProjectError::io("create starter diagram", error));
        }

        Ok(Project {
            name: name.to_owned(),
        })
    }

    pub fn list_project_tree(&self, project: &str) -> Result<Vec<TreeNode>, ProjectError> {
        let project_root = self.project_root(project)?;
        self.read_tree(&project_root, &project_root)
    }

    pub fn read_diagram(&self, project: &str, path: &str) -> Result<String, ProjectError> {
        let project_root = self.project_root(project)?;
        let file = self.existing_entry(&project_root, path, EntryKind::File)?;
        validate_diagram_path(&file)?;
        read_text_file(&file).map_err(|error| ProjectError::io("read diagram", error))
    }

    pub fn write_diagram(
        &self,
        project: &str,
        path: &str,
        content: &str,
    ) -> Result<(), ProjectError> {
        let project_root = self.project_root(project)?;
        let file = self.existing_entry(&project_root, path, EntryKind::File)?;
        validate_diagram_path(&file)?;
        write_text_file(&file, content).map_err(|error| ProjectError::io("write diagram", error))
    }

    pub fn get_diagram_config(&self, project: &str) -> Result<ProjectDiagramConfig, ProjectError> {
        let project_root = self.project_root(project)?;
        self.read_diagram_config(&project_root)
    }

    pub fn save_project_diagram_defaults(
        &self,
        project: &str,
        defaults: DiagramConfigOverrides,
    ) -> Result<ProjectDiagramConfig, ProjectError> {
        defaults.validate().map_err(ProjectError::invalid_config)?;
        let project_root = self.project_root(project)?;
        let mut config = self.read_diagram_config(&project_root)?;
        config.defaults = defaults;
        self.write_diagram_config(&project_root, &config)?;
        Ok(config)
    }

    pub fn save_diagram_overrides(
        &self,
        project: &str,
        path: &str,
        overrides: DiagramConfigOverrides,
    ) -> Result<ProjectDiagramConfig, ProjectError> {
        overrides.validate().map_err(ProjectError::invalid_config)?;
        let project_root = self.project_root(project)?;
        let file = self.existing_entry(&project_root, path, EntryKind::File)?;
        validate_diagram_path(&file)?;
        let mut config = self.read_diagram_config(&project_root)?;
        if overrides.is_empty() {
            config.diagrams.remove(path);
        } else {
            config.diagrams.insert(path.to_owned(), overrides);
        }
        self.write_diagram_config(&project_root, &config)?;
        Ok(config)
    }

    pub fn create_entry(
        &self,
        project: &str,
        request: CreateEntryRequest,
    ) -> Result<TreeNode, ProjectError> {
        validate_name(&request.name, request.kind)?;
        let project_root = self.project_root(project)?;
        let parent = if request.parent_path.is_empty() {
            project_root.clone()
        } else {
            self.existing_entry(&project_root, &request.parent_path, EntryKind::Folder)?
        };
        let path = parent.join(&request.name);
        ensure_parent_contained(&project_root, &parent)?;

        let result = match request.kind {
            EntryKind::File => create_file(&path),
            EntryKind::Folder => create_directory(&path),
        };
        if let Err(error) = result {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                return Err(ProjectError::new(
                    "alreadyExists",
                    format!("An entry named \"{}\" already exists.", request.name),
                ));
            }
            return Err(ProjectError::io("create entry", error));
        }

        Ok(TreeNode {
            name: request.name,
            path: relative_path(&project_root, &path)?,
            kind: request.kind,
            children: Vec::new(),
        })
    }

    pub fn rename_entry(
        &self,
        project: &str,
        request: RenameEntryRequest,
    ) -> Result<String, ProjectError> {
        validate_name(&request.new_name, request.kind)?;
        let project_root = self.project_root(project)?;
        let source = self.existing_entry(&project_root, &request.path, request.kind)?;
        let mut config = self.read_diagram_config(&project_root)?;
        let parent = source
            .parent()
            .ok_or_else(|| ProjectError::invalid_path("Cannot rename the project root."))?;
        let destination = parent.join(&request.new_name);
        ensure_parent_contained(&project_root, parent)?;
        if path_exists(&destination)
            .map_err(|error| ProjectError::io("inspect rename destination", error))?
        {
            return Err(ProjectError::new(
                "alreadyExists",
                format!("An entry named \"{}\" already exists.", request.new_name),
            ));
        }

        rename(&source, &destination).map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                ProjectError::new(
                    "alreadyExists",
                    format!("An entry named \"{}\" already exists.", request.new_name),
                )
            } else {
                ProjectError::io("rename entry", error)
            }
        })?;
        let destination_relative = relative_path(&project_root, &destination)?;
        if Self::remap_diagram_configs(
            &mut config,
            &request.path,
            &destination_relative,
            request.kind,
        ) {
            self.write_diagram_config(&project_root, &config)?;
        }
        Ok(destination_relative)
    }

    pub fn delete_entry(
        &self,
        project: &str,
        path: &str,
        kind: EntryKind,
    ) -> Result<(), ProjectError> {
        let project_root = self.project_root(project)?;
        let entry = self.existing_entry(&project_root, path, kind)?;
        let mut config = self.read_diagram_config(&project_root)?;
        match kind {
            EntryKind::File => {
                validate_diagram_path(&entry)?;
                delete_file(&entry).map_err(|error| ProjectError::io("delete file", error))
            }
            EntryKind::Folder => delete_directory_all(&entry)
                .map_err(|error| ProjectError::io("delete folder", error)),
        }?;
        if Self::remove_diagram_configs(&mut config, path, kind) {
            self.write_diagram_config(&project_root, &config)?;
        }
        Ok(())
    }

    fn project_root(&self, project: &str) -> Result<PathBuf, ProjectError> {
        validate_name(project, EntryKind::Folder)?;
        let path = self.root.join(project);
        let project_type =
            entry_type(&path).map_err(|error| ProjectError::io("open project", error))?;
        if project_type != EntryType::Directory {
            return Err(ProjectError::invalid_path(
                "The selected project is not a real directory.",
            ));
        }
        let canonical_root = canonicalize(&self.root)
            .map_err(|error| ProjectError::io("resolve project root", error))?;
        let canonical_project =
            canonicalize(&path).map_err(|error| ProjectError::io("resolve project", error))?;
        if !canonical_project.starts_with(&canonical_root) {
            return Err(ProjectError::invalid_path(
                "The selected project escapes the project root.",
            ));
        }
        Ok(canonical_project)
    }

    fn existing_entry(
        &self,
        project_root: &Path,
        relative: &str,
        kind: EntryKind,
    ) -> Result<PathBuf, ProjectError> {
        let relative = validate_relative_path(relative)?;
        reject_symlink_components(project_root, &relative)?;
        let joined = project_root.join(relative);
        let actual_type =
            entry_type(&joined).map_err(|error| ProjectError::io("open project entry", error))?;
        if actual_type == EntryType::Symlink {
            return Err(ProjectError::invalid_path(
                "Symbolic links are not allowed.",
            ));
        }
        let type_matches = match kind {
            EntryKind::File => actual_type == EntryType::File,
            EntryKind::Folder => actual_type == EntryType::Directory,
        };
        if !type_matches {
            return Err(ProjectError::invalid_path("The entry type does not match."));
        }
        let canonical = canonicalize(&joined)
            .map_err(|error| ProjectError::io("resolve project entry", error))?;
        if !canonical.starts_with(project_root) || canonical == project_root {
            return Err(ProjectError::invalid_path(
                "The entry escapes the selected project.",
            ));
        }
        Ok(canonical)
    }

    fn read_tree(
        &self,
        project_root: &Path,
        directory: &Path,
    ) -> Result<Vec<TreeNode>, ProjectError> {
        let mut nodes = Vec::new();
        let entries = read_directory(directory)
            .map_err(|error| ProjectError::io("read project folder", error))?;
        for entry in entries {
            if entry.entry_type == EntryType::Symlink {
                return Err(ProjectError::invalid_path(format!(
                    "Symbolic links are not allowed: {}",
                    entry.path.display()
                )));
            }
            if entry.entry_type == EntryType::Directory {
                nodes.push(TreeNode {
                    name: entry.name,
                    path: relative_path(project_root, &entry.path)?,
                    kind: EntryKind::Folder,
                    children: self.read_tree(project_root, &entry.path)?,
                });
            } else if entry.entry_type == EntryType::File && is_diagram_path(&entry.path) {
                nodes.push(TreeNode {
                    name: entry.name,
                    path: relative_path(project_root, &entry.path)?,
                    kind: EntryKind::File,
                    children: Vec::new(),
                });
            }
        }
        nodes.sort_by(compare_nodes);
        Ok(nodes)
    }

    fn read_diagram_config(
        &self,
        project_root: &Path,
    ) -> Result<ProjectDiagramConfig, ProjectError> {
        let path = project_root.join(PROJECT_CONFIG_FILE);
        if !path_exists(&path).map_err(|error| ProjectError::io("inspect project config", error))? {
            return Ok(ProjectDiagramConfig::default());
        }
        let content = read_text_file(&path)
            .map_err(|error| ProjectError::io("read project config", error))?;
        let config: ProjectDiagramConfig = serde_json::from_str(&content).map_err(|error| {
            ProjectError::invalid_config(format!("Could not parse {PROJECT_CONFIG_FILE}: {error}"))
        })?;
        if config.version != 1 {
            return Err(ProjectError::invalid_config(format!(
                "Unsupported project config version {}.",
                config.version
            )));
        }
        config
            .defaults
            .validate()
            .map_err(ProjectError::invalid_config)?;
        for (path, overrides) in &config.diagrams {
            let path_buf = validate_relative_path(path)?;
            validate_diagram_path(&path_buf)?;
            overrides.validate().map_err(ProjectError::invalid_config)?;
        }
        Ok(config)
    }

    fn write_diagram_config(
        &self,
        project_root: &Path,
        config: &ProjectDiagramConfig,
    ) -> Result<(), ProjectError> {
        let content = serde_json::to_string_pretty(config)
            .map_err(|error| ProjectError::io("serialize project config", error))?;
        write_text_file(
            &project_root.join(PROJECT_CONFIG_FILE),
            &format!("{content}\n"),
        )
        .map_err(|error| ProjectError::io("write project config", error))
    }

    fn remap_diagram_configs(
        config: &mut ProjectDiagramConfig,
        old_path: &str,
        new_path: &str,
        kind: EntryKind,
    ) -> bool {
        let old_prefix = format!("{old_path}/");
        let updates = config
            .diagrams
            .keys()
            .filter_map(|path| {
                if path == old_path {
                    Some((path.clone(), new_path.to_owned()))
                } else if kind == EntryKind::Folder && path.starts_with(&old_prefix) {
                    Some((
                        path.clone(),
                        format!("{new_path}/{}", &path[old_prefix.len()..]),
                    ))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for (old, new) in &updates {
            if let Some(value) = config.diagrams.remove(old) {
                config.diagrams.insert(new.clone(), value);
            }
        }
        !updates.is_empty()
    }

    fn remove_diagram_configs(
        config: &mut ProjectDiagramConfig,
        path: &str,
        kind: EntryKind,
    ) -> bool {
        let prefix = format!("{path}/");
        let before = config.diagrams.len();
        config.diagrams.retain(|candidate, _| {
            candidate != path && !(kind == EntryKind::Folder && candidate.starts_with(&prefix))
        });
        before != config.diagrams.len()
    }
}
