import { goto } from "$app/navigation";
import { resolve } from "$app/paths";
import {
  errorMessage,
  projectsApi,
  type EntryKind,
  type Project,
  type TreeNode,
} from "@/api/projects";
import {
  defaultDiagramExtension,
  diagramHref,
} from "@/features/diagrams/diagram-types";
import { settingsApi } from "@/api/settings";
import { toast } from "svelte-sonner";
import type { QuickAddResult } from "@/api/shell";
import { createDiagram } from "./diagram-creation";
import { EntryDialogState } from "./entry-dialog-state.svelte";

export class ProjectFilesController {
  tree = $state<TreeNode[]>([]);
  error = $state("");
  loading = $state(true);
  activePath = $state<string | null>(null);
  projects = $state<Project[]>([]);
  loadingProjects = $state(true);
  projectDialogOpen = $state(false);
  creatingProject = $state(false);
  projectDialogError = $state("");
  aiEnabled = $state(false);
  moveToTrashDialogOpen = $state(false);
  moveToTrashTarget = $state<TreeNode | null>(null);
  movingToTrash = $state(false);

  readonly dialog = new EntryDialogState();
  private flushActiveDiagram: () => Promise<boolean> = async () => true;

  constructor(readonly projectName: string) {}

  initialize = async () => {
    await Promise.all([
      this.refreshTree(),
      this.refreshProjects(),
      this.refreshSettings(),
    ]);
  };

  refreshSettings = async () => {
    try {
      const settings = await settingsApi.get();
      this.aiEnabled = Boolean(
        settings.ollama.enabled && settings.ollama.model,
      );
    } catch {
      this.aiEnabled = false;
    }
  };

  setActivePath = (path: string | null) => {
    this.activePath = path;
  };

  registerDiagramFlush = (flush: () => Promise<boolean>) => {
    this.flushActiveDiagram = flush;
    return () => {
      if (this.flushActiveDiagram === flush) {
        this.flushActiveDiagram = async () => true;
      }
    };
  };

  prepareQuickAddNavigation = async (result: QuickAddResult) => {
    if (!(await this.flushActiveDiagram())) return false;
    if (result.project === this.projectName) await this.refreshTree();
    return true;
  };

  refreshTree = async () => {
    this.loading = true;
    this.error = "";

    try {
      this.tree = await projectsApi.listTree(this.projectName);
    } catch (cause) {
      this.error = errorMessage(cause);
    } finally {
      this.loading = false;
    }
  };

  refreshProjects = async () => {
    this.loadingProjects = true;

    try {
      this.projects = await projectsApi.listProjects();
    } catch (cause) {
      this.error = errorMessage(cause);
    } finally {
      this.loadingProjects = false;
    }
  };

  openProjectDialog = () => {
    this.projectDialogError = "";
    this.projectDialogOpen = true;
  };

  createProject = async (name: string) => {
    this.creatingProject = true;
    this.projectDialogError = "";

    try {
      if (!(await this.flushActiveDiagram())) return;
      const project = await projectsApi.createProject(name);
      this.projectDialogOpen = false;
      await goto(resolve("/projects/[project]", { project: project.name }));
    } catch (cause) {
      this.projectDialogError = errorMessage(cause);
    } finally {
      this.creatingProject = false;
    }
  };

  openProject = async (name: string) => {
    if (name === this.projectName || !(await this.flushActiveDiagram())) return;
    await goto(resolve("/projects/[project]", { project: name }));
  };

  openDiagram = async (node: TreeNode) => {
    if (node.kind !== "file") return;
    await goto(diagramHref(this.projectName, node.path));
  };

  openCreateDialog = (parentPath: string, kind: EntryKind) => {
    this.dialog.openCreate(parentPath, kind);
  };

  openRenameDialog = (node: TreeNode) => {
    this.dialog.openRename(node);
  };

  submitEntry = async (rawName: string, description = "") => {
    this.dialog.busy = true;
    this.dialog.error = "";
    const name =
      this.dialog.kind === "file" &&
      !rawName.toLowerCase().endsWith(defaultDiagramExtension)
        ? `${rawName}${defaultDiagramExtension}`
        : rawName;

    try {
      if (this.dialog.mode === "create") {
        const { diagram: created, warning: generationWarning } =
          this.dialog.kind === "file"
            ? await createDiagram(
                this.projectName,
                name,
                description,
                this.dialog.parentPath,
                {
                  onStatus: (status) => {
                    this.dialog.status =
                      status === "creating"
                        ? "Creating diagram file..."
                        : status === "generating"
                          ? "Generating Mermaid source with Ollama..."
                          : "Saving generated source...";
                  },
                },
              )
            : {
                diagram: await projectsApi.createEntry(
                  this.projectName,
                  this.dialog.parentPath,
                  name,
                  this.dialog.kind,
                ),
                warning: "",
              };

        this.dialog.open = false;
        await this.refreshTree();

        if (created.kind === "file") {
          await this.openDiagram(created);
          if (generationWarning) {
            toast.warning("Diagram created without AI content", {
              description: generationWarning,
            });
          }
        }
      } else if (this.dialog.target) {
        const oldPath = this.dialog.target.path;
        const renamesActiveDiagram =
          this.activePath === oldPath ||
          (this.dialog.target.kind === "folder" &&
            this.activePath?.startsWith(`${oldPath}/`));
        if (renamesActiveDiagram && !(await this.flushActiveDiagram())) return;

        const newPath = await projectsApi.renameEntry(
          this.projectName,
          oldPath,
          name,
          this.dialog.target.kind,
        );
        const nextPath =
          this.activePath === oldPath
            ? newPath
            : this.dialog.target.kind === "folder" &&
                this.activePath?.startsWith(`${oldPath}/`)
              ? `${newPath}${this.activePath.slice(oldPath.length)}`
              : this.activePath;

        this.dialog.open = false;
        await this.refreshTree();

        if (nextPath !== this.activePath && nextPath) {
          await goto(diagramHref(this.projectName, nextPath), {
            replaceState: true,
          });
        }
      }
    } catch (cause) {
      this.dialog.error = errorMessage(cause);
    } finally {
      this.dialog.busy = false;
      this.dialog.status = "";
    }
  };

  openMoveToTrashDialog = (node: TreeNode) => {
    this.moveToTrashTarget = node;
    this.moveToTrashDialogOpen = true;
  };

  confirmMoveEntryToTrash = async () => {
    const node = this.moveToTrashTarget;
    if (!node) return;

    this.movingToTrash = true;
    this.error = "";
    try {
      const movedActiveDiagram =
        this.activePath === node.path ||
        (node.kind === "folder" &&
          this.activePath?.startsWith(`${node.path}/`));
      if (movedActiveDiagram && !(await this.flushActiveDiagram())) return;

      await projectsApi.moveEntryToTrash(this.projectName, node.path, node.kind);
      this.moveToTrashDialogOpen = false;
      this.moveToTrashTarget = null;
      await this.refreshTree();

      if (movedActiveDiagram) {
        await goto(
          resolve("/projects/[project]", { project: this.projectName }),
          { replaceState: true },
        );
      }
    } catch (cause) {
      this.error = errorMessage(cause);
    } finally {
      this.movingToTrash = false;
    }
  };

  switchProject = async () => {
    await goto(resolve("/"));
  };
}
