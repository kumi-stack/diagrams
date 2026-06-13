<script lang="ts">
  import DiagramScreen from "@/features/diagrams/diagram-screen.svelte";

  let { params }: import("./$types").PageProps = $props();
  let projectName = $derived(params.project);
  let diagramPath = $derived(params.diagram);
</script>

<svelte:head>
  <title>{diagramPath} | Diagram Studio</title>
  <meta
    name="description"
    content={`Edit ${diagramPath} in ${projectName}.`}
  />
</svelte:head>

{#key `${projectName}:${diagramPath}`}
  <DiagramScreen {projectName} {diagramPath} />
{/key}
