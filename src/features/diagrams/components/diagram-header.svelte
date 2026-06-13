<script lang="ts">
  import NetworkIcon from "@lucide/svelte/icons/network";
  import ClipboardIcon from "@lucide/svelte/icons/clipboard";
  import DownloadIcon from "@lucide/svelte/icons/download";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import * as Sidebar from "$lib/components/ui/sidebar";
  import { Spinner } from "$lib/components/ui/spinner";
  import type { SaveStatus } from "@/api/projects";

  let {
    projectName,
    diagramPath,
    diagramType,
    isRendering = false,
    hasError = false,
    saveStatus = "idle",
    canExport = false,
    exporting = false,
    copying = false,
    onExport,
    onCopy,
  }: {
    projectName: string;
    diagramPath: string;
    diagramType: string;
    isRendering?: boolean;
    hasError?: boolean;
    saveStatus?: SaveStatus;
    canExport?: boolean;
    exporting?: boolean;
    copying?: boolean;
    onExport?: () => void;
    onCopy?: () => void;
  } = $props();

  let statusLabel = $derived(
    isRendering ? "Rendering" : hasError ? "Syntax error" : "Live preview",
  );
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
      <h1 class="truncate text-sm font-semibold tracking-tight">{diagramPath}</h1>
      <p class="text-muted-foreground truncate text-[0.65rem]">
        {projectName} · {diagramType}
      </p>
    </div>
  </div>

  <div class="flex items-center gap-2">
    <Button
      variant="outline"
      size="sm"
      disabled={!canExport || copying}
      onclick={onCopy}
      aria-label="Copy diagram as PNG"
    >
      {#if copying}
        <Spinner class="size-3.5" />
      {:else}
        <ClipboardIcon class="size-3.5" />
      {/if}
      <span class="hidden lg:inline">Copy image</span>
    </Button>
    <Button
      size="sm"
      disabled={!canExport || exporting}
      onclick={onExport}
      aria-label="Export diagram as PNG"
    >
      {#if exporting}
        <Spinner class="size-3.5" />
      {:else}
        <DownloadIcon class="size-3.5" />
      {/if}
      <span class="hidden sm:inline">Export PNG</span>
    </Button>
    <Badge
      variant={saveStatus === "error" ? "destructive" : "outline"}
      class="bg-background/70 hidden h-7 gap-2 px-2.5 backdrop-blur sm:flex"
    >
      {#if saveStatus === "saving"}
        <Spinner class="size-3" />
        Saving
      {:else if saveStatus === "error"}
        Save failed
      {:else}
        <span class="size-1.5 rounded-full bg-emerald-500"></span>
        Saved
      {/if}
    </Badge>
    <Badge
      variant={hasError ? "destructive" : "outline"}
      class="bg-background/70 hidden h-7 gap-2 px-2.5 backdrop-blur md:flex"
    >
      {#if isRendering}
        <Spinner class="size-3" />
      {:else}
        <span
          class:bg-destructive={hasError}
          class:bg-emerald-500={!hasError}
          class="size-1.5 rounded-full"
        ></span>
      {/if}
      {statusLabel}
    </Badge>
  </div>
</header>
