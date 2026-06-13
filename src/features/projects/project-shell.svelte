<script lang="ts">
  import type { Snippet } from "svelte";
  import { onMount } from "svelte";
  import * as Sidebar from "$lib/components/ui/sidebar";
  import EntryDialog from "./components/entry-dialog.svelte";
  import ProjectDialog from "./components/project-dialog.svelte";
  import ProjectSidebar from "./components/project-sidebar.svelte";
  import { ProjectFilesController } from "./project-files-controller.svelte";
  import { setProjectFilesContext } from "./project-files-context";

  let {
    projectName,
    activePath,
    children,
  }: {
    projectName: string;
    activePath: string | null;
    children: Snippet;
  } = $props();

  // The route layout keys this shell by project name.
  // svelte-ignore state_referenced_locally
  const project = new ProjectFilesController(projectName);
  setProjectFilesContext(project);

  onMount(project.initialize);

  $effect(() => {
    project.setActivePath(activePath);
  });
</script>

<Sidebar.Provider class="h-svh max-h-svh overflow-hidden">
  <ProjectSidebar
    {projectName}
    tree={project.tree}
    activePath={project.activePath}
    loading={project.loading}
    projects={project.projects}
    loadingProjects={project.loadingProjects}
    onselect={project.openDiagram}
    oncreate={project.openCreateDialog}
    onrename={project.openRenameDialog}
    ondelete={project.deleteEntry}
    onrefresh={project.refreshTree}
    onrefreshprojects={project.refreshProjects}
    onopenproject={project.openProject}
    onnewproject={project.openProjectDialog}
  />
  <Sidebar.Inset class="h-full min-h-0 max-h-svh overflow-hidden">
    {@render children()}
  </Sidebar.Inset>
</Sidebar.Provider>

<EntryDialog
  bind:open={project.dialog.open}
  mode={project.dialog.mode}
  kind={project.dialog.kind}
  initialName={project.dialog.mode === "rename"
    ? (project.dialog.target?.name ?? "")
    : ""}
  busy={project.dialog.busy}
  error={project.dialog.error}
  onsubmit={project.submitEntry}
/>

<ProjectDialog
  bind:open={project.projectDialogOpen}
  busy={project.creatingProject}
  error={project.projectDialogError}
  onsubmit={project.createProject}
/>
