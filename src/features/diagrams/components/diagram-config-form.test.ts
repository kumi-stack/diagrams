// @vitest-environment jsdom
import { fireEvent, render, screen, waitFor } from "@testing-library/svelte";
import { beforeEach, describe, expect, it, vi } from "vitest";
import TestWrapper from "./diagram-config-form.test-wrapper.svelte";

const mermaidMocks = vi.hoisted(() => ({
  initialize: vi.fn(),
  parse: vi.fn(async () => true),
  render: vi.fn(async () => ({ svg: "" })),
}));

vi.mock("mermaid", () => ({
  default: mermaidMocks,
}));

describe("DiagramConfigForm", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("re-renders the preview when a setting changes", async () => {
    render(TestWrapper);

    const theme = screen.getByLabelText("Theme");
    await fireEvent.change(theme, { target: { value: "dark" } });

    await waitFor(
      () => {
        expect(mermaidMocks.initialize).toHaveBeenCalledWith(
          expect.objectContaining({ theme: "dark" }),
        );
      },
      { timeout: 1_500 },
    );
  });
});
