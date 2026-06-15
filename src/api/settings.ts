import { invoke } from "@tauri-apps/api/core";

export interface OllamaSettings {
  enabled: boolean;
  model: string | null;
}

export interface AppSettings {
  ollama: OllamaSettings;
}

export interface OllamaModel {
  name: string;
}

export const settingsApi = {
  get: () => invoke<AppSettings>("get_settings"),
  save: (settings: AppSettings) =>
    invoke<AppSettings>("save_settings", { settings }),
};

export const aiApi = {
  listModels: () => invoke<OllamaModel[]>("list_ollama_models"),
  generateDiagram: (description: string) =>
    invoke<string>("generate_diagram", { description }),
};
