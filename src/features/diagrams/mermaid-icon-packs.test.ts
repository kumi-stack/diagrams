import { describe, expect, it, vi } from "vitest";
import { registerMermaidIconPacks } from "./mermaid-icon-packs";

describe("registerMermaidIconPacks", () => {
  it("registers a referenced theSVG icon under the icons alias", async () => {
    const registerIconPacks = vi.fn();
    const fetchMock = vi
      .spyOn(globalThis, "fetch")
      .mockResolvedValue(
        new Response(
          '<?xml version="1.0" encoding="UTF-8"?><svg viewBox="0 0 24 32"><script>alert(1)</script><path onload="alert(2)" d="M0 0h24v32H0z"/></svg>',
        ),
      );

    await registerMermaidIconPacks(
      { registerIconPacks },
      "architecture-beta\nservice github(icons:github)[GitHub]",
    );

    expect(fetchMock).toHaveBeenCalledWith(
      "https://cdn.jsdelivr.net/gh/glincker/thesvg@main/public/icons/github/default.svg",
    );
    expect(registerIconPacks).toHaveBeenLastCalledWith(
      expect.arrayContaining([
      expect.objectContaining({
        name: "icons",
        icons: expect.objectContaining({
          prefix: "icons",
          icons: {
            github: {
              body: '<path d="M0 0h24v32H0z"/>',
              width: 24,
              height: 32,
            },
          },
        }),
      }),
      ]),
    );

    fetchMock.mockRestore();
  });
});
