<script lang="ts">
  import {
    resolveDiagramConfig,
    type DiagramConfigOverrides,
  } from "../diagram-config";
  import DiagramConfigForm from "./diagram-config-form.svelte";

  let value = $state<DiagramConfigOverrides>({});
  let resolved = $derived(resolveDiagramConfig(value));
</script>

<DiagramConfigForm bind:value {resolved} showPreview />
