<script lang="ts">
  import { page } from "$app/state";
  import ProjectShell from "@/features/projects/project-shell.svelte";

  let { params, children }: import("./$types").LayoutProps = $props();
  let projectName = $derived(params.project);
  let activePath = $derived(page.params.diagram ?? null);
</script>

{#key projectName}
  <ProjectShell {projectName} {activePath}>
    {@render children()}
  </ProjectShell>
{/key}
