use super::{
    model::{CreateEntryRequest, EntryKind, RenameEntryRequest},
    service::ProjectService,
};
use crate::features::settings::model::{
    CommonDiagramConfigOverrides, DiagramConfigOverrides, DiagramTheme,
};
use std::fs;
use tempfile::TempDir;

fn service() -> (TempDir, ProjectService) {
    let temp = TempDir::new().unwrap();
    let service = ProjectService::new(temp.path().join("projects")).unwrap();
    (temp, service)
}

#[test]
fn creates_and_lists_projects_with_a_starter_diagram() {
    let (_temp, service) = service();
    let project = service.create_project("System Design").unwrap();
    assert_eq!(project.name, "System Design");
    assert!(service
        .read_diagram("System Design", "diagram.mmd")
        .unwrap()
        .contains("flowchart LR"));
    service
        .write_diagram("System Design", "diagram.mmd", "sequenceDiagram")
        .unwrap();
    assert_eq!(
        service
            .read_diagram("System Design", "diagram.mmd")
            .unwrap(),
        "sequenceDiagram"
    );
    assert_eq!(service.list_projects().unwrap(), vec![project]);
}

#[test]
fn rejects_duplicate_and_invalid_project_names() {
    let (_temp, service) = service();
    service.create_project("one").unwrap();
    assert_eq!(
        service.create_project("one").unwrap_err().code,
        "alreadyExists"
    );
    for name in ["", ".", "..", "../escape", "nested/name", " trailing"] {
        assert_eq!(
            service.create_project(name).unwrap_err().code,
            "invalidName"
        );
    }
}

#[test]
fn supports_nested_crud_and_directory_first_sorting() {
    let (_temp, service) = service();
    service.create_project("project").unwrap();
    service
        .create_entry(
            "project",
            CreateEntryRequest {
                parent_path: String::new(),
                name: "z-folder".into(),
                kind: EntryKind::Folder,
            },
        )
        .unwrap();
    service
        .create_entry(
            "project",
            CreateEntryRequest {
                parent_path: "z-folder".into(),
                name: "nested.mmd".into(),
                kind: EntryKind::File,
            },
        )
        .unwrap();
    service
        .create_entry(
            "project",
            CreateEntryRequest {
                parent_path: String::new(),
                name: "alpha.mmd".into(),
                kind: EntryKind::File,
            },
        )
        .unwrap();
    assert_eq!(
        service
            .create_entry(
                "project",
                CreateEntryRequest {
                    parent_path: String::new(),
                    name: "alpha.mmd".into(),
                    kind: EntryKind::File,
                },
            )
            .unwrap_err()
            .code,
        "alreadyExists"
    );

    let tree = service.list_project_tree("project").unwrap();
    assert_eq!(tree[0].name, "z-folder");
    assert_eq!(tree[1].name, "alpha.mmd");
    assert_eq!(tree[2].name, "diagram.mmd");

    let renamed = service
        .rename_entry(
            "project",
            RenameEntryRequest {
                path: "z-folder/nested.mmd".into(),
                new_name: "renamed.mmd".into(),
                kind: EntryKind::File,
            },
        )
        .unwrap();
    assert_eq!(renamed, "z-folder/renamed.mmd");
    assert_eq!(
        service
            .rename_entry(
                "project",
                RenameEntryRequest {
                    path: "z-folder/renamed.mmd".into(),
                    new_name: "renamed.mmd".into(),
                    kind: EntryKind::File,
                },
            )
            .unwrap_err()
            .code,
        "alreadyExists"
    );
    service
        .delete_entry("project", "z-folder", EntryKind::Folder)
        .unwrap();
    assert_eq!(service.list_project_tree("project").unwrap().len(), 2);
}

#[test]
fn enforces_mmd_and_rejects_unsafe_paths() {
    let (_temp, service) = service();
    service.create_project("project").unwrap();
    let invalid_file = service.create_entry(
        "project",
        CreateEntryRequest {
            parent_path: String::new(),
            name: "notes.txt".into(),
            kind: EntryKind::File,
        },
    );
    assert_eq!(invalid_file.unwrap_err().code, "invalidName");
    for path in ["/tmp/file.mmd", "../file.mmd", "./diagram.mmd"] {
        assert_eq!(
            service.read_diagram("project", path).unwrap_err().code,
            "invalidPath"
        );
    }
}

#[cfg(unix)]
#[test]
fn rejects_symlink_entries() {
    use std::os::unix::fs::symlink;

    let (temp, service) = service();
    service.create_project("project").unwrap();
    let outside = temp.path().join("outside.mmd");
    fs::write(&outside, "flowchart LR").unwrap();
    symlink(outside, temp.path().join("projects/project/linked.mmd")).unwrap();

    assert_eq!(
        service
            .read_diagram("project", "linked.mmd")
            .unwrap_err()
            .code,
        "invalidPath"
    );
    assert_eq!(
        service.list_project_tree("project").unwrap_err().code,
        "invalidPath"
    );

    let inside = temp.path().join("projects/project/inside");
    fs::create_dir(&inside).unwrap();
    symlink(&inside, temp.path().join("projects/project/linked-folder")).unwrap();
    fs::write(inside.join("nested.mmd"), "flowchart LR").unwrap();
    assert_eq!(
        service
            .read_diagram("project", "linked-folder/nested.mmd")
            .unwrap_err()
            .code,
        "invalidPath"
    );
}

#[test]
fn saves_project_and_diagram_configuration() {
    let (temp, service) = service();
    service.create_project("project").unwrap();
    let defaults = DiagramConfigOverrides {
        common: Some(CommonDiagramConfigOverrides {
            theme: Some(DiagramTheme::Dark),
            ..Default::default()
        }),
        ..Default::default()
    };
    service
        .save_project_diagram_defaults("project", defaults.clone())
        .unwrap();
    service
        .save_diagram_overrides("project", "diagram.mmd", defaults.clone())
        .unwrap();

    let config = service.get_diagram_config("project").unwrap();
    assert_eq!(config.defaults, defaults);
    assert_eq!(config.diagrams.get("diagram.mmd"), Some(&defaults));
    assert!(temp
        .path()
        .join("projects/project/.arch-diagrams.json")
        .exists());
    assert_eq!(service.list_project_tree("project").unwrap().len(), 1);
}

#[test]
fn remaps_and_removes_diagram_configuration_with_entries() {
    let (_temp, service) = service();
    service.create_project("project").unwrap();
    service
        .create_entry(
            "project",
            CreateEntryRequest {
                parent_path: String::new(),
                name: "folder".into(),
                kind: EntryKind::Folder,
            },
        )
        .unwrap();
    service
        .create_entry(
            "project",
            CreateEntryRequest {
                parent_path: "folder".into(),
                name: "nested.mmd".into(),
                kind: EntryKind::File,
            },
        )
        .unwrap();
    service
        .save_diagram_overrides(
            "project",
            "folder/nested.mmd",
            DiagramConfigOverrides {
                common: Some(CommonDiagramConfigOverrides {
                    theme: Some(DiagramTheme::Forest),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .unwrap();
    service
        .rename_entry(
            "project",
            RenameEntryRequest {
                path: "folder".into(),
                new_name: "renamed".into(),
                kind: EntryKind::Folder,
            },
        )
        .unwrap();
    assert!(service
        .get_diagram_config("project")
        .unwrap()
        .diagrams
        .contains_key("renamed/nested.mmd"));

    service
        .delete_entry("project", "renamed", EntryKind::Folder)
        .unwrap();
    assert!(service
        .get_diagram_config("project")
        .unwrap()
        .diagrams
        .is_empty());
}

#[test]
fn refuses_to_overwrite_invalid_project_configuration() {
    let (temp, service) = service();
    service.create_project("project").unwrap();
    let path = temp.path().join("projects/project/.arch-diagrams.json");
    fs::write(&path, "{ invalid").unwrap();

    assert_eq!(
        service
            .save_project_diagram_defaults("project", Default::default())
            .unwrap_err()
            .code,
        "invalidConfig"
    );
    assert_eq!(fs::read_to_string(path).unwrap(), "{ invalid");
}
