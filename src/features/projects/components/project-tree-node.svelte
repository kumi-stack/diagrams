<script lang="ts">
  import ChevronRightIcon from "@lucide/svelte/icons/chevron-right";
  import FileCodeIcon from "@lucide/svelte/icons/file-code-2";
  import FolderIcon from "@lucide/svelte/icons/folder";
  import FolderOpenIcon from "@lucide/svelte/icons/folder-open";
  import MoreHorizontalIcon from "@lucide/svelte/icons/more-horizontal";
  import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
  import type { EntryKind, TreeNode } from "@/api/projects";
  import ProjectTreeNode from "./project-tree-node.svelte";

  let {
    node,
    activePath,
    depth = 0,
    onselect,
    oncreate,
    onrename,
    onmovetotrash,
  }: {
    node: TreeNode;
    activePath: string | null;
    depth?: number;
    onselect: (node: TreeNode) => void;
    oncreate: (parentPath: string, kind: EntryKind) => void;
    onrename: (node: TreeNode) => void;
    onmovetotrash: (node: TreeNode) => void;
  } = $props();

  let open = $state(true);
  let isFolder = $derived(node.kind === "folder");
</script>

<div
  role="treeitem"
  aria-expanded={isFolder ? open : undefined}
  aria-selected={activePath === node.path}
>
  <div
    class:bg-sidebar-accent={activePath === node.path}
    class="group/tree-node hover:bg-sidebar-accent/70 flex h-8 items-center rounded-xl pr-1"
    style:padding-left={`${depth * 0.8 + 0.25}rem`}
  >
    <button
      type="button"
      class="focus-visible:ring-sidebar-ring flex min-w-0 flex-1 items-center gap-1.5 rounded-lg px-1.5 py-1 text-left text-xs outline-none focus-visible:ring-2"
      onclick={() => (isFolder ? (open = !open) : onselect(node))}
    >
      {#if isFolder}
        <ChevronRightIcon
          class={`size-3 shrink-0 transition-transform ${open ? "rotate-90" : ""}`}
        />
        {#if open}
          <FolderOpenIcon class="text-primary size-3.5 shrink-0" />
        {:else}
          <FolderIcon class="text-primary size-3.5 shrink-0" />
        {/if}
      {:else}
        <span class="w-3 shrink-0"></span>
        <FileCodeIcon class="text-muted-foreground size-3.5 shrink-0" />
      {/if}
      <span class="truncate">{node.name}</span>
    </button>

    <DropdownMenu.Root>
      <DropdownMenu.Trigger
        aria-label={`Actions for ${node.name}`}
        class="hover:bg-background focus-visible:ring-ring flex size-6 shrink-0 items-center justify-center rounded-lg opacity-0 outline-none group-hover/tree-node:opacity-100 focus:opacity-100 focus-visible:ring-2"
      >
        <MoreHorizontalIcon class="size-3.5" />
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="start">
        {#if isFolder}
          <DropdownMenu.Item onclick={() => oncreate(node.path, "file")}>
            New diagram
          </DropdownMenu.Item>
          <DropdownMenu.Item onclick={() => oncreate(node.path, "folder")}>
            New folder
          </DropdownMenu.Item>
          <DropdownMenu.Separator />
        {/if}
        <DropdownMenu.Item onclick={() => onrename(node)}>Rename</DropdownMenu.Item>
        <DropdownMenu.Item class="text-destructive" onclick={() => onmovetotrash(node)}>
          Move to trash
        </DropdownMenu.Item>
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>

  {#if isFolder && open}
    <div role="group">
      {#each node.children as child (child.path)}
        <ProjectTreeNode
          node={child}
          {activePath}
          depth={depth + 1}
          {onselect}
          {oncreate}
          {onrename}
          {onmovetotrash}
        />
      {/each}
    </div>
  {/if}
</div>
