import { invoke } from "@tauri-apps/api/core";
import type { DiagramConfigOverrides } from "@/features/diagrams/diagram-config";

export interface OllamaSettings {
  enabled: boolean;
  model: string | null;
}

export interface AppSettings {
  ollama: OllamaSettings;
  diagramDefaults: DiagramConfigOverrides;
}

export interface OllamaModel {
  name: string;
}

export const settingsApi = {
  get: () => invoke<AppSettings>("get_settings"),
  save: (settings: AppSettings) =>
    invoke<AppSettings>("save_settings", { settings }),
  saveDiagramDefaults: (defaults: DiagramConfigOverrides) =>
    invoke<AppSettings>("save_global_diagram_defaults", { defaults }),
};

export const aiApi = {
  listModels: () => invoke<OllamaModel[]>("list_ollama_models"),
  generateDiagram: (description: string) =>
    invoke<string>("generate_diagram", { description }),
};
