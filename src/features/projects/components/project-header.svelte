<script lang="ts">
  import FolderSyncIcon from "@lucide/svelte/icons/folder-sync";
  import NetworkIcon from "@lucide/svelte/icons/network";
  import { Button } from "$lib/components/ui/button";
  import * as Sidebar from "$lib/components/ui/sidebar";

  let {
    projectName = "",
    onswitchproject,
  }: {
    projectName?: string;
    onswitchproject: () => void;
  } = $props();
</script>

<header class="flex min-h-16 items-center justify-between gap-4 border-b px-4 sm:px-5">
  <div class="flex min-w-0 items-center gap-3">
    <Sidebar.Trigger />
    <div
      class="bg-primary text-primary-foreground hidden size-9 shrink-0 items-center justify-center rounded-2xl shadow-sm sm:flex"
    >
      <NetworkIcon class="size-4" aria-hidden="true" />
    </div>

    <div class="min-w-0">
      <h1 class="truncate text-sm font-semibold tracking-tight">{projectName}</h1>
      <p class="text-muted-foreground truncate text-[0.65rem]">
        Project files
      </p>
    </div>
  </div>

  <Button variant="outline" size="sm" onclick={onswitchproject}>
    <FolderSyncIcon data-icon="inline-start" />
    <span class="hidden sm:inline">Switch project</span>
  </Button>
</header>
