<script lang="ts">
  import { onNavigate } from "$app/navigation";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Sheet from "$lib/components/ui/sheet";
  import { toast } from "svelte-sonner";
  import DiagramEditor from "./components/diagram-editor.svelte";
  import DiagramHeader from "./components/diagram-header.svelte";
  import PngExportDialog from "./components/png-export-dialog.svelte";
  import { DiagramDocument } from "./diagram-document.svelte";
  import { getDiagramDefinition } from "./diagram-types";
  import { getProjectFilesContext } from "@/features/projects/project-files-context";
  import { errorMessage, exportApi, projectsApi } from "@/api/projects";
  import { settingsApi } from "@/api/settings";
  import DiagramConfigForm from "./components/diagram-config-form.svelte";
  import { ConfigAutosave } from "./config-autosave.svelte";
  import {
    resolveDiagramConfig,
    toPngOptions,
    type DiagramConfigOverrides,
  } from "./diagram-config";

  let {
    projectName,
    diagramPath,
  }: {
    projectName: string;
    diagramPath: string;
  } = $props();

  let error = $state("");
  let definition = $derived(getDiagramDefinition(diagramPath));
  let exportOpen = $state(false);
  let copying = $state(false);
  let configOpen = $state(false);
  let configLoading = $state(true);
  let configLoaded = $state(false);
  let configLoadError = $state("");
  let globalDefaults = $state<DiagramConfigOverrides>({});
  let projectDefaults = $state<DiagramConfigOverrides>({});
  let diagramOverrides = $state<DiagramConfigOverrides>({});
  let resolvedConfig = $derived(
    resolveDiagramConfig(globalDefaults, projectDefaults, diagramOverrides),
  );
  let pngOptions = $derived(toPngOptions(resolvedConfig));
  const project = getProjectFilesContext();
  // The route keys this component by project and path, so the document is
  // recreated whenever a different diagram is opened.
  // svelte-ignore state_referenced_locally
  const document = new DiagramDocument(
    projectName,
    (message) => (error = message),
  );
  const configAutosave = new ConfigAutosave<DiagramConfigOverrides>((value) =>
    projectsApi.saveDiagramOverrides(projectName, diagramPath, value),
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
      diagramOverrides = manifest.diagrams[diagramPath] ?? {};
      configAutosave.markSaved(diagramOverrides);
      configLoaded = true;
    } catch (cause) {
      configLoadError = errorMessage(cause);
      error = configLoadError;
    } finally {
      configLoading = false;
    }
  }

  async function flushDiagram() {
    const [documentSaved, configSaved] = await Promise.all([
      document.flushSave(),
      configAutosave.flush(),
    ]);
    return documentSaved && configSaved;
  }

  onMount(() => {
    const unregisterFlush = project.registerDiagramFlush(flushDiagram);

    if (!definition) {
      error = `The diagram type for "${diagramPath}" is not supported.`;
      return unregisterFlush;
    }

    void document.load(diagramPath, () => true);
    void loadConfiguration();
    return unregisterFlush;
  });

  onNavigate(async () => {
    await flushDiagram();
  });

  $effect(() => {
    return document.scheduleSave();
  });

  $effect(() => {
    if (configLoaded) configAutosave.schedule(diagramOverrides);
  });

  async function copyImage() {
    if (!document.renderedSvg) return;

    copying = true;
    try {
      const result = await exportApi.copyPng(document.renderedSvg, pngOptions);
      toast.success(`Copied PNG (${result.width} x ${result.height})`);
    } catch (cause) {
      toast.error(errorMessage(cause));
    } finally {
      copying = false;
    }
  }
</script>

<DiagramHeader
  {projectName}
  {diagramPath}
  diagramType={definition?.label ?? "Unsupported"}
  isRendering={document.isRendering}
  hasError={Boolean(document.renderError)}
  saveStatus={document.saveStatus}
  canExport={!document.isRendering &&
    !document.renderError &&
    Boolean(document.renderedSvg)}
  {copying}
  onExport={() => (exportOpen = true)}
  onCopy={copyImage}
  onConfigure={() => (configOpen = true)}
/>

<PngExportDialog
  bind:open={exportOpen}
  svg={document.renderedSvg}
  {diagramPath}
  defaultBackground={resolvedConfig.common.background}
/>

<Sheet.Root bind:open={configOpen}>
  <Sheet.Content class="overflow-y-auto sm:max-w-xl">
    <Sheet.Header class="border-b p-5 pr-14">
      <Sheet.Title>Diagram configuration</Sheet.Title>
      <Sheet.Description>
        Overrides for {diagramPath}. Changes are previewed and saved automatically.
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
          bind:value={diagramOverrides}
          resolved={resolvedConfig}
          saveStatus={configAutosave.status}
          saveError={configAutosave.error}
          onretry={configAutosave.retry}
        />
      {/if}
    </div>
  </Sheet.Content>
</Sheet.Root>

<div class="bg-muted/30 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden p-3 sm:p-5">
  {#if error}
    <div
      class="border-destructive/30 bg-destructive/5 text-destructive mb-3 flex items-center justify-between gap-3 rounded-2xl border px-4 py-2 text-xs"
      role="alert"
    >
      <span>{error}</span>
      {#if document.saveStatus === "error"}
        <Button variant="destructive" size="xs" onclick={document.flushSave}>
          Retry save
        </Button>
      {/if}
    </div>
  {/if}

  {#if definition && document.loadedPath === diagramPath}
    <DiagramEditor type={definition.type} {document} config={resolvedConfig} />
  {:else if document.loading}
    <section class="grid flex-1 place-items-center">
      <p class="text-muted-foreground text-sm">Loading diagram...</p>
    </section>
  {:else}
    <section class="grid flex-1 place-items-center">
      <p class="text-muted-foreground text-sm">
        This diagram cannot be opened.
      </p>
    </section>
  {/if}
</div>
