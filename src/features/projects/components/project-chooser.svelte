<script lang="ts">
  import { resolve } from "$app/paths";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import FolderPlusIcon from "@lucide/svelte/icons/folder-plus";
  import NetworkIcon from "@lucide/svelte/icons/network";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Input } from "$lib/components/ui/input";
  import { Spinner } from "$lib/components/ui/spinner";
  import type { Project } from "@/api/projects";

  let {
    projects,
    loading = false,
    creating = false,
    error = "",
    oncreate,
    onopen,
    onrefresh,
  }: {
    projects: Project[];
    loading?: boolean;
    creating?: boolean;
    error?: string;
    oncreate: (name: string) => void;
    onopen: (name: string) => void;
    onrefresh: () => void;
  } = $props();

  let projectName = $state("");

  function submit(event: SubmitEvent) {
    event.preventDefault();
    const name = projectName.trim();
    if (!name) return;
    oncreate(name);
  }
</script>

<main class="bg-muted/30 grid min-h-screen place-items-center p-4 sm:p-8">
  <Card.Root class="w-full max-w-2xl gap-0 overflow-hidden py-0 shadow-lg">
    <Card.Header class="border-b px-6 py-6 sm:px-8">
      <div class="flex items-center gap-4">
        <div
          class="bg-primary text-primary-foreground flex size-11 items-center justify-center rounded-2xl"
        >
          <NetworkIcon class="size-5" aria-hidden="true" />
        </div>
        <div>
          <Card.Title>Choose a project</Card.Title>
          <Card.Description class="mt-1">
            Diagram projects are stored in ~/.arch-diagrams
          </Card.Description>
        </div>
      </div>
      <Card.Action>
        <div class="flex items-center">
          <Button
            href={resolve("/settings")}
            variant="ghost"
            size="icon-sm"
            aria-label="Settings"
          >
            <SettingsIcon />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Refresh projects"
            disabled={loading}
            onclick={onrefresh}
          >
            <RefreshCwIcon class={loading ? "animate-spin" : ""} />
          </Button>
        </div>
      </Card.Action>
    </Card.Header>

    <Card.Content class="grid gap-6 p-6 sm:p-8">
      <form class="flex gap-2" onsubmit={submit}>
        <Input
          bind:value={projectName}
          aria-label="New project name"
          placeholder="New project name"
          disabled={creating}
        />
        <Button type="submit" disabled={creating || !projectName.trim()}>
          {#if creating}
            <Spinner />
          {:else}
            <FolderPlusIcon data-icon="inline-start" />
          {/if}
          Create
        </Button>
      </form>

      {#if error}
        <p class="text-destructive text-xs" role="alert">{error}</p>
      {/if}

      <section aria-label="Projects">
        <div class="mb-3 flex items-center justify-between">
          <h2 class="text-xs font-semibold tracking-wider uppercase">Projects</h2>
          <span class="text-muted-foreground text-xs">{projects.length}</span>
        </div>

        {#if loading && projects.length === 0}
          <div class="text-muted-foreground flex h-32 items-center justify-center gap-2 text-sm">
            <Spinner />
            Loading projects
          </div>
        {:else if projects.length === 0}
          <div class="border-border bg-muted/20 grid h-32 place-items-center rounded-3xl border border-dashed">
            <div class="text-center">
              <FolderIcon class="text-muted-foreground mx-auto mb-2 size-5" />
              <p class="text-sm font-medium">No projects yet</p>
              <p class="text-muted-foreground mt-1 text-xs">Create one to get started.</p>
            </div>
          </div>
        {:else}
          <div class="grid gap-2 sm:grid-cols-2">
            {#each projects as project (project.name)}
              <a
                href={resolve("/projects/[project]", {
                  project: project.name,
                })}
                onclick={(event) => {
                  event.preventDefault();
                  onopen(project.name);
                }}
                class="hover:bg-muted focus-visible:ring-ring flex min-w-0 items-center gap-3 rounded-3xl border p-4 text-left transition-colors outline-none focus-visible:ring-2"
              >
                <FolderIcon class="text-primary size-4 shrink-0" />
                <span class="truncate text-sm font-medium">{project.name}</span>
              </a>
            {/each}
          </div>
        {/if}
      </section>
    </Card.Content>
  </Card.Root>
</main>
