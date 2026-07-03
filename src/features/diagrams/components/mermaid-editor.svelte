<script lang="ts">
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import RotateCcwIcon from "@lucide/svelte/icons/rotate-ccw";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";
  import {
    getMermaidCompletionContext,
    getMermaidCompletions,
    type MermaidCompletionItem,
  } from "../mermaid-autocomplete";

  let {
    value = $bindable(),
    initialValue,
  }: {
    value: string;
    initialValue: string;
  } = $props();

  let editorElement: HTMLDivElement;
  let editor: Editor | undefined;
  let completionItems = $state<MermaidCompletionItem[]>([]);
  let activeCompletionIndex = $state(0);
  let completionPopupStyle = $state("");
  let suppressNextAutocomplete = false;

  function escapeHtml(source: string) {
    return source
      .replaceAll("&", "&amp;")
      .replaceAll("<", "&lt;")
      .replaceAll(">", "&gt;");
  }

  function setEditorContent(source: string) {
    editor?.commands.setContent(`<pre><code>${escapeHtml(source)}</code></pre>`);
  }

  function resetDiagram() {
    closeAutocomplete();
    value = initialValue;
    setEditorContent(initialValue);
  }

  function closeAutocomplete() {
    completionItems = [];
    activeCompletionIndex = 0;
    completionPopupStyle = "";
  }

  function updateAutocomplete() {
    const currentEditor = editor;
    if (!currentEditor) return;

    if (suppressNextAutocomplete) {
      suppressNextAutocomplete = false;
      closeAutocomplete();
      return;
    }

    const { selection } = currentEditor.state;
    if (!selection.empty || selection.$from.parent.type.name !== "codeBlock") {
      closeAutocomplete();
      return;
    }

    const source = currentEditor.getText({ blockSeparator: "\n" });
    const cursorOffset = selection.$from.parentOffset;
    const nextCompletions = getMermaidCompletions(source, cursorOffset).slice(0, 8);

    if (nextCompletions.length === 0) {
      closeAutocomplete();
      return;
    }

    const cursorPosition = currentEditor.view.coordsAtPos(selection.from);
    completionItems = nextCompletions;
    activeCompletionIndex = Math.min(activeCompletionIndex, nextCompletions.length - 1);
    completionPopupStyle = `left: ${cursorPosition.left}px; top: ${cursorPosition.bottom + 6}px;`;
  }

  function acceptCompletion(item = completionItems[activeCompletionIndex]) {
    const currentEditor = editor;
    if (!currentEditor || !item) return false;

    const { selection } = currentEditor.state;
    if (!selection.empty || selection.$from.parent.type.name !== "codeBlock") {
      closeAutocomplete();
      return false;
    }

    const source = currentEditor.getText({ blockSeparator: "\n" });
    const context = getMermaidCompletionContext(source, selection.$from.parentOffset);
    if (!context) {
      closeAutocomplete();
      return false;
    }

    suppressNextAutocomplete = true;
    currentEditor.view.dispatch(
      currentEditor.state.tr.insertText(
        item.label,
        selection.from - context.prefix.length,
        selection.from,
      ),
    );
    currentEditor.commands.focus();
    closeAutocomplete();
    return true;
  }

  function insertEditorTab() {
    const currentEditor = editor;
    if (!currentEditor) return false;

    const { selection } = currentEditor.state;
    if (selection.$from.parent.type.name !== "codeBlock") return false;

    currentEditor.view.dispatch(
      currentEditor.state.tr.insertText("\t", selection.from, selection.to),
    );
    currentEditor.commands.focus();
    closeAutocomplete();
    return true;
  }

  function handleAutocompleteKeyDown(event: KeyboardEvent) {
    if (completionItems.length === 0) {
      if (event.key === "Tab" && !event.shiftKey) {
        event.preventDefault();
        return insertEditorTab();
      }

      return false;
    }

    if ((event.key === "Enter" || event.key === "Tab") && !event.shiftKey) {
      event.preventDefault();
      return acceptCompletion();
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      activeCompletionIndex = (activeCompletionIndex + 1) % completionItems.length;
      return true;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      activeCompletionIndex =
        (activeCompletionIndex - 1 + completionItems.length) % completionItems.length;
      return true;
    }

    if (event.key === "Escape") {
      event.preventDefault();
      closeAutocomplete();
      return true;
    }

    return false;
  }

  function completionKindLabel(kind: MermaidCompletionItem["kind"]) {
    return kind === "identifier" ? "node" : kind;
  }

  onMount(() => {
    editor = new Editor({
      element: editorElement,
      extensions: [
        StarterKit.configure({
          codeBlock: {
            HTMLAttributes: {
              spellcheck: "false",
            },
          },
        }),
      ],
      content: `<pre><code>${escapeHtml(value)}</code></pre>`,
      autofocus: "end",
      editorProps: {
        attributes: {
          "aria-label": "Mermaid diagram source",
          class: "mermaid-source",
        },
        handleKeyDown: (_view, event) => handleAutocompleteKeyDown(event),
      },
      onUpdate: ({ editor: currentEditor }) => {
        value = currentEditor.getText({ blockSeparator: "\n" });
        queueMicrotask(updateAutocomplete);
      },
      onSelectionUpdate: () => {
        queueMicrotask(updateAutocomplete);
      },
    });

    return () => editor?.destroy();
  });
</script>

<Card.Root class="h-full min-h-[30rem] min-w-0 gap-0 py-0 shadow-sm xl:min-h-0">
  <Card.Header
    class="border-b px-4 py-4 sm:px-5"
  >
    <div>
      <Card.Description class="mb-1 text-[0.65rem] font-semibold tracking-[0.16em] uppercase">
        Input
      </Card.Description>
      <Card.Title class="text-sm">Mermaid source</Card.Title>
    </div>

    <Card.Action>
      <Button variant="outline" size="sm" onclick={resetDiagram}>
        <RotateCcwIcon data-icon="inline-start" />
        Reset
      </Button>
    </Card.Action>
  </Card.Header>

  <Card.Content class="grid min-h-0 flex-1 grid-cols-[2.75rem_minmax(0,1fr)] p-0">
    <div
      aria-hidden="true"
      class="text-muted-foreground/60 border-r bg-muted/20 pt-7 text-center text-[0.65rem]"
    >
      01
    </div>
    <div class="min-w-0 overflow-auto p-6" bind:this={editorElement}></div>
  </Card.Content>

  <Card.Footer
    class="text-muted-foreground min-h-9 justify-between border-t px-4 text-[0.65rem]"
  >
    <span>Mermaid syntax</span>
    <span>{value.length} characters</span>
  </Card.Footer>
</Card.Root>

{#if completionItems.length > 0}
  <div
    class="mermaid-completions"
    style={completionPopupStyle}
    role="listbox"
    aria-label="Mermaid autocomplete suggestions"
  >
    {#each completionItems as item, index (item.kind + item.label)}
      <button
        type="button"
        class:active={index === activeCompletionIndex}
        role="option"
        aria-selected={index === activeCompletionIndex}
        onmouseenter={() => (activeCompletionIndex = index)}
        onmousedown={(event) => {
          event.preventDefault();
          acceptCompletion(item);
        }}
      >
        <span>{item.label}</span>
        <small>{completionKindLabel(item.kind)}</small>
      </button>
    {/each}
  </div>
{/if}

<style>
  :global(.mermaid-source) {
    min-height: 100%;
    outline: none;
    caret-color: var(--primary);
  }

  :global(.mermaid-source pre) {
    margin: 0;
    padding: 0;
    color: var(--foreground);
    background: transparent;
    font-family: var(--font-mono);
    font-size: 0.78rem;
    line-height: 1.85;
    white-space: pre-wrap;
    word-break: normal;
  }

  :global(.mermaid-source code) {
    color: inherit;
    background: transparent;
  }

  :global(.mermaid-source .ProseMirror-selectednode) {
    outline: 2px solid color-mix(in oklab, var(--primary) 35%, transparent);
    outline-offset: 4px;
  }

  .mermaid-completions {
    position: fixed;
    z-index: 50;
    min-width: 11rem;
    max-width: min(20rem, calc(100vw - 2rem));
    overflow: hidden;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    background: var(--popover);
    color: var(--popover-foreground);
    box-shadow: 0 12px 28px color-mix(in oklab, var(--foreground) 16%, transparent);
  }

  .mermaid-completions button {
    display: flex;
    width: 100%;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.45rem 0.65rem;
    border: 0;
    color: inherit;
    background: transparent;
    font-family: var(--font-mono);
    font-size: 0.72rem;
    line-height: 1.2;
    text-align: left;
  }

  .mermaid-completions button:hover,
  .mermaid-completions button.active {
    background: var(--accent);
    color: var(--accent-foreground);
  }

  .mermaid-completions small {
    color: var(--muted-foreground);
    font-family: var(--font-sans);
    font-size: 0.62rem;
  }
</style>
