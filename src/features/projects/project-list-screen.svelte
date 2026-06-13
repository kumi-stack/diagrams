<script lang="ts">
  import { goto } from "$app/navigation";
  import { resolve } from "$app/paths";
  import { onMount } from "svelte";
  import { errorMessage, projectsApi, type Project } from "@/api/projects";
  import ProjectChooser from "./components/project-chooser.svelte";

  let projects = $state<Project[]>([]);
  let error = $state("");
  let loading = $state(true);
  let creating = $state(false);

  onMount(loadProjects);

  async function loadProjects() {
    loading = true;
    error = "";

    try {
      projects = await projectsApi.listProjects();
    } catch (cause) {
      error = errorMessage(cause);
    } finally {
      loading = false;
    }
  }

  async function createProject(name: string) {
    creating = true;
    error = "";

    try {
      const project = await projectsApi.createProject(name);
      await goto(resolve("/projects/[project]", { project: project.name }));
    } catch (cause) {
      error = errorMessage(cause);
    } finally {
      creating = false;
    }
  }

  function openProject(name: string) {
    return goto(resolve("/projects/[project]", { project: name }));
  }
</script>

<ProjectChooser
  {projects}
  {loading}
  {creating}
  {error}
  oncreate={createProject}
  onopen={openProject}
  onrefresh={loadProjects}
/>
