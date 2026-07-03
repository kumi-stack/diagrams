<script lang="ts">
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
  import PanelLeftCloseIcon from "@lucide/svelte/icons/panel-left-close";
  import PanelLeftOpenIcon from "@lucide/svelte/icons/panel-left-open";
  import mermaid from "mermaid";
  import { onMount } from "svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Badge } from "$lib/components/ui/badge";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Spinner } from "$lib/components/ui/spinner";
  import * as Tooltip from "$lib/components/ui/tooltip";
  import SvgViewport from "$lib/components/svg-viewport.svelte";
  import {
    toMermaidConfig,
    type ResolvedDiagramConfig,
  } from "../diagram-config";

  const renderDebounceMs = 450;

  let {
    source,
    editorHidden,
    onToggleEditor,
    isRendering = $bindable(true),
    renderError = $bindable(""),
    renderedSvg = $bindable(""),
    config,
    compact = false,
  }: {
    source: string;
    editorHidden: boolean;
    onToggleEditor: () => void;
    isRendering?: boolean;
    renderError?: string;
    renderedSvg?: string;
    config: ResolvedDiagramConfig;
    compact?: boolean;
  } = $props();

  let diagramSvg = $state("");
  let renderSequence = 0;
  let isReady = $state(false);

  function removeRenderArtifacts(renderId: string) {
    document.getElementById(renderId)?.remove();
    document.getElementById(`d${renderId}`)?.remove();
    document.getElementById(`i${renderId}`)?.remove();
  }

  onMount(() => {
    isReady = true;
  });

  $effect(() => {
    if (!isReady) return;

    const currentSource = source;
    const currentConfig = config;
    const currentSequence = ++renderSequence;

    const timeout = window.setTimeout(async () => {
      const renderId = `mermaid-preview-${currentSequence}`;
      isRendering = true;

      try {
        mermaid.initialize(toMermaidConfig(currentConfig));
        await mermaid.parse(currentSource);
        const { svg } = await mermaid.render(renderId, currentSource);

        if (currentSequence !== renderSequence) return;

        diagramSvg = svg;
        renderedSvg = svg;
        renderError = "";
      } catch (error) {
        if (currentSequence !== renderSequence) return;

        renderError =
          error instanceof Error ? error.message : "Unable to render diagram.";
      } finally {
        removeRenderArtifacts(renderId);

        if (currentSequence === renderSequence) {
          isRendering = false;
        }
      }
    }, renderDebounceMs);

    return () => window.clearTimeout(timeout);
  });
</script>

<Card.Root class={compact ? "min-h-64 min-w-0 gap-0 py-0 shadow-sm" : "h-full min-h-[30rem] min-w-0 gap-0 py-0 shadow-sm xl:min-h-0"}>
  <Card.Header class="border-b px-4 py-4 sm:px-5">
    <div>
      <Card.Description class="mb-1 text-[0.65rem] font-semibold tracking-[0.16em] uppercase">
        Output
      </Card.Description>
      <Card.Title class="text-sm">Diagram preview</Card.Title>
    </div>

    <Card.Action class="flex items-center gap-2">
      <Badge variant="secondary">Auto-render</Badge>
      {#if !compact}
      <Tooltip.Provider>
        <Tooltip.Root>
          <Tooltip.Trigger>
            {#snippet child({ props })}
              <Button
                {...props}
                variant="outline"
                size="icon-sm"
                onclick={onToggleEditor}
                aria-label={editorHidden ? "Show editor" : "Hide editor"}
                aria-pressed={editorHidden}
              >
                {#if editorHidden}
                  <PanelLeftOpenIcon aria-hidden="true" />
                {:else}
                  <PanelLeftCloseIcon aria-hidden="true" />
                {/if}
              </Button>
            {/snippet}
          </Tooltip.Trigger>
          <Tooltip.Content>
            {editorHidden ? "Show editor" : "Hide editor"}
          </Tooltip.Content>
        </Tooltip.Root>
      </Tooltip.Provider>
      {/if}
    </Card.Action>
  </Card.Header>

  <Card.Content
    class="relative grid min-h-0 flex-1 place-items-center overflow-hidden bg-muted/20 p-0"
    style={config.common.background === "white" ? "background-color: white" : undefined}
  >
    <div
      class="pointer-events-none absolute inset-0 bg-[radial-gradient(circle,var(--border)_1px,transparent_1px)] [background-size:18px_18px]"
    ></div>

    {#if renderError}
      <Alert.Root variant="destructive" class="z-10 max-w-lg bg-background/95 shadow-sm">
        <CircleAlertIcon />
        <Alert.Title>Mermaid could not parse this diagram</Alert.Title>
        <Alert.Description
          class="max-h-32 overflow-auto whitespace-pre-wrap font-mono text-[0.65rem] leading-relaxed"
        >
          {renderError}
        </Alert.Description>
      </Alert.Root>
    {:else if diagramSvg}
      <div class:opacity-50={isRendering} class="z-10 h-full w-full transition-opacity">
        <SvgViewport svg={diagramSvg} ariaLabel="Diagram preview" />
      </div>
    {:else}
      <div class="text-muted-foreground z-10 flex items-center gap-2 text-xs">
        <Spinner class="size-3.5" />
        Preparing preview
      </div>
    {/if}
  </Card.Content>

  {#if !compact}<Card.Footer
    class="text-muted-foreground min-h-9 justify-end border-t px-4 text-[0.65rem]"
  >
    Updates as you type
  </Card.Footer>{/if}
</Card.Root>
