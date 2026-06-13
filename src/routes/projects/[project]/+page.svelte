<script lang="ts">
  import ProjectFilesScreen from "@/features/projects/project-files-screen.svelte";

  let { params }: import("./$types").PageProps = $props();
  let projectName = $derived(params.project);
</script>

<svelte:head>
  <title>{projectName} | Diagram Studio</title>
  <meta
    name="description"
    content={`Manage diagram files in ${projectName}.`}
  />
</svelte:head>

{#key projectName}
  <ProjectFilesScreen {projectName} />
{/key}
