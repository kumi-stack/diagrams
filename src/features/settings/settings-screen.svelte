<script lang="ts">
  import { resolve } from "$app/paths";
  import { onNavigate } from "$app/navigation";
  import ArrowLeftIcon from "@lucide/svelte/icons/arrow-left";
  import BotIcon from "@lucide/svelte/icons/bot";
  import RefreshCwIcon from "@lucide/svelte/icons/refresh-cw";
  import SaveIcon from "@lucide/svelte/icons/save";
  import SlidersHorizontalIcon from "@lucide/svelte/icons/sliders-horizontal";
  import { onMount } from "svelte";
  import { toast } from "svelte-sonner";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import { Label } from "$lib/components/ui/label";
  import * as NativeSelect from "$lib/components/ui/native-select";
  import { Spinner } from "$lib/components/ui/spinner";
  import { Switch } from "$lib/components/ui/switch";
  import { aiApi, settingsApi, type OllamaModel } from "@/api/settings";
  import { errorMessage } from "@/api/projects";
  import DiagramConfigForm from "@/features/diagrams/components/diagram-config-form.svelte";
  import { ConfigAutosave } from "@/features/diagrams/config-autosave.svelte";
  import {
    resolveDiagramConfig,
    type DiagramConfigOverrides,
  } from "@/features/diagrams/diagram-config";

  let enabled = $state(false);
  let selectedModel = $state("");
  let models = $state<OllamaModel[]>([]);
  let loading = $state(true);
  let loadingModels = $state(false);
  let saving = $state(false);
  let error = $state("");
  let connectionError = $state("");
  let diagramDefaults = $state<DiagramConfigOverrides>({});
  let diagramConfigLoaded = $state(false);
  let resolvedDiagramConfig = $derived(resolveDiagramConfig(diagramDefaults));
  const diagramAutosave = new ConfigAutosave<DiagramConfigOverrides>((value) =>
    settingsApi.saveDiagramDefaults(value),
  );

  onMount(loadSettings);
  onNavigate(async () => {
    await diagramAutosave.flush();
  });

  async function loadSettings() {
    loading = true;
    diagramConfigLoaded = false;
    error = "";
    try {
      const settings = await settingsApi.get();
      enabled = settings.ollama.enabled;
      selectedModel = settings.ollama.model ?? "";
      diagramDefaults = settings.diagramDefaults;
      diagramAutosave.markSaved(diagramDefaults);
      diagramConfigLoaded = true;
      if (enabled) await refreshModels();
    } catch (cause) {
      error = errorMessage(cause);
    } finally {
      loading = false;
    }
  }

  async function refreshModels() {
    loadingModels = true;
    connectionError = "";
    try {
      models = await aiApi.listModels();
      if (
        selectedModel &&
        !models.some((model) => model.name === selectedModel)
      ) {
        models = [{ name: selectedModel }, ...models];
      }
    } catch (cause) {
      models = selectedModel ? [{ name: selectedModel }] : [];
      connectionError = errorMessage(cause);
    } finally {
      loadingModels = false;
    }
  }

  async function saveSettings() {
    saving = true;
    error = "";
    try {
      await diagramAutosave.flush();
      await settingsApi.save({
        ollama: {
          enabled,
          model: selectedModel || null,
        },
        diagramDefaults,
      });
      toast.success("Settings saved");
    } catch (cause) {
      error = errorMessage(cause);
    } finally {
      saving = false;
    }
  }

  async function toggleOllama(nextEnabled: boolean) {
    enabled = nextEnabled;
    if (nextEnabled && models.length === 0) await refreshModels();
  }

  $effect(() => {
    if (diagramConfigLoaded) diagramAutosave.schedule(diagramDefaults);
  });
</script>

<main class="bg-muted/30 min-h-screen p-4 sm:p-8">
  <div class="mx-auto grid w-full max-w-3xl gap-5">
    <header class="flex items-center gap-3">
      <Button href={resolve("/")} variant="outline" size="icon-sm" aria-label="Back to projects">
        <ArrowLeftIcon />
      </Button>
      <div>
        <h1 class="text-lg font-semibold">Settings</h1>
        <p class="text-muted-foreground text-xs">
          Configure local AI integration for Diagram Studio.
        </p>
      </div>
    </header>

    <Card.Root>
      <Card.Header>
        <div class="flex items-center gap-3">
          <div class="bg-primary/10 text-primary grid size-10 place-items-center rounded-2xl">
            <BotIcon class="size-5" />
          </div>
          <div>
            <Card.Title>Ollama</Card.Title>
            <Card.Description>Local server at localhost:11434</Card.Description>
          </div>
        </div>
      </Card.Header>

      <Card.Content class="grid gap-6">
        {#if loading}
          <div class="text-muted-foreground flex items-center gap-2 text-sm">
            <Spinner />
            Loading settings
          </div>
        {:else}
          <div class="flex items-center justify-between gap-4 rounded-3xl border p-4">
            <div>
              <Label for="ollama-enabled">Enable Ollama</Label>
              <p class="text-muted-foreground mt-1 text-xs">
                Show AI generation when creating Mermaid diagrams.
              </p>
            </div>
            <Switch
              id="ollama-enabled"
              checked={enabled}
              onCheckedChange={toggleOllama}
            />
          </div>

          <div class="grid gap-2">
            <div class="flex items-center justify-between gap-3">
              <Label for="ollama-model">Local model</Label>
              <Button
                variant="ghost"
                size="xs"
                disabled={!enabled || loadingModels}
                onclick={refreshModels}
              >
                <RefreshCwIcon class={loadingModels ? "animate-spin" : ""} />
                Refresh
              </Button>
            </div>
            <NativeSelect.Root
              id="ollama-model"
              bind:value={selectedModel}
              class="w-full"
              disabled={!enabled || loadingModels}
              aria-label="Ollama model"
            >
              <NativeSelect.Option value="">
                {loadingModels ? "Loading models..." : "Select a model"}
              </NativeSelect.Option>
              {#each models as model (model.name)}
                <NativeSelect.Option value={model.name}>{model.name}</NativeSelect.Option>
              {/each}
            </NativeSelect.Root>
            {#if connectionError}
              <p class="text-destructive text-xs" role="alert">{connectionError}</p>
            {:else if enabled && !loadingModels}
              <p class="text-muted-foreground text-xs">
                Connected. {models.length} local {models.length === 1 ? "model" : "models"} available.
              </p>
            {/if}
          </div>
        {/if}

        {#if error}
          <p class="text-destructive text-xs" role="alert">{error}</p>
        {/if}
      </Card.Content>

      <Card.Footer class="justify-end border-t">
        <Button
          disabled={loading || saving || (enabled && !selectedModel)}
          onclick={saveSettings}
        >
          {#if saving}
            <Spinner />
          {:else}
            <SaveIcon />
          {/if}
          Save settings
        </Button>
      </Card.Footer>
    </Card.Root>

    <Card.Root>
      <Card.Header>
        <div class="flex items-center gap-3">
          <div class="bg-primary/10 text-primary grid size-10 place-items-center rounded-2xl">
            <SlidersHorizontalIcon class="size-5" />
          </div>
          <div>
            <Card.Title>Diagram defaults</Card.Title>
            <Card.Description>
              Global defaults inherited by every project and diagram.
            </Card.Description>
          </div>
        </div>
      </Card.Header>
      <Card.Content>
        {#if loading}
          <div class="text-muted-foreground flex items-center gap-2 text-sm">
            <Spinner /> Loading configuration
          </div>
        {:else if !diagramConfigLoaded}
          <div class="border-destructive/30 bg-destructive/5 grid gap-3 rounded-2xl border p-4">
            <p class="text-destructive text-xs">
              Diagram defaults could not be loaded. No changes will be saved.
            </p>
            <Button variant="outline" size="sm" onclick={loadSettings}>Retry</Button>
          </div>
        {:else}
          <DiagramConfigForm
            bind:value={diagramDefaults}
            resolved={resolvedDiagramConfig}
            saveStatus={diagramAutosave.status}
            saveError={diagramAutosave.error}
            onretry={diagramAutosave.retry}
            showPreview
          />
        {/if}
      </Card.Content>
    </Card.Root>
  </div>
</main>
