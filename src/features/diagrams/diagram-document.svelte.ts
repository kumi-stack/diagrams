import {
  errorMessage,
  projectsApi,
  type SaveStatus,
} from "@/api/projects";

export class DiagramDocument {
  source = $state("");
  lastSavedSource = $state("");
  saveStatus = $state<SaveStatus>("idle");
  loading = $state(false);
  isRendering = $state(false);
  renderError = $state("");
  renderedSvg = $state("");
  loadedPath = $state<string | null>(null);

  private saveTimer: number | undefined;
  private saveQueue = Promise.resolve();

  constructor(
    private readonly projectName: string,
    private readonly reportError: (message: string) => void,
  ) {}

  load = async (path: string, isCurrent: () => boolean) => {
    this.loading = true;

    try {
      const content = await projectsApi.readDiagram(
        this.projectName,
        path,
      );
      if (!isCurrent()) return;

      this.loadedPath = path;
      this.source = content;
      this.lastSavedSource = content;
      this.saveStatus = "saved";
    } catch (cause) {
      if (isCurrent()) this.reportError(errorMessage(cause));
    } finally {
      if (isCurrent()) this.loading = false;
    }
  };

  scheduleSave = () => {
    const path = this.loadedPath;
    const content = this.source;
    if (!path || this.loading || content === this.lastSavedSource) return;

    window.clearTimeout(this.saveTimer);
    this.saveStatus = "saving";
    this.saveTimer = window.setTimeout(() => {
      void this.persist(path, content);
    }, 600);

    return () => window.clearTimeout(this.saveTimer);
  };

  flushSave = async () => {
    window.clearTimeout(this.saveTimer);
    await this.saveQueue;

    if (!this.loadedPath || this.source === this.lastSavedSource) {
      return this.saveStatus !== "error";
    }

    await this.persist(this.loadedPath, this.source);
    return this.saveStatus !== "error";
  };

  clear() {
    this.loadedPath = null;
    this.source = "";
    this.lastSavedSource = "";
    this.renderError = "";
    this.renderedSvg = "";
    this.saveStatus = "idle";
  }

  private persist(path: string, content: string) {
    window.clearTimeout(this.saveTimer);
    this.saveStatus = "saving";
    this.saveQueue = this.saveQueue.then(async () => {
      try {
        await projectsApi.writeDiagram(
          this.projectName,
          path,
          content,
        );
        if (this.loadedPath === path && this.source === content) {
          this.lastSavedSource = content;
          this.saveStatus = "saved";
        }
      } catch (cause) {
        if (this.loadedPath === path) {
          const message = errorMessage(cause);
          this.saveStatus = "error";
          this.reportError(`Could not save ${path}: ${message}`);
        }
        throw cause;
      }
    });
    this.saveQueue = this.saveQueue.catch(() => {});
    return this.saveQueue;
  }
}
