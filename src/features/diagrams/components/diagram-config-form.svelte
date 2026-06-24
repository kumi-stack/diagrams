<script lang="ts">
  import { Button } from "$lib/components/ui/button";
  import { Label } from "$lib/components/ui/label";
  import * as NativeSelect from "$lib/components/ui/native-select";
  import { Spinner } from "$lib/components/ui/spinner";
  import type { SaveStatus } from "@/api/projects";
  import {
    cleanDiagramOverrides,
    configOptions,
    type DiagramConfigField,
    type DiagramConfigOverrides,
    type ResolvedDiagramConfig,
  } from "../diagram-config";
  import MermaidPreview from "./mermaid-preview.svelte";

  let {
    value = $bindable({}),
    resolved,
    saveStatus = "idle",
    saveError = "",
    onretry,
    showPreview = false,
  }: {
    value?: DiagramConfigOverrides;
    resolved: ResolvedDiagramConfig;
    saveStatus?: SaveStatus;
    saveError?: string;
    onretry?: () => void;
    showPreview?: boolean;
  } = $props();

  const sample = `flowchart LR
  A[Client] --> B{Gateway}
  B -->|Allowed| C[(Service)]`;

  function ownValue(field: DiagramConfigField): string {
    const values: Record<DiagramConfigField, unknown> = {
      theme: value.common?.theme,
      fontFamily: value.common?.fontFamily,
      background: value.common?.background,
      look: value.types?.mermaid?.look,
      curve: value.types?.mermaid?.curve,
    };
    return values[field] === undefined ? "__inherit" : String(values[field]);
  }

  function update(field: DiagramConfigField, raw: string) {
    const next: DiagramConfigOverrides = {
      common: value.common ? { ...value.common } : undefined,
      types: value.types
        ? {
            ...value.types,
            mermaid: value.types.mermaid
              ? { ...value.types.mermaid }
              : undefined,
          }
        : undefined,
    };
    const inherited = raw === "__inherit";
    if (["theme", "fontFamily", "background"].includes(field)) {
      next.common ??= {};
      if (inherited) delete next.common[field as keyof typeof next.common];
      else Object.assign(next.common, { [field]: raw });
    } else {
      next.types ??= {};
      next.types.mermaid ??= {};
      const mermaidField = field as "look" | "curve";
      if (inherited) delete next.types.mermaid[mermaidField];
      else Object.assign(next.types.mermaid, { [mermaidField]: raw });
    }
    value = cleanDiagramOverrides(next);
  }

  const fields = [
    { key: "theme", label: "Theme", options: configOptions.theme },
    { key: "fontFamily", label: "Font", options: configOptions.fontFamily },
    { key: "background", label: "Background", options: configOptions.background },
    { key: "look", label: "Mermaid look", options: configOptions.look },
    { key: "curve", label: "Connection curve", options: configOptions.curve },
  ] as const;
</script>

<div class="grid gap-5">
  <div class="grid gap-3 sm:grid-cols-2">
    {#each fields as field (field.key)}
      <div class="grid gap-1.5">
        <Label for={`diagram-config-${field.key}`}>{field.label}</Label>
        <NativeSelect.Root
          id={`diagram-config-${field.key}`}
          value={ownValue(field.key)}
          class="w-full"
          onchange={(event) =>
            update(field.key, event.currentTarget.value)}
        >
          <NativeSelect.Option value="__inherit">
            Inherit ({resolved.sources[field.key]})
          </NativeSelect.Option>
          {#each field.options as option (option[0])}
            <NativeSelect.Option value={option[0]}>{option[1]}</NativeSelect.Option>
          {/each}
        </NativeSelect.Root>
        <p class="text-muted-foreground text-[0.65rem]">
          Effective source: {resolved.sources[field.key]}
        </p>
      </div>
    {/each}
  </div>

  {#if showPreview}
    <MermaidPreview
      source={sample}
      config={resolved}
      editorHidden={true}
      onToggleEditor={() => {}}
      compact
    />
  {/if}

  <div class="flex min-h-8 items-center justify-between gap-3 text-xs">
    {#if saveStatus === "saving"}
      <span class="text-muted-foreground flex items-center gap-2">
        <Spinner class="size-3.5" /> Saving configuration
      </span>
    {:else if saveStatus === "error"}
      <span class="text-destructive truncate" title={saveError}>Save failed</span>
      <Button variant="destructive" size="xs" onclick={onretry}>Retry</Button>
    {:else if saveStatus === "saved"}
      <span class="text-muted-foreground">Configuration saved</span>
    {/if}
  </div>
</div>
