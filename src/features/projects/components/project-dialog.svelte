<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Input } from "$lib/components/ui/input";
  import { Spinner } from "$lib/components/ui/spinner";

  let {
    open = $bindable(false),
    busy = false,
    error = "",
    onsubmit,
  }: {
    open: boolean;
    busy?: boolean;
    error?: string;
    onsubmit: (name: string) => void;
  } = $props();

  let name = $state("");

  $effect(() => {
    if (open) name = "";
  });

  function submit(event: SubmitEvent) {
    event.preventDefault();
    const projectName = name.trim();
    if (projectName) onsubmit(projectName);
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Create project</Dialog.Title>
      <Dialog.Description>
        A starter Mermaid diagram will be added automatically.
      </Dialog.Description>
    </Dialog.Header>

    <form class="grid gap-4" onsubmit={submit}>
      <Input
        bind:value={name}
        aria-label="New project name"
        placeholder="Project name"
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
          {#if busy}
            <Spinner />
          {/if}
          Create project
        </Button>
      </Dialog.Footer>
    </form>
  </Dialog.Content>
</Dialog.Root>
