import {
  errorMessage,
  projectsApi,
  type TreeNode,
} from "@/api/projects";
import { aiApi } from "@/api/settings";
import { defaultDiagramExtension } from "@/features/diagrams/diagram-types";

export interface CreateDiagramResult {
  diagram: TreeNode;
  warning: string;
}

export type DiagramCreationStatus =
  | "creating"
  | "generating"
  | "saving";

export interface CreateDiagramOptions {
  onStatus?: (status: DiagramCreationStatus) => void;
}

export function diagramFileName(rawName: string) {
  const name = rawName.trim();
  return name.toLowerCase().endsWith(defaultDiagramExtension)
    ? name
    : `${name}${defaultDiagramExtension}`;
}

export async function createDiagram(
  project: string,
  rawName: string,
  description = "",
  parentPath = "",
  options: CreateDiagramOptions = {},
): Promise<CreateDiagramResult> {
  options.onStatus?.("creating");
  const diagram = await projectsApi.createEntry(
    project,
    parentPath,
    diagramFileName(rawName),
    "file",
  );

  let warning = "";
  if (description.trim()) {
    try {
      options.onStatus?.("generating");
      const source = await aiApi.generateDiagram(description.trim());
      options.onStatus?.("saving");
      await projectsApi.writeDiagram(project, diagram.path, source);
    } catch (cause) {
      warning = errorMessage(cause);
    }
  }

  return { diagram, warning };
}
