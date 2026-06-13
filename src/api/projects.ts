import { invoke } from "@tauri-apps/api/core";

export type EntryKind = "file" | "folder";
export type SaveStatus = "idle" | "saving" | "saved" | "error";

export interface Project {
  name: string;
}

export interface TreeNode {
  name: string;
  path: string;
  kind: EntryKind;
  children: TreeNode[];
}

export interface ProjectError {
  code: string;
  message: string;
}

export type PngScale = 1 | 2 | 3;
export type PngBackground = "transparent" | "white";

export interface PngOptions {
  scale: PngScale;
  background: PngBackground;
}

export interface PngMetadata {
  width: number;
  height: number;
}

export function errorMessage(error: unknown) {
  if (
    typeof error === "object" &&
    error !== null &&
    "message" in error &&
    typeof error.message === "string"
  ) {
    return error.message;
  }
  return error instanceof Error ? error.message : String(error);
}

export const projectsApi = {
  listProjects: () => invoke<Project[]>("list_projects"),
  createProject: (name: string) =>
    invoke<Project>("create_project", { name }),
  listTree: (project: string) =>
    invoke<TreeNode[]>("list_project_tree", { project }),
  readDiagram: (project: string, path: string) =>
    invoke<string>("read_diagram", { project, path }),
  writeDiagram: (project: string, path: string, content: string) =>
    invoke<void>("write_diagram", { project, path, content }),
  createEntry: (
    project: string,
    parentPath: string,
    name: string,
    kind: EntryKind,
  ) =>
    invoke<TreeNode>("create_entry", {
      project,
      request: { parentPath, name, kind },
    }),
  renameEntry: (
    project: string,
    path: string,
    newName: string,
    kind: EntryKind,
  ) =>
    invoke<string>("rename_entry", {
      project,
      request: { path, newName, kind },
    }),
  deleteEntry: (project: string, path: string, kind: EntryKind) =>
    invoke<void>("delete_entry", { project, path, kind }),
};

export const exportApi = {
  inspectPng: (svg: string, options: PngOptions) =>
    invoke<PngMetadata>("inspect_diagram_png", { svg, options }),
  savePng: (svg: string, path: string, options: PngOptions) =>
    invoke<PngMetadata>("save_diagram_png", { svg, path, options }),
  copyPng: (svg: string) =>
    invoke<PngMetadata>("copy_diagram_png", { svg }),
};
