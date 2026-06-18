import type { EntryKind, TreeNode } from "@/api/projects";

export type EntryDialogMode = "create" | "rename";

export class EntryDialogState {
  open = $state(false);
  mode = $state<EntryDialogMode>("create");
  kind = $state<EntryKind>("file");
  parentPath = $state("");
  target = $state<TreeNode | null>(null);
  busy = $state(false);
  error = $state("");
  description = $state("");
  status = $state("");

  openCreate(parentPath: string, kind: EntryKind) {
    this.mode = "create";
    this.kind = kind;
    this.parentPath = parentPath;
    this.target = null;
    this.error = "";
    this.description = "";
    this.status = "";
    this.open = true;
  }

  openRename(node: TreeNode) {
    this.mode = "rename";
    this.kind = node.kind;
    this.target = node;
    this.error = "";
    this.status = "";
    this.open = true;
  }
}
