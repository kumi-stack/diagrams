<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import { Spinner } from "$lib/components/ui/spinner";
  import { Textarea } from "$lib/components/ui/textarea";
  import type { EntryKind } from "@/api/projects";
  import { defaultDiagramExtension } from "@/features/diagrams/diagram-types";

  let {
    open = $bindable(false),
    mode,
    kind,
    initialName = "",
    busy = false,
    error = "",
    aiEnabled = false,
    onsubmit,
  }: {
    open: boolean;
    mode: "create" | "rename";
    kind: EntryKind;
    initialName?: string;
    busy?: boolean;
    error?: string;
    aiEnabled?: boolean;
    onsubmit: (name: string, description: string) => void;
  } = $props();

  let name = $state("");
  let description = $state("");

  $effect(() => {
    if (open) {
      name = initialName;
      description = "";
    }
  });

  function submit(event: SubmitEvent) {
    event.preventDefault();
    const nextName = name.trim();
    if (nextName) onsubmit(nextName, description.trim());
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
      {#if mode === "create" && kind === "file" && aiEnabled}
        <div class="grid gap-2">
          <Label for="diagram-description">Describe the diagram (optional)</Label>
          <Textarea
            id="diagram-description"
            bind:value={description}
            placeholder="For example: A web application with an API gateway, two services, PostgreSQL and a message queue."
            rows={5}
            disabled={busy}
          />
          <p class="text-muted-foreground text-xs">
            The selected local Ollama model will generate the initial Mermaid source.
          </p>
        </div>
      {/if}
      {#if error}
        <p class="text-destructive text-xs" role="alert">{error}</p>
      {/if}
      <Dialog.Footer>
        <Button variant="outline" onclick={() => (open = false)} disabled={busy}>
          Cancel
        </Button>
        <Button type="submit" disabled={busy || !name.trim()}>
          {#if busy}
            <Spinner />
          {/if}
          {mode === "create" ? "Create" : "Rename"}
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
