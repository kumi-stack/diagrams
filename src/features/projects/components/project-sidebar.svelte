<script lang="ts">
  import { resolve } from "$app/paths";
  import FilePlusIcon from "@lucide/svelte/icons/file-plus-2";
  import FolderPlusIcon from "@lucide/svelte/icons/folder-plus";
  import ChevronsUpDownIcon from "@lucide/svelte/icons/chevrons-up-down";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SettingsIcon from "@lucide/svelte/icons/settings";
  import ProjectTreeNode from "./project-tree-node.svelte";
  import { Button } from "$lib/components/ui/button";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import * as Sidebar from "$lib/components/ui/sidebar";
  import type { EntryKind, Project, TreeNode } from "@/api/projects";

  let {
    projectName,
    tree,
    activePath,
    loading = false,
    projects,
    loadingProjects = false,
    onselect,
    oncreate,
    onrename,
    ondelete,
    onrefresh,
    onrefreshprojects,
    onopenproject,
    onnewproject,
  }: {
    projectName: string;
    tree: TreeNode[];
    activePath: string | null;
    loading?: boolean;
    projects: Project[];
    loadingProjects?: boolean;
    onselect: (node: TreeNode) => void;
    oncreate: (parentPath: string, kind: EntryKind) => void;
    onrename: (node: TreeNode) => void;
    ondelete: (node: TreeNode) => void;
    onrefresh: () => void;
    onrefreshprojects: () => void;
    onopenproject: (name: string) => void;
    onnewproject: () => void;
  } = $props();
</script>

<Sidebar.Root collapsible="offcanvas" class="border-r">
  <Sidebar.Header class="border-b p-4">
    <div class="flex items-center gap-2">
      <DropdownMenu.Root>
        <DropdownMenu.Trigger
          class="hover:bg-sidebar-accent focus-visible:ring-sidebar-ring flex min-w-0 flex-1 items-center gap-2 rounded-xl p-2 text-left outline-none focus-visible:ring-2"
        >
          <div class="bg-primary/10 text-primary grid size-8 shrink-0 place-items-center rounded-xl">
            <FolderIcon class="size-4" />
          </div>
          <div class="min-w-0 flex-1">
            <p class="text-muted-foreground text-[0.58rem] font-semibold tracking-widest uppercase">
              Project
            </p>
            <p class="truncate text-sm font-semibold">{projectName}</p>
          </div>
          <ChevronsUpDownIcon class="text-muted-foreground size-3.5 shrink-0" />
        </DropdownMenu.Trigger>
        <DropdownMenu.Content align="start" class="w-60">
          <DropdownMenu.Label>Switch project</DropdownMenu.Label>
          {#if loadingProjects}
            <DropdownMenu.Item disabled>Loading projects...</DropdownMenu.Item>
          {:else}
            {#each projects as project (project.name)}
              <DropdownMenu.Item
                disabled={project.name === projectName}
                onclick={() => onopenproject(project.name)}
              >
                <FolderIcon />
                <span class="truncate">{project.name}</span>
              </DropdownMenu.Item>
            {/each}
          {/if}
          <DropdownMenu.Separator />
          <DropdownMenu.Item onclick={onnewproject}>
            <PlusIcon />
            New project
          </DropdownMenu.Item>
          <DropdownMenu.Item onclick={onrefreshprojects}>
            <RefreshCwIcon />
            Refresh projects
          </DropdownMenu.Item>
        </DropdownMenu.Content>
      </DropdownMenu.Root>
      <Button
        variant="ghost"
        size="icon-xs"
        aria-label="Refresh project"
        disabled={loading}
        onclick={onrefresh}
      >
        <RefreshCwIcon class={loading ? "animate-spin" : ""} />
      </Button>
    </div>
  </Sidebar.Header>

  <Sidebar.Content class="p-2">
    <div class="mb-2 flex items-center justify-between px-2">
      <span class="text-muted-foreground text-[0.62rem] font-semibold tracking-widest uppercase">
        Diagrams
      </span>
      <div class="flex">
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="New diagram"
          onclick={() => oncreate("", "file")}
        >
          <FilePlusIcon />
        </Button>
        <Button
          variant="ghost"
          size="icon-xs"
          aria-label="New folder"
          onclick={() => oncreate("", "folder")}
        >
          <FolderPlusIcon />
        </Button>
      </div>
    </div>

    {#if tree.length === 0}
      <p class="text-muted-foreground px-3 py-8 text-center text-xs">
        This project has no diagrams.
      </p>
    {:else}
      <div role="tree" aria-label="Project files">
        {#each tree as node (node.path)}
          <ProjectTreeNode
            {node}
            {activePath}
            {onselect}
            {oncreate}
            {onrename}
            {ondelete}
          />
        {/each}
      </div>
    {/if}
  </Sidebar.Content>
  <Sidebar.Footer class="border-t p-3">
    <Button
      href={resolve("/settings")}
      variant="ghost"
      class="w-full justify-start"
    >
      <SettingsIcon />
      Settings
    </Button>
  </Sidebar.Footer>
</Sidebar.Root>
