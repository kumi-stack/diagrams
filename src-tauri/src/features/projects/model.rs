use crate::features::settings::model::DiagramConfigOverrides;
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::BTreeMap};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDiagramConfig {
    pub version: u32,
    #[serde(default)]
    pub defaults: DiagramConfigOverrides,
    #[serde(default)]
    pub diagrams: BTreeMap<String, DiagramConfigOverrides>,
}

impl Default for ProjectDiagramConfig {
    fn default() -> Self {
        Self {
            version: 1,
            defaults: Default::default(),
            diagrams: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum EntryKind {
    File,
    Folder,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TreeNode {
    pub name: String,
    pub path: String,
    pub kind: EntryKind,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEntryRequest {
    pub parent_path: String,
    pub name: String,
    pub kind: EntryKind,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameEntryRequest {
    pub path: String,
    pub new_name: String,
    pub kind: EntryKind,
}

pub fn compare_names(left: &str, right: &str) -> Ordering {
    left.to_lowercase()
        .cmp(&right.to_lowercase())
        .then_with(|| left.cmp(right))
}

pub fn compare_nodes(left: &TreeNode, right: &TreeNode) -> Ordering {
    match (left.kind, right.kind) {
        (EntryKind::Folder, EntryKind::File) => Ordering::Less,
        (EntryKind::File, EntryKind::Folder) => Ordering::Greater,
        _ => compare_names(&left.name, &right.name),
    }
}
