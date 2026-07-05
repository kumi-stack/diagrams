<script lang="ts">
  import Trash2Icon from "@lucide/svelte/icons/trash-2";
  import { Button } from "$lib/components/ui/button";
  import * as AlertDialog from "$lib/components/ui/alert-dialog";
  import type { TreeNode } from "@/api/projects";

  let {
    open = $bindable(false),
    target,
    busy,
    onconfirm,
  }: {
    open: boolean;
    target: TreeNode | null;
    busy: boolean;
    onconfirm: () => void;
  } = $props();

  let targetKind = $derived(target?.kind === "folder" ? "folder" : "file");
  let title = $derived(
    targetKind === "folder" ? "Move folder to trash?" : "Move file to trash?",
  );
  let description = $derived(
    target
      ? targetKind === "folder"
        ? `"${target.name}" and everything inside it will be moved to the system trash.`
        : `"${target.name}" will be moved to the system trash.`
      : "",
  );
</script>

<AlertDialog.Root bind:open>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Media class="bg-destructive/10 text-destructive">
        <Trash2Icon class="size-5" />
      </AlertDialog.Media>
      <AlertDialog.Title>{title}</AlertDialog.Title>
      <AlertDialog.Description>{description}</AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel disabled={busy}>Cancel</AlertDialog.Cancel>
      <Button variant="destructive" disabled={busy} onclick={onconfirm}>
        {busy ? "Moving..." : "Move to trash"}
      </Button>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>
