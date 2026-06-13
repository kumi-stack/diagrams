<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import type { EntryKind } from "@/api/projects";
  import { defaultDiagramExtension } from "@/features/diagrams/diagram-types";

  let {
    open = $bindable(false),
    mode,
    kind,
    initialName = "",
    busy = false,
    error = "",
    onsubmit,
  }: {
    open: boolean;
    mode: "create" | "rename";
    kind: EntryKind;
    initialName?: string;
    busy?: boolean;
    error?: string;
    onsubmit: (name: string) => void;
  } = $props();

  let name = $state("");

  $effect(() => {
    if (open) name = initialName;
  });

  function submit(event: SubmitEvent) {
    event.preventDefault();
    const nextName = name.trim();
    if (nextName) onsubmit(nextName);
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>
        {mode === "create" ? "Create" : "Rename"} {kind}
      </Dialog.Title>
      <Dialog.Description>
        {kind === "file"
          ? `New diagrams currently use the ${defaultDiagramExtension} extension.`
          : "Folders can contain diagrams and other folders."}
      </Dialog.Description>
    </Dialog.Header>

    <form class="grid gap-4" onsubmit={submit}>
      <Input
        bind:value={name}
        aria-label={`${kind} name`}
        placeholder={kind === "file"
          ? `diagram${defaultDiagramExtension}`
          : "Folder name"}
        disabled={busy}
        autofocus
      />
      {#if error}
        <p class="text-destructive text-xs" role="alert">{error}</p>
      {/if}
      <Dialog.Footer>
        <Button variant="outline" onclick={() => (open = false)} disabled={busy}>
          Cancel
        </Button>
        <Button type="submit" disabled={busy || !name.trim()}>
          {mode === "create" ? "Create" : "Rename"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
