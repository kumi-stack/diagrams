<script lang="ts">
  import type { DiagramDocument } from "../diagram-document.svelte";
  import type { DiagramType } from "../diagram-types";
  import MermaidEditor from "./mermaid-editor.svelte";
  import MermaidPreview from "./mermaid-preview.svelte";

  let {
    type,
    document,
  }: {
    type: DiagramType;
    document: DiagramDocument;
  } = $props();
</script>

{#if type === "mermaid"}
  <section
    class="grid min-h-0 flex-1 grid-cols-1 gap-3 lg:grid-cols-[minmax(20rem,0.86fr)_minmax(26rem,1.14fr)]"
  >
    <MermaidEditor
      bind:value={document.source}
      initialValue={document.lastSavedSource}
    />
    <MermaidPreview
      source={document.source}
      bind:isRendering={document.isRendering}
      bind:renderError={document.renderError}
      bind:renderedSvg={document.renderedSvg}
    />
  </section>
{/if}
