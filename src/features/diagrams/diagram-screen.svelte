<script lang="ts">
  import { onNavigate } from "$app/navigation";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import { toast } from "svelte-sonner";
  import DiagramEditor from "./components/diagram-editor.svelte";
  import DiagramHeader from "./components/diagram-header.svelte";
  import PngExportDialog from "./components/png-export-dialog.svelte";
  import { DiagramDocument } from "./diagram-document.svelte";
  import { getDiagramDefinition } from "./diagram-types";
  import { getProjectFilesContext } from "@/features/projects/project-files-context";
  import { errorMessage, exportApi } from "@/api/projects";

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
  const project = getProjectFilesContext();
  // The route keys this component by project and path, so the document is
  // recreated whenever a different diagram is opened.
  // svelte-ignore state_referenced_locally
  const document = new DiagramDocument(
    projectName,
    (message) => (error = message),
  );

  onMount(() => {
    const unregisterFlush = project.registerDiagramFlush(document.flushSave);

    if (!definition) {
      error = `The diagram type for "${diagramPath}" is not supported.`;
      return unregisterFlush;
    }

    void document.load(diagramPath, () => true);
    return unregisterFlush;
  });

  onNavigate(async () => {
    await document.flushSave();
  });

  $effect(() => {
    return document.scheduleSave();
  });

  async function copyImage() {
    if (!document.renderedSvg) return;

    copying = true;
    try {
      const result = await exportApi.copyPng(document.renderedSvg);
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
/>

<PngExportDialog
  bind:open={exportOpen}
  svg={document.renderedSvg}
  {diagramPath}
/>

<div class="bg-muted/30 flex min-h-0 flex-1 flex-col p-3 sm:p-5">
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
    <DiagramEditor type={definition.type} {document} />
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
