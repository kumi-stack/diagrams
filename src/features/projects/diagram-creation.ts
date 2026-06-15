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
): Promise<CreateDiagramResult> {
  const diagram = await projectsApi.createEntry(
    project,
    parentPath,
    diagramFileName(rawName),
    "file",
  );

  let warning = "";
  if (description.trim()) {
    try {
      const source = await aiApi.generateDiagram(description.trim());
      await projectsApi.writeDiagram(project, diagram.path, source);
    } catch (cause) {
      warning = errorMessage(cause);
    }
  }

  return { diagram, warning };
}
