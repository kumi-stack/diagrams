import { describe, expect, it } from "vitest";
import {
  cleanDiagramOverrides,
  resolveDiagramConfig,
  type DiagramConfigOverrides,
} from "./diagram-config";

describe("resolveDiagramConfig", () => {
  it("uses application defaults when no overrides exist", () => {
    const result = resolveDiagramConfig();

    expect(result.common).toEqual({
      theme: "studio",
      fontFamily: "jetbrainsMono",
      background: "transparent",
    });
    expect(result.sources.theme).toBe("application");
  });

  it("merges individual fields in diagram, project, global order", () => {
    const global: DiagramConfigOverrides = {
      common: { theme: "dark", fontFamily: "systemSans" },
    };
    const project: DiagramConfigOverrides = {
      common: { theme: "forest" },
      types: { mermaid: { look: "neo" } },
    };
    const diagram: DiagramConfigOverrides = {
      common: { fontFamily: "systemSerif" },
      types: { mermaid: { curve: "linear" } },
    };

    const result = resolveDiagramConfig(global, project, diagram);

    expect(result.common.theme).toBe("forest");
    expect(result.sources.theme).toBe("project");
    expect(result.common.fontFamily).toBe("systemSerif");
    expect(result.sources.fontFamily).toBe("diagram");
    expect(result.types.mermaid.look).toBe("neo");
    expect(result.types.mermaid.curve).toBe("linear");
  });

  it("keeps Mermaid overrides independent from common settings", () => {
    const result = resolveDiagramConfig(
      { common: { background: "white" } },
      { types: { mermaid: { look: "handDrawn" } } },
    );

    expect(result.common.background).toBe("white");
    expect(result.types.mermaid.look).toBe("handDrawn");
    expect(result.sources.background).toBe("global");
    expect(result.sources.look).toBe("project");
  });
});

describe("cleanDiagramOverrides", () => {
  it("removes empty branches created by resetting fields to inherit", () => {
    expect(
      cleanDiagramOverrides({
        common: {},
        types: { mermaid: {} },
      }),
    ).toEqual({});
  });
});
