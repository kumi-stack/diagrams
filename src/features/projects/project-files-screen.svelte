<script lang="ts">
  import FolderTreeIcon from "@lucide/svelte/icons/folder-tree";
  import { Button } from "$lib/components/ui/button";
  import ProjectHeader from "./components/project-header.svelte";
  import { getProjectFilesContext } from "./project-files-context";

  let { projectName }: { projectName: string } = $props();
  const project = getProjectFilesContext();
</script>

<ProjectHeader
  {projectName}
  onswitchproject={project.switchProject}
/>
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
