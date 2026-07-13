import type mermaid from "mermaid";

export type MermaidIconPackRegistry = Pick<
  typeof mermaid,
  "registerIconPacks"
>;
type RegisteredIconPack = Parameters<
  MermaidIconPackRegistry["registerIconPacks"]
>[0][number];

const logos = new Map<string, TheSvgIcon>();
const iconPackNames = ["logos", "icons"] as const;

interface TheSvgIcon {
  body: string;
  width: number;
  height: number;
}

/**
 * Fetches only the theSVG icons referenced as `logos:*` or `icons:*` in the
 * current diagram. `icons:*` is retained as a friendly alias.
 */
export async function registerMermaidIconPacks(
  mermaid: MermaidIconPackRegistry,
  source: string,
) {
  const iconNames = getLogosIconNames(source);
  if (iconNames.length === 0) return;

  const missingIconNames = iconNames.filter((name) => !logos.has(name));
  const loadedIcons = await Promise.all(
    missingIconNames.map(async (name) => [name, await loadLogo(name)] as const),
  );
  for (const [name, icon] of loadedIcons) {
    logos.set(name, icon);
  }

  mermaid.registerIconPacks([
    ...iconPackNames.map(
      (name) =>
        ({
          name,
          icons: {
            prefix: name,
            icons: Object.fromEntries(logos),
          },
        }) satisfies RegisteredIconPack,
    ),
  ]);
}

function getLogosIconNames(source: string) {
  return [
    ...new Set(
      [...source.matchAll(/\b(?:logos|icons):([a-z0-9-]+)\b/gi)].map((match) =>
        match[1].toLowerCase(),
      ),
    ),
  ];
}

function sanitizeSvgBody(body: string) {
  return body
    .replace(/<script\b[^>]*>[\s\S]*?<\/script\s*>/gi, "")
    .replace(/<foreignObject\b[^>]*>[\s\S]*?<\/foreignObject\s*>/gi, "")
    .replace(/\son[a-z-]+\s*=\s*(?:"[^"]*"|'[^']*'|[^\s>]+)/gi, "");
}

async function loadLogo(name: string): Promise<TheSvgIcon> {
  const response = await fetch(
    `https://cdn.jsdelivr.net/gh/glincker/thesvg@main/public/icons/${name}/default.svg`,
  );
  if (!response.ok) {
    throw new Error(`Could not load theSVG logo "${name}" (${response.status}).`);
  }

  return toIconifyIcon(await response.text());
}

function toIconifyIcon(svg: string): TheSvgIcon {
  const match = svg.match(/<svg\b([^>]*)>([\s\S]*?)<\/svg>/i);
  if (!match) {
    throw new Error("theSVG returned an invalid SVG document.");
  }

  const viewBox = match[1].match(/\bviewBox\s*=\s*["']([^"']+)["']/i)?.[1];
  const dimensions = viewBox?.trim().split(/[\s,]+/).map(Number);
  const width = dimensions?.[2];
  const height = dimensions?.[3];
  if (
    width === undefined ||
    height === undefined ||
    !Number.isFinite(width) ||
    !Number.isFinite(height)
  ) {
    throw new Error("theSVG icon is missing a valid viewBox.");
  }

  return { body: sanitizeSvgBody(match[2]), width, height };
}
