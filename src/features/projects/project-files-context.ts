import { createContext } from "svelte";
import type { ProjectFilesController } from "./project-files-controller.svelte";

export const [getProjectFilesContext, setProjectFilesContext] =
  createContext<ProjectFilesController>();
