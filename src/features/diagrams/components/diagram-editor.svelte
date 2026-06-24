<script lang="ts">
  import type { DiagramDocument } from "../diagram-document.svelte";
  import type { DiagramType } from "../diagram-types";
  import MermaidEditor from "./mermaid-editor.svelte";
  import MermaidPreview from "./mermaid-preview.svelte";
  import type { ResolvedDiagramConfig } from "../diagram-config";

  let {
    type,
    document,
    config,
  }: {
    type: DiagramType;
    document: DiagramDocument;
    config: ResolvedDiagramConfig;
  } = $props();

  let editorHidden = $state(false);
</script>

{#if type === "mermaid"}
  <section
    class={editorHidden
      ? "grid min-h-0 min-w-0 flex-1 grid-cols-1 overflow-y-auto xl:overflow-hidden"
      : "grid min-h-0 min-w-0 flex-1 grid-cols-1 gap-3 overflow-y-auto xl:grid-cols-[minmax(18rem,0.86fr)_minmax(22rem,1.14fr)] xl:overflow-hidden"}
  >
    <div class:hidden={editorHidden} class="h-full min-h-[30rem] min-w-0 xl:min-h-0">
      <MermaidEditor
        bind:value={document.source}
        initialValue={document.lastSavedSource}
      />
    </div>
    <MermaidPreview
      source={document.source}
      {config}
      {editorHidden}
      onToggleEditor={() => (editorHidden = !editorHidden)}
      bind:isRendering={document.isRendering}
      bind:renderError={document.renderError}
      bind:renderedSvg={document.renderedSvg}
    />
  </section>
{/if}
