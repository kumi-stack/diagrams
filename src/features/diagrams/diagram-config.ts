import type { MermaidConfig } from "mermaid";
import type { PngOptions } from "@/api/projects";

export type DiagramTheme =
  | "studio"
  | "default"
  | "base"
  | "dark"
  | "forest"
  | "neutral";
export type DiagramFontFamily =
  | "jetbrainsMono"
  | "systemSans"
  | "systemSerif";
export type DiagramBackground = "transparent" | "white";
export type MermaidLook = "classic" | "handDrawn" | "neo";
export type MermaidCurve = "basis" | "linear" | "rounded" | "step";
export type DiagramConfigLevel = "application" | "global" | "project" | "diagram";

export interface DiagramConfigOverrides {
  common?: {
    theme?: DiagramTheme;
    fontFamily?: DiagramFontFamily;
    background?: DiagramBackground;
  };
  types?: {
    mermaid?: {
      look?: MermaidLook;
      curve?: MermaidCurve;
    };
  };
}

export interface ResolvedDiagramConfig {
  common: {
    theme: DiagramTheme;
    fontFamily: DiagramFontFamily;
    background: DiagramBackground;
  };
  types: {
    mermaid: {
      look: MermaidLook;
      curve: MermaidCurve;
    };
  };
  sources: Record<DiagramConfigField, DiagramConfigLevel>;
}

export interface ProjectDiagramConfig {
  version: 1;
  defaults: DiagramConfigOverrides;
  diagrams: Record<string, DiagramConfigOverrides>;
}

export type DiagramConfigField =
  | "theme"
  | "fontFamily"
  | "background"
  | "look"
  | "curve";

export const applicationDiagramConfig = {
  common: {
    theme: "studio",
    fontFamily: "jetbrainsMono",
    background: "transparent",
  },
  types: {
    mermaid: { look: "classic", curve: "basis" },
  },
} as const;

const fieldPaths: Record<DiagramConfigField, readonly string[]> = {
  theme: ["common", "theme"],
  fontFamily: ["common", "fontFamily"],
  background: ["common", "background"],
  look: ["types", "mermaid", "look"],
  curve: ["types", "mermaid", "curve"],
};

function readPath(value: unknown, path: readonly string[]): unknown {
  let current = value;
  for (const segment of path) {
    if (!current || typeof current !== "object") return undefined;
    current = (current as Record<string, unknown>)[segment];
  }
  return current;
}

export function resolveDiagramConfig(
  global: DiagramConfigOverrides = {},
  project: DiagramConfigOverrides = {},
  diagram: DiagramConfigOverrides = {},
): ResolvedDiagramConfig {
  const layers = [
    ["diagram", diagram],
    ["project", project],
    ["global", global],
    ["application", applicationDiagramConfig],
  ] as const;
  const values = {} as Record<DiagramConfigField, unknown>;
  const sources = {} as Record<DiagramConfigField, DiagramConfigLevel>;

  for (const [field, path] of Object.entries(fieldPaths) as [
    DiagramConfigField,
    readonly string[],
  ][]) {
    for (const [level, layer] of layers) {
      const value = readPath(layer, path);
      if (value !== undefined) {
        values[field] = value;
        sources[field] = level;
        break;
      }
    }
  }

  return {
    common: {
      theme: values.theme as DiagramTheme,
      fontFamily: values.fontFamily as DiagramFontFamily,
      background: values.background as DiagramBackground,
    },
    types: {
      mermaid: {
        look: values.look as MermaidLook,
        curve: values.curve as MermaidCurve,
      },
    },
    sources,
  };
}

export function cleanDiagramOverrides(
  overrides: DiagramConfigOverrides,
): DiagramConfigOverrides {
  const result = structuredClone(overrides);
  if (result.common && Object.keys(result.common).length === 0) delete result.common;
  if (result.types?.mermaid && Object.keys(result.types.mermaid).length === 0) {
    delete result.types.mermaid;
  }
  if (result.types && Object.keys(result.types).length === 0) delete result.types;
  return result;
}

const fontFamilies: Record<DiagramFontFamily, string> = {
  jetbrainsMono: "JetBrains Mono Variable, monospace",
  systemSans: "system-ui, sans-serif",
  systemSerif: "ui-serif, Georgia, serif",
};

export function toMermaidConfig(config: ResolvedDiagramConfig): MermaidConfig {
  const studio = config.common.theme === "studio";
  return {
    startOnLoad: false,
    suppressErrorRendering: true,
    securityLevel: "strict",
    htmlLabels: false,
    theme: studio ? "base" : (config.common.theme as Exclude<DiagramTheme, "studio">),
    themeVariables: studio
      ? {
          primaryColor: "#ecfdf8",
          primaryTextColor: "#172423",
          primaryBorderColor: "#4b9f91",
          lineColor: "#657c78",
          secondaryColor: "#f4f8f7",
          tertiaryColor: "#ffffff",
          fontFamily: fontFamilies[config.common.fontFamily],
        }
      : { fontFamily: fontFamilies[config.common.fontFamily] },
    fontFamily: fontFamilies[config.common.fontFamily],
    look: config.types.mermaid.look,
    flowchart: { curve: config.types.mermaid.curve },
  };
}

export function toPngOptions(config: ResolvedDiagramConfig): PngOptions {
  return {
    scale: 2,
    background: config.common.background,
  };
}

export const configOptions = {
  theme: [
    ["studio", "Studio"],
    ["default", "Default"],
    ["base", "Base"],
    ["dark", "Dark"],
    ["forest", "Forest"],
    ["neutral", "Neutral"],
  ],
  fontFamily: [
    ["jetbrainsMono", "JetBrains Mono"],
    ["systemSans", "System sans"],
    ["systemSerif", "System serif"],
  ],
  background: [
    ["transparent", "Transparent"],
    ["white", "White"],
  ],
  look: [
    ["classic", "Classic"],
    ["handDrawn", "Hand drawn"],
    ["neo", "Neo"],
  ],
  curve: [
    ["basis", "Smooth"],
    ["linear", "Linear"],
    ["rounded", "Rounded"],
    ["step", "Step"],
  ],
} as const;
