export type DiagramType = "mermaid";

export interface DiagramDefinition {
  type: DiagramType;
  label: string;
  extensions: readonly string[];
}

const diagramDefinitions: readonly DiagramDefinition[] = [
  {
    type: "mermaid",
    label: "Mermaid",
    extensions: [".mmd"],
  },
];

export const defaultDiagramExtension = diagramDefinitions[0].extensions[0];

export function getDiagramDefinition(path: string): DiagramDefinition | null {
  const normalizedPath = path.toLowerCase();
  return (
    diagramDefinitions.find((definition) =>
      definition.extensions.some((extension) =>
        normalizedPath.endsWith(extension),
      ),
    ) ?? null
  );
}

export function diagramHref(projectName: string, diagramPath: string) {
  const project = encodeURIComponent(projectName);
  const diagram = diagramPath
    .split("/")
    .map((segment) => encodeURIComponent(segment))
    .join("/");

  return `/projects/${project}/diagram/${diagram}`;
}
