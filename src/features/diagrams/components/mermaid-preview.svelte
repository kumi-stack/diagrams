<script lang="ts">
  import CircleAlertIcon from "@lucide/svelte/icons/circle-alert";
  import mermaid from "mermaid";
  import { onMount } from "svelte";
  import * as Alert from "$lib/components/ui/alert";
  import { Badge } from "$lib/components/ui/badge";
  import * as Card from "$lib/components/ui/card";
  import { Spinner } from "$lib/components/ui/spinner";

  let {
    source,
    isRendering = $bindable(true),
    renderError = $bindable(""),
    renderedSvg = $bindable(""),
  }: {
    source: string;
    isRendering?: boolean;
    renderError?: string;
    renderedSvg?: string;
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
    mermaid.initialize({
      startOnLoad: false,
      suppressErrorRendering: true,
      securityLevel: "strict",
      theme: "base",
      htmlLabels: false,
      themeVariables: {
        primaryColor: "#ecfdf8",
        primaryTextColor: "#172423",
        primaryBorderColor: "#4b9f91",
        lineColor: "#657c78",
        secondaryColor: "#f4f8f7",
        tertiaryColor: "#ffffff",
        fontFamily: "JetBrains Mono Variable, monospace",
      },
      flowchart: {
        curve: "basis",
      },
    });

    isReady = true;
  });

  $effect(() => {
    if (!isReady) return;

    const currentSource = source;
    const currentSequence = ++renderSequence;
    isRendering = true;

    const timeout = window.setTimeout(async () => {
      const renderId = `mermaid-preview-${currentSequence}`;

      try {
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
    }, 220);

    return () => window.clearTimeout(timeout);
  });
</script>

<Card.Root class="h-full min-h-[30rem] gap-0 py-0 shadow-sm">
  <Card.Header class="border-b px-4 py-4 sm:px-5">
    <div>
      <Card.Description class="mb-1 text-[0.65rem] font-semibold tracking-[0.16em] uppercase">
        Output
      </Card.Description>
      <Card.Title class="text-sm">Diagram preview</Card.Title>
    </div>

    <Card.Action>
      <Badge variant="secondary">Auto-render</Badge>
    </Card.Action>
  </Card.Header>

  <Card.Content
    class="relative grid min-h-0 flex-1 place-items-center overflow-auto bg-muted/20 p-6 sm:p-10"
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
      <div class:opacity-50={isRendering} class="diagram z-10 grid w-full min-w-72 place-items-center transition-opacity">
        {@html diagramSvg}
      </div>
    {:else}
      <div class="text-muted-foreground z-10 flex items-center gap-2 text-xs">
        <Spinner class="size-3.5" />
        Preparing preview
      </div>
    {/if}
  </Card.Content>

  <Card.Footer
    class="text-muted-foreground min-h-9 justify-end border-t px-4 text-[0.65rem]"
  >
    Updates as you type
  </Card.Footer>
</Card.Root>

<style>
  .diagram :global(svg) {
    width: 100%;
    max-width: 54rem;
    max-height: calc(100vh - 16rem);
    filter: drop-shadow(0 0.75rem 1.25rem rgb(15 23 42 / 0.06));
  }

  @media (max-width: 48rem) {
    .diagram :global(svg) {
      max-height: none;
    }
  }
</style>
