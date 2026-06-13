<script lang="ts">
  import { Editor } from "@tiptap/core";
  import StarterKit from "@tiptap/starter-kit";
  import RotateCcwIcon from "@lucide/svelte/icons/rotate-ccw";
  import { onMount } from "svelte";
  import { Button } from "$lib/components/ui/button";
  import * as Card from "$lib/components/ui/card";

  let {
    value = $bindable(),
    initialValue,
  }: {
    value: string;
    initialValue: string;
  } = $props();

  let editorElement: HTMLDivElement;
  let editor: Editor | undefined;

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
    value = initialValue;
    setEditorContent(initialValue);
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
      },
      onUpdate: ({ editor: currentEditor }) => {
        value = currentEditor.getText({ blockSeparator: "\n" });
      },
    });

    return () => editor?.destroy();
  });
</script>

<Card.Root class="h-full min-h-[30rem] gap-0 py-0 shadow-sm">
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
</style>
