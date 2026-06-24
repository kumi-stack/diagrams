import { errorMessage, type SaveStatus } from "@/api/projects";

export class ConfigAutosave<T> {
  status = $state<SaveStatus>("idle");
  error = $state("");

  private timer: number | undefined;
  private queue = Promise.resolve();
  private lastSaved = "";
  private pending: T | null = null;

  constructor(private readonly save: (value: T) => Promise<unknown>) {}

  markSaved(value: T) {
    this.lastSaved = JSON.stringify(value);
    this.pending = null;
    this.status = "saved";
    this.error = "";
  }

  schedule(value: T) {
    const serialized = JSON.stringify(value);
    if (serialized === this.lastSaved) return;
    this.pending = JSON.parse(serialized) as T;
    window.clearTimeout(this.timer);
    this.status = "saving";
    this.timer = window.setTimeout(() => void this.persist(), 500);
  }

  retry = () => {
    if (this.pending) void this.persist();
  };

  flush = async () => {
    window.clearTimeout(this.timer);
    if (this.pending && JSON.stringify(this.pending) !== this.lastSaved) {
      await this.persist();
    }
    await this.queue;
    return this.status !== "error";
  };

  private persist() {
    const value = this.pending;
    if (!value) return this.queue;
    const serialized = JSON.stringify(value);
    this.status = "saving";
    this.queue = this.queue.then(async () => {
      try {
        await this.save(value);
        if (this.pending && JSON.stringify(this.pending) === serialized) {
          this.lastSaved = serialized;
          this.pending = null;
          this.status = "saved";
          this.error = "";
        }
      } catch (cause) {
        this.status = "error";
        this.error = errorMessage(cause);
      }
    });
    this.queue = this.queue.catch(() => {});
    return this.queue;
  }
}
