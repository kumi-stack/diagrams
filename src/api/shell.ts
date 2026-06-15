import { invoke } from "@tauri-apps/api/core";

export const QUICK_ADD_CREATED_EVENT = "quick-add-created";
export const QUICK_ADD_OPENED_EVENT = "quick-add-opened";

export interface QuickAddResult {
  project: string;
  path: string;
  warning: string | null;
}

export const shellApi = {
  setCurrentProject: (project: string | null) =>
    invoke<void>("set_current_project", { project }),
  getCurrentProject: () =>
    invoke<string | null>("get_current_project"),
  finishQuickAdd: (result: QuickAddResult) =>
    invoke<void>("finish_quick_add", { result }),
  cancelQuickAdd: () => invoke<void>("cancel_quick_add"),
};
