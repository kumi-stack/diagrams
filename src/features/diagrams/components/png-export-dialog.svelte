<script lang="ts">
  import { save } from "@tauri-apps/plugin-dialog";
  import { toast } from "svelte-sonner";
  import { Button } from "$lib/components/ui/button";
  import * as Dialog from "$lib/components/ui/dialog";
  import { Label } from "$lib/components/ui/label";
  import * as RadioGroup from "$lib/components/ui/radio-group";
  import { Spinner } from "$lib/components/ui/spinner";
  import {
    errorMessage,
    exportApi,
    type PngBackground,
    type PngMetadata,
    type PngScale,
  } from "@/api/projects";

  let {
    open = $bindable(false),
    svg,
    diagramPath,
  }: {
    open?: boolean;
    svg: string;
    diagramPath: string;
  } = $props();

  let scale = $state("2");
  let background = $state<PngBackground>("transparent");
  let metadata = $state<PngMetadata | null>(null);
  let inspecting = $state(false);
  let saving = $state(false);
  let inspectionSequence = 0;

  let baseName = $derived(
    (diagramPath.split("/").pop() ?? "diagram").replace(/\.[^.]+$/, ""),
  );
  let pngScale = $derived(Number(scale) as PngScale);

  $effect(() => {
    if (!open || !svg) return;

    const currentSequence = ++inspectionSequence;
    const options = { scale: pngScale, background };
    inspecting = true;

    void exportApi
      .inspectPng(svg, options)
      .then((result) => {
        if (currentSequence === inspectionSequence) metadata = result;
      })
      .catch((cause) => {
        if (currentSequence === inspectionSequence) {
          metadata = null;
          toast.error(errorMessage(cause));
        }
      })
      .finally(() => {
        if (currentSequence === inspectionSequence) inspecting = false;
      });
  });

  async function savePng() {
    const path = await save({
      title: "Save diagram as PNG",
      defaultPath: `${baseName}.png`,
      filters: [{ name: "PNG image", extensions: ["png"] }],
    });
    if (!path) return;

    const pngPath = path.toLowerCase().endsWith(".png") ? path : `${path}.png`;
    saving = true;
    try {
      const result = await exportApi.savePng(svg, pngPath, {
        scale: pngScale,
        background,
      });
      open = false;
      toast.success(`Saved PNG (${result.width} x ${result.height})`);
    } catch (cause) {
      toast.error(errorMessage(cause));
    } finally {
      saving = false;
    }
  }
</script>

<Dialog.Root bind:open>
  <Dialog.Content>
    <Dialog.Header>
      <Dialog.Title>Export PNG</Dialog.Title>
      <Dialog.Description>
        Choose raster size and background, then select where to save the image.
      </Dialog.Description>
    </Dialog.Header>

    <div class="grid gap-5">
      <fieldset class="grid gap-3">
        <legend class="text-sm font-medium">Scale</legend>
        <RadioGroup.Root
          bind:value={scale}
          class="grid grid-cols-3 gap-2"
        >
          {#each ["1", "2", "3"] as value (value)}
            <Label
              class="has-data-checked:border-primary has-data-checked:bg-primary/5 flex cursor-pointer items-center gap-2 rounded-xl border p-3"
            >
              <RadioGroup.Item value={value} />
              {value}x
            </Label>
          {/each}
        </RadioGroup.Root>
      </fieldset>

      <fieldset class="grid gap-3">
        <legend class="text-sm font-medium">Background</legend>
        <RadioGroup.Root bind:value={background} class="grid grid-cols-2 gap-2">
          <Label
            class="has-data-checked:border-primary has-data-checked:bg-primary/5 flex cursor-pointer items-center gap-2 rounded-xl border p-3"
          >
            <RadioGroup.Item value="transparent" />
            Transparent
          </Label>
          <Label
            class="has-data-checked:border-primary has-data-checked:bg-primary/5 flex cursor-pointer items-center gap-2 rounded-xl border p-3"
          >
            <RadioGroup.Item value="white" />
            White
          </Label>
        </RadioGroup.Root>
      </fieldset>

      <div class="bg-muted/60 flex min-h-10 items-center justify-between rounded-xl px-3 text-xs">
        <span class="text-muted-foreground">Output size</span>
        {#if inspecting}
          <Spinner class="size-3.5" />
        {:else if metadata}
          <span class="font-medium">{metadata.width} x {metadata.height} px</span>
        {:else}
          <span class="text-destructive">Unavailable</span>
        {/if}
      </div>
    </div>

    <Dialog.Footer>
      <Button variant="outline" onclick={() => (open = false)} disabled={saving}>
        Cancel
      </Button>
      <Button onclick={savePng} disabled={saving || inspecting || !metadata}>
        {#if saving}<Spinner class="size-3.5" />{/if}
        Save PNG
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
