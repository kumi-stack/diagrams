export type MermaidCompletionItem = {
  label: string;
  kind: "identifier" | "keyword" | "direction";
};

type DiagramKind = "flowchart" | "sequence" | "state" | "er" | "unknown";

export type MermaidCompletionContext = {
  prefix: string;
  from: number;
};

type CompletionParser = (source: string) => MermaidCompletionItem[];

const identifierPattern = "[A-Za-z_][A-Za-z0-9_]*";
const identifierRegex = new RegExp(identifierPattern, "g");

const flowchartKeywords = new Set([
  "flowchart",
  "graph",
  "subgraph",
  "direction",
  "end",
]);

const flowchartDirections = new Set(["TB", "TD", "BT", "RL", "LR"]);
const sequenceKeywords = new Set(["sequenceDiagram", "participant", "actor"]);
const stateKeywords = new Set(["stateDiagram-v2", "state"]);
const stateSpecials = new Set(["[*]"]);
const erKeywords = new Set(["erDiagram"]);

const allReservedWords = new Set([
  ...flowchartKeywords,
  ...flowchartDirections,
  ...sequenceKeywords,
  ...stateKeywords,
  ...stateSpecials,
  ...erKeywords,
  "as",
  "create",
]);

const parsers: Record<Exclude<DiagramKind, "unknown">, CompletionParser> = {
  flowchart: parseFlowchartCompletions,
  sequence: parseSequenceCompletions,
  state: parseStateCompletions,
  er: parseErCompletions,
};

export function getMermaidCompletions(
  source: string,
  cursorOffset: number,
): MermaidCompletionItem[] {
  const context = getMermaidCompletionContext(source, cursorOffset);
  if (!context) return [];

  const ignoredRanges = getIgnoredRanges(source);
  if (isOffsetIgnored(ignoredRanges, Math.max(context.from, 0))) {
    return [];
  }

  const diagramKind = detectDiagramKind(source);
  const parser = diagramKind === "unknown" ? undefined : parsers[diagramKind];
  const suggestions = parser ? parser(source) : [];
  const normalizedPrefix = context.prefix.toLowerCase();

  const matchingSuggestions = dedupeCompletions(suggestions).filter((suggestion) =>
    suggestion.label.toLowerCase().startsWith(normalizedPrefix) &&
    suggestion.label.toLowerCase() !== normalizedPrefix,
  );
  const matchingIdentifiers = matchingSuggestions.filter(
    (suggestion) => suggestion.kind === "identifier",
  );

  return (
    matchingIdentifiers.length > 0 ? matchingIdentifiers : matchingSuggestions
  ).sort(compareCompletionItems);
}

export function getMermaidCompletionContext(
  source: string,
  cursorOffset: number,
): MermaidCompletionContext | null {
  const offset = clamp(cursorOffset, 0, source.length);
  if (isInsideToken(source, offset)) return null;

  const beforeCursor = source.slice(0, offset);
  const bracketStateMatch = beforeCursor.match(/\[[*]?$/);
  const identifierMatch = beforeCursor.match(/[A-Za-z_][A-Za-z0-9_]*$/);

  const prefix = bracketStateMatch?.[0] ?? identifierMatch?.[0];
  if (!prefix) return null;

  return {
    prefix,
    from: offset - prefix.length,
  };
}

function detectDiagramKind(source: string): DiagramKind {
  for (const line of source.split("\n")) {
    const trimmed = stripComment(line).trim();
    if (!trimmed) continue;

    if (/^(flowchart|graph)\b/.test(trimmed)) return "flowchart";
    if (/^sequenceDiagram\b/.test(trimmed)) return "sequence";
    if (/^stateDiagram(?:-v2)?\b/.test(trimmed)) return "state";
    if (/^erDiagram\b/.test(trimmed)) return "er";

    return "unknown";
  }

  return "unknown";
}

function parseFlowchartCompletions(source: string) {
  const identifiers = new Set<string>();

  for (const line of source.split("\n")) {
    const lineWithoutComments = stripComment(line);
    const definitionLine = maskQuotedText(lineWithoutComments);
    const cleanLine = maskFlowchartLine(lineWithoutComments);
    const trimmed = cleanLine.trim();
    if (!trimmed) continue;

    const subgraphMatch = definitionLine.trim().match(
      new RegExp(`^subgraph\\s+(${identifierPattern})\\b`),
    );
    if (subgraphMatch) identifiers.add(subgraphMatch[1]);

    for (const match of definitionLine.matchAll(
      new RegExp(`(?:^|[\\s;])(${identifierPattern})\\s*(?=[\\[({])`, "g"),
    )) {
      identifiers.add(match[1]);
    }

    if (containsFlowchartArrow(cleanLine)) {
      addIdentifiersFromText(identifiers, cleanLine);
    }
  }

  return [
    ...toIdentifierCompletions(identifiers),
    ...toCompletions(flowchartKeywords, "keyword"),
    ...toCompletions(flowchartDirections, "direction"),
  ];
}

function parseSequenceCompletions(source: string) {
  const identifiers = new Set<string>();

  for (const line of source.split("\n")) {
    const originalLine = stripComment(line);
    const cleanLine = maskQuotedText(originalLine);
    const trimmed = cleanLine.trim();
    if (!trimmed) continue;

    const participantMatch = trimmed.match(
      new RegExp(
        `^(?:create\\s+)?(?:participant|actor)\\s+(${identifierPattern})(?:\\s+as\\s+(${identifierPattern}))?\\b`,
      ),
    );
    if (participantMatch) {
      identifiers.add(participantMatch[1]);
      if (participantMatch[2]) identifiers.add(participantMatch[2]);
    }

    const messageMatch = trimmed.match(
      new RegExp(
        `^(${identifierPattern})\\s*[-=x)(]+[>)]?\\+?\\s*(${identifierPattern})\\s*:`,
      ),
    );
    if (messageMatch) {
      identifiers.add(messageMatch[1]);
      identifiers.add(messageMatch[2]);
    }
  }

  return [
    ...toIdentifierCompletions(identifiers),
    ...toCompletions(sequenceKeywords, "keyword"),
  ];
}

function parseStateCompletions(source: string) {
  const identifiers = new Set<string>();

  for (const line of source.split("\n")) {
    const originalLine = stripComment(line);
    const cleanLine = maskQuotedText(originalLine);
    const trimmed = cleanLine.trim();
    if (!trimmed) continue;

    const quotedStateMatch = originalLine.match(
      new RegExp(`\\bstate\\s+"[^"]+"\\s+as\\s+(${identifierPattern})\\b`),
    );
    if (quotedStateMatch) identifiers.add(quotedStateMatch[1]);

    const stateMatch = trimmed.match(
      new RegExp(`^state\\s+(${identifierPattern})\\b`),
    );
    if (stateMatch) identifiers.add(stateMatch[1]);

    const transitionMatch = trimmed.match(
      new RegExp(
        `(\\[\\*\\]|${identifierPattern})\\s*-+>\\s*(\\[\\*\\]|${identifierPattern})`,
      ),
    );
    if (transitionMatch) {
      identifiers.add(transitionMatch[1]);
      identifiers.add(transitionMatch[2]);
    }
  }

  return [
    ...toIdentifierCompletions(identifiers),
    ...toCompletions(stateKeywords, "keyword"),
    ...toCompletions(stateSpecials, "keyword"),
  ];
}

function parseErCompletions(source: string) {
  const identifiers = new Set<string>();

  for (const line of source.split("\n")) {
    const cleanLine = maskQuotedText(stripComment(line));
    const trimmed = cleanLine.trim();
    if (!trimmed) continue;

    const entityBlockMatch = trimmed.match(
      new RegExp(`^(${identifierPattern})\\s*\\{`),
    );
    if (entityBlockMatch) identifiers.add(entityBlockMatch[1]);

    const relationshipMatch = trimmed.match(
      new RegExp(
        `^(${identifierPattern})\\s+(?:[|}o{]{1,2}--[|}o{]{1,2}|[|}o{]{1,2}\\.\\.[|}o{]{1,2})\\s+(${identifierPattern})\\b`,
      ),
    );
    if (relationshipMatch) {
      identifiers.add(relationshipMatch[1]);
      identifiers.add(relationshipMatch[2]);
    }
  }

  return [
    ...toIdentifierCompletions(identifiers),
    ...toCompletions(erKeywords, "keyword"),
  ];
}

function stripComment(line: string) {
  const commentStart = line.indexOf("%%");
  return commentStart === -1 ? line : line.slice(0, commentStart);
}

function maskFlowchartLine(line: string) {
  return maskBracketText(maskFlowEdgeLabels(maskQuotedText(line)));
}

function maskQuotedText(line: string) {
  return line.replace(/"[^"]*"|'[^']*'/g, (match) => " ".repeat(match.length));
}

function maskBracketText(line: string) {
  return line.replace(/(\[[^\]]*\]|\([^)]*\)|\{[^}]*\})/g, (match) =>
    " ".repeat(match.length),
  );
}

function maskFlowEdgeLabels(line: string) {
  return line.replace(
    /([-.=]{2,}>?)([^-.=\n|]+)([-.=]{2,}>?)/g,
    (_match, start: string, label: string, end: string) =>
      `${start}${" ".repeat(label.length)}${end}`,
  );
}

function containsFlowchartArrow(line: string) {
  return /(?:--+|==+|\.\.+)[ox>]?\s*/.test(line);
}

function addIdentifiersFromText(target: Set<string>, text: string) {
  for (const match of text.matchAll(identifierRegex)) {
    target.add(match[0]);
  }
}

function toIdentifierCompletions(values: Set<string>): MermaidCompletionItem[] {
  return [...values]
    .filter((label) => !allReservedWords.has(label))
    .map((label) => ({ label, kind: "identifier" }));
}

function toCompletions(
  values: Set<string>,
  kind: MermaidCompletionItem["kind"],
): MermaidCompletionItem[] {
  return [...values].map((label) => ({ label, kind }));
}

function dedupeCompletions(items: MermaidCompletionItem[]) {
  const seen = new Set<string>();
  const result: MermaidCompletionItem[] = [];

  for (const item of items) {
    const key = `${item.kind}:${item.label}`;
    if (seen.has(key)) continue;

    seen.add(key);
    result.push(item);
  }

  return result;
}

function compareCompletionItems(
  left: MermaidCompletionItem,
  right: MermaidCompletionItem,
) {
  const kindDifference = kindRank(left.kind) - kindRank(right.kind);
  if (kindDifference !== 0) return kindDifference;

  return left.label.localeCompare(right.label);
}

function kindRank(kind: MermaidCompletionItem["kind"]) {
  if (kind === "identifier") return 0;
  if (kind === "direction") return 1;
  return 2;
}

function getIgnoredRanges(source: string) {
  const ranges: Array<[number, number]> = [];
  let lineStart = 0;

  for (const line of source.split("\n")) {
    const commentStart = line.indexOf("%%");
    if (commentStart !== -1) {
      ranges.push([lineStart + commentStart, lineStart + line.length]);
    }

    for (const match of line.matchAll(/"[^"]*"|'[^']*'|\|[^|]*\|/g)) {
      ranges.push([
        lineStart + (match.index ?? 0),
        lineStart + (match.index ?? 0) + match[0].length,
      ]);
    }

    lineStart += line.length + 1;
  }

  return ranges;
}

function isOffsetIgnored(ranges: Array<[number, number]>, offset: number) {
  return ranges.some(([from, to]) => offset >= from && offset < to);
}

function isInsideToken(source: string, offset: number) {
  const nextCharacter = source[offset];
  return Boolean(nextCharacter && /[A-Za-z0-9_*]/.test(nextCharacter));
}

function clamp(value: number, minimum: number, maximum: number) {
  return Math.min(Math.max(value, minimum), maximum);
}
