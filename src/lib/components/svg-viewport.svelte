<script lang="ts">
  import MinusIcon from "@lucide/svelte/icons/minus";
  import PlusIcon from "@lucide/svelte/icons/plus";
  import ScanIcon from "@lucide/svelte/icons/scan";
  import { Button } from "$lib/components/ui/button";
  import * as ButtonGroup from "$lib/components/ui/button-group";
  import * as Tooltip from "$lib/components/ui/tooltip";

  const MIN_SCALE = 0.25;
  const MAX_SCALE = 4;
  const SCALE_STEP = 0.2;
  const KEYBOARD_PAN_RATIO = 0.06;

  type ViewBox = {
    x: number;
    y: number;
    width: number;
    height: number;
  };

  let {
    svg,
    ariaLabel = "SVG preview",
  }: {
    svg: string;
    ariaLabel?: string;
  } = $props();

  let viewport: HTMLDivElement;
  let svgElement = $state<SVGSVGElement | null>(null);
  let baseViewBox = $state<ViewBox>({ x: 0, y: 0, width: 1, height: 1 });
  let scale = $state(1);
  let viewX = $state(0);
  let viewY = $state(0);
  let activePointerId: number | null = null;
  let pointerX = 0;
  let pointerY = 0;
  let isPanning = $state(false);
  let hasRenderedSvg = false;

  let scalePercent = $derived(Math.round(scale * 100));
  let canZoomIn = $derived(scale < MAX_SCALE);
  let canZoomOut = $derived(scale > MIN_SCALE);
  let viewWidth = $derived(baseViewBox.width / scale);
  let viewHeight = $derived(baseViewBox.height / scale);

  function attachViewport(node: HTMLDivElement) {
    viewport = node;
    node.addEventListener("wheel", handleWheel, { passive: false });
    node.addEventListener("pointerdown", handlePointerDown);
    node.addEventListener("pointermove", handlePointerMove);
    node.addEventListener("pointerup", stopPanning);
    node.addEventListener("pointercancel", stopPanning);
    node.addEventListener("keydown", handleKeydown);

    return {
      destroy() {
        node.removeEventListener("wheel", handleWheel);
        node.removeEventListener("pointerdown", handlePointerDown);
        node.removeEventListener("pointermove", handlePointerMove);
        node.removeEventListener("pointerup", stopPanning);
        node.removeEventListener("pointercancel", stopPanning);
        node.removeEventListener("keydown", handleKeydown);
      },
    };
  }

  function readViewBox(element: SVGSVGElement): ViewBox {
    const values = element
      .getAttribute("viewBox")
      ?.trim()
      .split(/[\s,]+/)
      .map(Number);

    if (
      values?.length === 4 &&
      values.every(Number.isFinite) &&
      values[2] > 0 &&
      values[3] > 0
    ) {
      return {
        x: values[0],
        y: values[1],
        width: values[2],
        height: values[3],
      };
    }

    const width = element.width.baseVal.value || 1;
    const height = element.height.baseVal.value || 1;
    return { x: 0, y: 0, width, height };
  }

  function updateRenderedSvg(node: HTMLElement, value: string) {
    const centerRatioX = hasRenderedSvg
      ? (viewX + viewWidth / 2 - baseViewBox.x) / baseViewBox.width
      : 0.5;
    const centerRatioY = hasRenderedSvg
      ? (viewY + viewHeight / 2 - baseViewBox.y) / baseViewBox.height
      : 0.5;

    node.innerHTML = value;
    const nextSvg = node.querySelector<SVGSVGElement>("svg");
    if (!nextSvg) {
      svgElement = null;
      return;
    }

    const nextBaseViewBox = readViewBox(nextSvg);
    baseViewBox = nextBaseViewBox;
    viewX =
      nextBaseViewBox.x +
      centerRatioX * nextBaseViewBox.width -
      nextBaseViewBox.width / scale / 2;
    viewY =
      nextBaseViewBox.y +
      centerRatioY * nextBaseViewBox.height -
      nextBaseViewBox.height / scale / 2;
    svgElement = nextSvg;
    hasRenderedSvg = true;
  }

  function renderSvg(node: HTMLElement, value: string) {
    updateRenderedSvg(node, value);

    return {
      update(nextValue: string) {
        updateRenderedSvg(node, nextValue);
      },
      destroy() {
        svgElement = null;
        node.replaceChildren();
      },
    };
  }

  $effect(() => {
    if (!svgElement) return;

    svgElement.setAttribute(
      "viewBox",
      `${viewX} ${viewY} ${viewWidth} ${viewHeight}`,
    );
  });

  function clampScale(nextScale: number) {
    return Math.min(MAX_SCALE, Math.max(MIN_SCALE, nextScale));
  }

  function clientToSvg(clientX: number, clientY: number) {
    const matrix = svgElement?.getScreenCTM();
    if (!matrix) return null;

    try {
      return new DOMPoint(clientX, clientY).matrixTransform(matrix.inverse());
    } catch {
      return null;
    }
  }

  function zoomAt(nextScale: number, clientX: number, clientY: number) {
    const clampedScale = clampScale(nextScale);
    if (clampedScale === scale) return;

    const cursor = clientToSvg(clientX, clientY);
    if (!cursor) return;

    const ratioX = Math.min(1, Math.max(0, (cursor.x - viewX) / viewWidth));
    const ratioY = Math.min(1, Math.max(0, (cursor.y - viewY) / viewHeight));
    const anchorX = viewX + ratioX * viewWidth;
    const anchorY = viewY + ratioY * viewHeight;
    const nextViewWidth = baseViewBox.width / clampedScale;
    const nextViewHeight = baseViewBox.height / clampedScale;

    viewX = anchorX - ratioX * nextViewWidth;
    viewY = anchorY - ratioY * nextViewHeight;
    scale = clampedScale;
  }

  function zoomFromCenter(delta: number) {
    const bounds = viewport.getBoundingClientRect();
    zoomAt(
      scale + delta,
      bounds.left + bounds.width / 2,
      bounds.top + bounds.height / 2,
    );
  }

  function fitView() {
    scale = 1;
    viewX = baseViewBox.x;
    viewY = baseViewBox.y;
  }

  function handleWheel(event: WheelEvent) {
    event.preventDefault();
    const zoomFactor = Math.exp(-event.deltaY * 0.0015);
    zoomAt(scale * zoomFactor, event.clientX, event.clientY);
  }

  function handlePointerDown(event: PointerEvent) {
    if (event.pointerType === "mouse" && event.button !== 0) return;

    event.preventDefault();
    activePointerId = event.pointerId;
    pointerX = event.clientX;
    pointerY = event.clientY;
    isPanning = true;
    viewport.setPointerCapture(event.pointerId);
  }

  function handlePointerMove(event: PointerEvent) {
    if (event.pointerId !== activePointerId) return;

    const previousPoint = clientToSvg(pointerX, pointerY);
    const currentPoint = clientToSvg(event.clientX, event.clientY);
    if (!previousPoint || !currentPoint) return;

    viewX -= currentPoint.x - previousPoint.x;
    viewY -= currentPoint.y - previousPoint.y;
    pointerX = event.clientX;
    pointerY = event.clientY;
  }

  function stopPanning(event: PointerEvent) {
    if (event.pointerId !== activePointerId) return;

    if (viewport.hasPointerCapture(event.pointerId)) {
      viewport.releasePointerCapture(event.pointerId);
    }
    activePointerId = null;
    isPanning = false;
  }

  function handleKeydown(event: KeyboardEvent) {
    switch (event.key) {
      case "+":
      case "=":
        event.preventDefault();
        zoomFromCenter(SCALE_STEP);
        break;
      case "-":
        event.preventDefault();
        zoomFromCenter(-SCALE_STEP);
        break;
      case "0":
        event.preventDefault();
        fitView();
        break;
      case "ArrowLeft":
        event.preventDefault();
        viewX += viewWidth * KEYBOARD_PAN_RATIO;
        break;
      case "ArrowRight":
        event.preventDefault();
        viewX -= viewWidth * KEYBOARD_PAN_RATIO;
        break;
      case "ArrowUp":
        event.preventDefault();
        viewY += viewHeight * KEYBOARD_PAN_RATIO;
        break;
      case "ArrowDown":
        event.preventDefault();
        viewY -= viewHeight * KEYBOARD_PAN_RATIO;
        break;
    }
  }
</script>

<div class="relative h-full w-full min-w-0 overflow-hidden">
  <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
  <div
    use:attachViewport
    class:cursor-grabbing={isPanning}
    class="absolute inset-0 block h-full w-full touch-none cursor-grab overflow-hidden border-0 bg-transparent p-0 text-inherit select-none outline-none focus-visible:ring-2 focus-visible:ring-ring/40 focus-visible:ring-inset"
    role="application"
    tabindex="0"
    aria-label={ariaLabel}
  >
    <span class="svg-stage grid h-full w-full place-items-center p-6 sm:p-10">
      <span
        class="svg-content grid h-full w-full min-w-72 place-items-center"
        use:renderSvg={svg}
      ></span>
    </span>
  </div>

  <Tooltip.Provider>
    <ButtonGroup.Root
      class="absolute right-3 bottom-3 z-20 overflow-hidden rounded-4xl bg-background/95 shadow-md backdrop-blur"
      aria-label="SVG zoom controls"
    >
      <Tooltip.Root>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="outline"
              size="icon-sm"
              disabled={!canZoomOut}
              onclick={() => zoomFromCenter(-SCALE_STEP)}
              aria-label="Zoom out"
            >
              <MinusIcon aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content>Zoom out</Tooltip.Content>
      </Tooltip.Root>

      <div
        class="border-border flex min-w-14 items-center justify-center border-y bg-background px-2 text-[0.65rem] font-medium tabular-nums"
        aria-live="polite"
        aria-label={`Zoom ${scalePercent}%`}
      >
        {scalePercent}%
      </div>

      <Tooltip.Root>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="outline"
              size="icon-sm"
              disabled={!canZoomIn}
              onclick={() => zoomFromCenter(SCALE_STEP)}
              aria-label="Zoom in"
            >
              <PlusIcon aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content>Zoom in</Tooltip.Content>
      </Tooltip.Root>

      <Tooltip.Root>
        <Tooltip.Trigger>
          {#snippet child({ props })}
            <Button
              {...props}
              variant="outline"
              size="icon-sm"
              onclick={fitView}
              aria-label="Fit SVG to view"
            >
              <ScanIcon aria-hidden="true" />
            </Button>
          {/snippet}
        </Tooltip.Trigger>
        <Tooltip.Content>Fit to view</Tooltip.Content>
      </Tooltip.Root>
    </ButtonGroup.Root>
  </Tooltip.Provider>
</div>

<style>
  .svg-content :global(svg) {
    display: block;
    width: 100% !important;
    height: 100% !important;
    max-width: none !important;
    max-height: none !important;
    filter: drop-shadow(0 0.75rem 1.25rem rgb(15 23 42 / 0.06));
  }
</style>
