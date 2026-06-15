<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import FilePlusIcon from "@lucide/svelte/icons/file-plus-2";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Label } from "$lib/components/ui/label";
  import * as NativeSelect from "$lib/components/ui/native-select";
  import { Spinner } from "$lib/components/ui/spinner";
  import { Textarea } from "$lib/components/ui/textarea";
  import { errorMessage, projectsApi, type Project } from "@/api/projects";
  import { settingsApi } from "@/api/settings";
  import {
    QUICK_ADD_OPENED_EVENT,
    shellApi,
  } from "@/api/shell";
  import { createDiagram } from "@/features/projects/diagram-creation";

  let projects = $state<Project[]>([]);
  let project = $state("");
  let name = $state("");
  let description = $state("");
  let aiEnabled = $state(false);
  let loading = $state(true);
  let creating = $state(false);
  let error = $state("");

  onMount(() => {
    void loadForm();
    const unlisten = listen(QUICK_ADD_OPENED_EVENT, () => void loadForm());
    return () => void unlisten.then((stop) => stop());
  });

  async function loadForm() {
    loading = true;
    error = "";
    name = "";
    description = "";

    try {
      const [availableProjects, currentProject, settings] = await Promise.all([
        projectsApi.listProjects(),
        shellApi.getCurrentProject(),
        settingsApi.get(),
      ]);
      projects = availableProjects;
      project =
        currentProject &&
        availableProjects.some((item) => item.name === currentProject)
          ? currentProject
          : (availableProjects[0]?.name ?? "");
      aiEnabled = Boolean(
        settings.ollama.enabled && settings.ollama.model,
      );
    } catch (cause) {
      error = errorMessage(cause);
    } finally {
      loading = false;
    }
  }

  async function submit(event: SubmitEvent) {
    event.preventDefault();
    if (!project || !name.trim()) return;

    creating = true;
    error = "";
    try {
      const result = await createDiagram(project, name, description);
      await shellApi.finishQuickAdd({
        project,
        path: result.diagram.path,
        warning: result.warning || null,
      });
    } catch (cause) {
      error = errorMessage(cause);
    } finally {
      creating = false;
    }
  }
</script>

<svelte:head>
  <title>New diagram | Diagram Studio</title>
</svelte:head>

<main class="bg-muted/30 grid min-h-screen place-items-center p-4">
  <Card.Root class="w-full max-w-md">
    <Card.Header>
      <div class="flex items-center gap-3">
        <div class="bg-primary/10 text-primary grid size-10 place-items-center rounded-2xl">
          <FilePlusIcon class="size-5" />
        </div>
        <div>
          <Card.Title>New diagram</Card.Title>
          <Card.Description>Create a Mermaid diagram in a project.</Card.Description>
        </div>
      </div>
    </Card.Header>

    <form onsubmit={submit}>
      <Card.Content class="grid gap-4">
        {#if loading}
          <div class="text-muted-foreground flex items-center gap-2 py-8 text-sm">
            <Spinner />
            Loading projects
          </div>
        {:else if projects.length === 0}
          <p class="text-muted-foreground py-8 text-sm">
            Create a project in the main application before adding a diagram.
          </p>
        {:else}
          <div class="grid gap-2">
            <Label for="quick-project">Project</Label>
            <NativeSelect.Root
              id="quick-project"
              bind:value={project}
              class="w-full"
              disabled={creating}
            >
              {#each projects as item (item.name)}
                <NativeSelect.Option value={item.name}>{item.name}</NativeSelect.Option>
              {/each}
            </NativeSelect.Root>
          </div>

          <div class="grid gap-2">
            <Label for="quick-name">Diagram name</Label>
            <Input
              id="quick-name"
              bind:value={name}
              placeholder="diagram.mmd"
              disabled={creating}
              autofocus
            />
          </div>

          {#if aiEnabled}
            <div class="grid gap-2">
              <Label for="quick-description">Describe the diagram (optional)</Label>
              <Textarea
                id="quick-description"
                bind:value={description}
                placeholder="A web application with an API gateway, services and a database."
                rows={4}
                disabled={creating}
              />
            </div>
          {/if}
        {/if}

        {#if error}
          <p class="text-destructive text-xs" role="alert">{error}</p>
        {/if}
      </Card.Content>

      <Card.Footer class="justify-end gap-2 border-t">
        <Button
          type="button"
          variant="outline"
          disabled={creating}
          onclick={() => void shellApi.cancelQuickAdd()}
        >
          Cancel
        </Button>
        <Button
          type="submit"
          disabled={loading || creating || !project || !name.trim()}
        >
          {#if creating}
            <Spinner />
          {:else}
            <FilePlusIcon />
          {/if}
          Create
        </Button>
      </Card.Footer>
    </form>
  </Card.Root>
</main>
