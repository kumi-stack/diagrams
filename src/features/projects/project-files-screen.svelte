<script lang="ts">
  import { onNavigate } from "$app/navigation";
  import FolderTreeIcon from "@lucide/svelte/icons/folder-tree";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Sheet from "$lib/components/ui/sheet";
  import { settingsApi } from "@/api/settings";
  import { errorMessage, projectsApi } from "@/api/projects";
  import DiagramConfigForm from "@/features/diagrams/components/diagram-config-form.svelte";
  import { ConfigAutosave } from "@/features/diagrams/config-autosave.svelte";
  import {
    resolveDiagramConfig,
    type DiagramConfigOverrides,
  } from "@/features/diagrams/diagram-config";
  import ProjectHeader from "./components/project-header.svelte";
  import { getProjectFilesContext } from "./project-files-context";

  let { projectName }: { projectName: string } = $props();
  const project = getProjectFilesContext();
  let configOpen = $state(false);
  let configLoading = $state(true);
  let configLoaded = $state(false);
  let configLoadError = $state("");
  let globalDefaults = $state<DiagramConfigOverrides>({});
  let projectDefaults = $state<DiagramConfigOverrides>({});
  let resolvedConfig = $derived(
    resolveDiagramConfig(globalDefaults, projectDefaults),
  );
  const configAutosave = new ConfigAutosave<DiagramConfigOverrides>((value) =>
    projectsApi.saveDiagramDefaults(projectName, value),
  );

  async function loadConfiguration() {
    configLoading = true;
    configLoaded = false;
    configLoadError = "";
    try {
      const [settings, manifest] = await Promise.all([
        settingsApi.get(),
        projectsApi.getDiagramConfig(projectName),
      ]);
      globalDefaults = settings.diagramDefaults;
      projectDefaults = manifest.defaults;
      configAutosave.markSaved(projectDefaults);
      configLoaded = true;
    } catch (cause) {
      configLoadError = errorMessage(cause);
    } finally {
      configLoading = false;
    }
  }

  onMount(loadConfiguration);

  onNavigate(async () => {
    await configAutosave.flush();
  });

  $effect(() => {
    if (configLoaded) configAutosave.schedule(projectDefaults);
  });
</script>

<ProjectHeader
  {projectName}
  onswitchproject={project.switchProject}
  onsettings={() => (configOpen = true)}
/>

<Sheet.Root bind:open={configOpen}>
  <Sheet.Content class="overflow-y-auto sm:max-w-xl">
    <Sheet.Header class="border-b p-5 pr-14">
      <Sheet.Title>Project diagram defaults</Sheet.Title>
      <Sheet.Description>
        Overrides inherited by diagrams in {projectName}.
      </Sheet.Description>
    </Sheet.Header>
    <div class="p-5">
      {#if configLoading}
        <p class="text-muted-foreground text-sm">Loading configuration...</p>
      {:else if !configLoaded}
        <div class="border-destructive/30 bg-destructive/5 grid gap-3 rounded-2xl border p-4">
          <p class="text-destructive text-xs">{configLoadError}</p>
          <Button variant="outline" size="sm" onclick={loadConfiguration}>Retry</Button>
        </div>
      {:else}
        <DiagramConfigForm
          bind:value={projectDefaults}
          resolved={resolvedConfig}
          saveStatus={configAutosave.status}
          saveError={configAutosave.error}
          onretry={configAutosave.retry}
          showPreview
        />
      {/if}
    </div>
  </Sheet.Content>
</Sheet.Root>
<div class="bg-muted/30 grid min-h-0 flex-1 place-items-center p-5">
  <div class="max-w-md text-center">
    <div
      class="bg-primary/10 text-primary mx-auto grid size-12 place-items-center rounded-2xl"
    >
      <FolderTreeIcon class="size-5" />
    </div>
    <h2 class="mt-4 text-base font-semibold">Project files</h2>
    <p class="text-muted-foreground mt-2 text-sm">
      Manage folders and diagram files here. Open a diagram from the sidebar
      to enter its dedicated editor.
    </p>
    <Button
      class="mt-5"
      onclick={() => project.openCreateDialog("", "file")}
    >
      Create diagram
    </Button>
    {#if project.error}
      <p class="text-destructive mt-4 text-xs" role="alert">
        {project.error}
      </p>
    {/if}
  </div>
</div>
